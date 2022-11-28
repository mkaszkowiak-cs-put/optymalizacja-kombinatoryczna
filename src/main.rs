/*
I will consider one-dimensional offline bin packing problem
One-dimensional, as in objects and containers only have a single dimension
Offline, as in all object dimensions are known before-hand 
*/
use rand::Rng;
use std::fs;
use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Clone)]
struct Item {
    size: u32
}

#[derive(Clone)]
struct Container {
    size: u32,
    total: u32,
    items: Vec<Item>
}

impl Container {
    fn add(&mut self, cont: Item) -> Option<Item> {
        if self.total + cont.size > self.size {
            return Some(cont);
        }
        
        self.total += cont.size;
        self.items.push(cont);
        
        return None;
    }

    fn new(size: u32) -> Self {
        Container {
            size: size,
            total: 0,
            items: Vec::new()
        }
    } 
}

#[derive(Clone, Serialize, Deserialize)]
struct GeneratorSettings {
    item_size_min: u32,
    item_size_max: u32,
    item_limit: u32
}

#[derive(Clone)]
struct Generator {
    settings: GeneratorSettings
}

impl Generator {
    fn generate(&self) -> Vec<Item> {
        let mut items: Vec<Item> = Vec::new();
        for _ in 0..self.settings.item_limit {
            items.push(self.generate_item());
        }

        return items;
    }
    
    fn generate_item(&self) -> Item {
        Item {
            size: rand::thread_rng().gen_range(
                self.settings.item_size_min..self.settings.item_size_max
            )
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SolverSettings {
    container_size: u32
}


trait Solver {
    fn solve(&self, input: Vec<Item>) -> Vec<Container>;
    fn get_settings(&self) -> &SolverSettings;
    fn get_name(&self) -> String;
    fn new_container(&self) -> Container {
        return Container::new(self.get_settings().container_size);
    }
}

struct SolverNextFit {
    settings: SolverSettings
}

impl Solver for SolverNextFit {
    /* Next Fit (NF) always keeps a single open bin. 
    When the new item does not fit into it, it closes the current bin and opens a new bin */
    fn get_settings(&self) -> &SolverSettings {
        return &self.settings;
    }

    fn get_name(&self) -> String{
        return "Next Fit".to_string();
    }

    fn solve(&self, input: Vec<Item>) -> Vec<Container> {
        let mut results: Vec<Container> = Vec::new();
        let mut last_index: usize = 0;
        let first_container: Container = self.new_container();
        results.push(first_container);

        for item in input {
            // Add the item to the last container from the list
            let rejected_item: Option<Item> = results.get_mut(last_index).unwrap().add(item);
            if rejected_item.is_none() {
                continue;  
            }
            
            // If the container won't fit the item, create a new one
            let mut new_container: Container = self.new_container();
            let rejected_item_from_new_container: Option<Item> = new_container.add(rejected_item.unwrap());

            // If a new container can't fit the item, panic. 
            // Won't be able to provide a solution
            if !rejected_item_from_new_container.is_none() {
                panic!("An item won't fit into an empty container!");
            }
            
            results.push(new_container);
            last_index += 1;
        }

        return results;
    }
}

struct SolverFirstFit {
    settings: SolverSettings
}

impl Solver for SolverFirstFit {
    /* First-Fit (FF) keeps all bins open, in the order in which they were opened. 
    It attempts to place each new item into the first bin in which it fits. */
    fn get_settings(&self) -> &SolverSettings {
        return &self.settings;
    }

    fn get_name(&self) -> String{
        return "First Fit".to_string();
    }

    fn solve(&self, input: Vec<Item>) -> Vec<Container> {
        let mut results: Vec<Container> = Vec::new();
        let mut containers_count: usize = 1;
        let first_container: Container = self.new_container();
        results.push(first_container);

        for item in input {
            // Add the item to the last container from the list
            let mut rejected_item: Option<Item> = Some(item);
            for index in 0..containers_count {
                rejected_item = results.get_mut(index).unwrap().add(rejected_item.unwrap());
                if rejected_item.is_none() {
                    break;
                }
            }
            
            if rejected_item.is_none() {
                continue;
            }

            // If the container won't fit the item, create a new one
            let mut new_container: Container = self.new_container();
            let rejected_item_from_new_container: Option<Item> = new_container.add(rejected_item.unwrap());

            // If a new container can't fit the item, panic. 
            // Won't be able to provide a solution
            if !rejected_item_from_new_container.is_none() {
                panic!("An item won't fit into an empty container!");
            }
            
            results.push(new_container);
            containers_count += 1;
        }

        return results;
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct SolverListItem {
    id: String,
    sorted: bool
}

#[derive(Clone, Serialize, Deserialize)]
struct ProgramInput {
    solvers: Vec<SolverListItem>,
    generator_settings: GeneratorSettings,
    solver_settings: SolverSettings
}

fn generate_solver(string: String, solver_settings: SolverSettings) -> Option<Box<dyn Solver>> {
    match string.as_str() {
        "Next Fit" => Some(Box::new(SolverNextFit {settings: solver_settings})),
        "First Fit" => Some(Box::new(SolverFirstFit {settings: solver_settings})),
        _ => None
    }
}



fn main() {
    let data: String = fs::read_to_string("/home/maciej/bin-packing/input.json").unwrap();
    println!("{}", data);

    let program_input: ProgramInput = serde_json::from_str(&data[..]).unwrap();
    
    let generator_settings: GeneratorSettings = program_input.generator_settings;
    let solver_settings: SolverSettings = program_input.solver_settings;

    let generator: Generator = Generator { settings: generator_settings.clone() };
    let mut solvers: Vec<Box<dyn Solver>> = Vec::new();

    let items: Vec<Item> = generator.generate();
    
    for solver_list_item in program_input.solvers {
        let solver = generate_solver(solver_list_item.id, solver_settings.clone()).unwrap();
        let sorted = solver_list_item.sorted;
        let mut items_copy: Vec<Item> = items.clone();
        if sorted {
            items_copy.sort_unstable_by_key(|x| x.size);
            items_copy.reverse();
        }
        let result: Vec<Container> = solver.solve(items_copy);

        println!("Results for {:} {:} - {:} containers", if sorted {"Desc-sorted"} else {"Unsorted"}, solver.get_name(), result.len());
    
        
        /*for container in result {
            println!("Container [{:} / {:}] ", container.total, container.size);
        }*/
    }
    
}
