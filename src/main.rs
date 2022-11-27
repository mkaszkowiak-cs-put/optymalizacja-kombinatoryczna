/*
I will consider one-dimensional offline bin packing problem
One-dimensional, as in objects and containers only have a single dimension
Offline, as in all object dimensions are known before-hand 
*/
use rand::Rng;

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

#[derive(Clone)]
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

#[derive(Clone)]
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





fn main() {
    let generator_settings: GeneratorSettings = GeneratorSettings {
        item_size_min: 0, item_size_max: 100, item_limit: 5000
    };
    let solver_settings: SolverSettings = SolverSettings { 
        container_size: 400
    };

    let generator: Generator = Generator { settings: generator_settings };
    let mut solvers: Vec<Box<dyn Solver>> = Vec::new();
    let solver: SolverNextFit = SolverNextFit { settings: solver_settings.clone() };
    solvers.push(Box::new(solver));
    let solver: SolverFirstFit = SolverFirstFit { settings: solver_settings.clone() };
    solvers.push(Box::new(solver));


    let items: Vec<Item> = generator.generate();
    
    for solver in solvers {
        let items_copy: Vec<Item> = items.clone();
        let result: Vec<Container> = solver.solve(items_copy);

        println!("Results for {:} - {:} containers", solver.get_name(), result.len());
        for container in result {
            //println!("Container [{:} / {:}] ", container.total, container.size);
        }
    }
    
}
