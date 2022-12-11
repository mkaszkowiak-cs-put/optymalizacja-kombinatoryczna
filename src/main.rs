/*
I will consider one-dimensional offline bin packing problem
One-dimensional, as in objects and containers only have a single dimension
Offline, as in all object dimensions are known before-hand 
*/
use rand::{thread_rng, Rng};
use std::fs;
use rand::seq::SliceRandom;

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
struct Settings {
    item_size_min: u32,
    item_size_max: u32,
    item_limit: u32,
    container_size: u32
}

struct GeneratorResults {
    items: Vec<Item>,
    optimal_container_count: u32
}

#[derive(Clone)]
struct Generator {
    settings: Settings
}

impl Generator {
    fn generate(&self) -> GeneratorResults {
        let mut current_size = 0;
        let mut containers = 0;

        let mut items: Vec<Item> = Vec::new();
        for n in 0..self.settings.item_limit {
            if current_size == 0 {
                containers += 1;
            }

            let mut size: u32 = rand::thread_rng().gen_range(
                self.settings.item_size_min..self.settings.item_size_max
            );

            let item_overflows_current_container: bool = (current_size + size) > self.settings.container_size;
            let last_item_to_generate: bool = n == self.settings.item_limit - 1;

            if item_overflows_current_container || last_item_to_generate {
                let biggest_item_possible_to_fit: u32 = self.settings.container_size - current_size;
                size = biggest_item_possible_to_fit;
            }

            items.push(Item {
                size: size 
            });

            current_size = (current_size + size) % self.settings.container_size;
        }

        items.shuffle(&mut thread_rng());

        return GeneratorResults {
            items: items,
            optimal_container_count: containers
        };
    }
}

trait Solver {
    fn solve(&self, input: Vec<Item>) -> Vec<Container>;
    fn get_settings(&self) -> &Settings;
    fn get_name(&self) -> String;
    fn new_container(&self) -> Container {
        return Container::new(self.get_settings().container_size);
    }
}

struct SolverNextFit {
    settings: Settings
}

impl Solver for SolverNextFit {
    /* Next Fit (NF) always keeps a single open bin. 
    When the new item does not fit into it, it closes the current bin and opens a new bin */
    fn get_settings(&self) -> &Settings {
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
    settings: Settings
}

impl Solver for SolverFirstFit {
    /* First-Fit (FF) keeps all bins open, in the order in which they were opened. 
    It attempts to place each new item into the first bin in which it fits. */
    fn get_settings(&self) -> &Settings {
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
    settings: Vec<Settings>
}

fn generate_solver(string: String, settings: Settings) -> Option<Box<dyn Solver>> {
    match string.as_str() {
        "Next Fit" => Some(Box::new(SolverNextFit {settings: settings})),
        "First Fit" => Some(Box::new(SolverFirstFit {settings: settings})),
        _ => None
    }
}



fn main() {
    let data: String = fs::read_to_string("/home/maciej/bin-packing/input.json").unwrap();
    println!("{}", data);

    let program_input: ProgramInput = serde_json::from_str(&data[..]).unwrap();
    
    for settings in program_input.settings {
        let generator: Generator = Generator { 
            settings: settings.clone()
        };
        let mut solvers: Vec<Box<dyn Solver>> = Vec::new();
    
        let generator_results: GeneratorResults = generator.generate();
        let items: Vec<Item> = generator_results.items;
    
        println!("Optimal solution: {:} containers", generator_results.optimal_container_count);

        let solvers_list = program_input.solvers.clone();
        
        for solver_list_item in solvers_list {
            let solver = generate_solver(solver_list_item.id, settings.clone()).unwrap();
            let sorted = solver_list_item.sorted;
            let mut items_copy: Vec<Item> = items.clone();
            if sorted {
                items_copy.sort_unstable_by_key(|x| x.size);
                items_copy.reverse();
            }
            let result: Vec<Container> = solver.solve(items_copy);
    
            println!("Results for {:} {:} - {:} containers", if sorted {"Desc-sorted"} else {"Unsorted"}, solver.get_name(), result.len());
        }
    }
    
}
