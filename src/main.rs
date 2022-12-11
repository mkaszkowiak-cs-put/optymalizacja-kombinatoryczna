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
use std::time::Instant;
use std::cmp;


#[derive(Clone, Serialize, Deserialize, Default)]
struct ProblemResult {
    solver_name: String,
    solver_sorted: bool,

    item_size_min: u32,
    item_size_max: u32,
    item_limit: u32,

    container_size: u32,

    iterations: u32,
    optimal_solutions_found: u32,

    quality_best_case: f32,
    quality_worst_case: f32,
    quality_avg_case: f32,

    time_us_best_case: u128,
    time_us_worst_case: u128,
    time_us_avg_case: f32
}


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
    settings: Vec<Settings>,
    iterations: u32
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
    let mut results: Vec<ProblemResult> = Vec::new();

    let settings_length = program_input.settings.len();
    let solvers_length = program_input.solvers.len();

    for _ in 0..settings_length {
        for _ in 0..solvers_length {
            results.push(ProblemResult::default());
        }
    }
    
    for (settings_n, settings) in program_input.settings.iter().enumerate() {
        let generator: Generator = Generator { 
            settings: settings.clone()
        };
        let mut solvers: Vec<Box<dyn Solver>> = Vec::new();
    
        for _ in 0..program_input.iterations {
            let generator_results: GeneratorResults = generator.generate();
            let items: Vec<Item> = generator_results.items;
        
            println!("Optimal solution: {:} containers", generator_results.optimal_container_count);

            let solvers_list = program_input.solvers.clone();
            
            for (solver_n, solver_list_item) in solvers_list.into_iter().enumerate() {
                let solver = generate_solver(solver_list_item.id, settings.clone()).unwrap();
                let sorted = solver_list_item.sorted;

                let result_i = settings_n * solvers_length + solver_n;
                if results[result_i].iterations == 0 {
                    results[result_i] = ProblemResult {
                        solver_name: solver.get_name(),
                        solver_sorted: sorted,
    
                        item_size_min: settings.item_size_min,
                        item_size_max: settings.item_size_max,
                        item_limit: settings.item_limit,
    
                        container_size: settings.container_size,
                        iterations: 0,
                        optimal_solutions_found: 0,
    
                        quality_best_case: f32::MAX,
                        quality_worst_case: 0.0,
                        quality_avg_case: 0.0,
    
                        time_us_best_case: u128::MAX,
                        time_us_worst_case: 0,
                        time_us_avg_case: 0.0
                    };
                }

                let mut items_copy: Vec<Item> = items.clone();

                let now = Instant::now();
                // Sorting the array is timed as a part of the solver
                if sorted {
                    items_copy.sort_unstable_by_key(|x| x.size);
                    items_copy.reverse();
                }
                let result: Vec<Container> = solver.solve(items_copy);
                let elapsed_us = now.elapsed().as_micros();

                results[result_i].iterations += 1;

                let quality: f32 = result.len() as f32 / generator_results.optimal_container_count as f32; 

                results[result_i].optimal_solutions_found += if quality == 1.0 {1} else {0};
                results[result_i].quality_avg_case += (quality - results[result_i].quality_avg_case) / results[result_i].iterations as f32;
                results[result_i].quality_worst_case = f32::max(results[result_i].quality_worst_case, quality);
                results[result_i].quality_best_case = f32::min(results[result_i].quality_best_case, quality);

                results[result_i].time_us_avg_case += (elapsed_us as f32 - results[result_i].time_us_avg_case) / results[result_i].iterations as f32;
                results[result_i].time_us_worst_case = cmp::max(results[result_i].time_us_worst_case, elapsed_us);
                results[result_i].time_us_best_case = cmp::min(results[result_i].time_us_best_case, elapsed_us);


                println!("Results for {:} {:} - {:} containers, {:} us", if sorted {"Desc-sorted"} else {"Unsorted"}, solver.get_name(), result.len(), elapsed_us);
            }


        }
        
    }

    //let final_results = results.into_iter().flatten().collect();
    let slice_string_in_json_format = serde_json::to_string(&results);
    println!("{:}", slice_string_in_json_format.unwrap());

    
}
