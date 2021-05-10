mod algo;
mod phenotype;
mod population;
mod stall;

use algo::{Optimizer, Step};
use population::Population;
use stall::GivenStalls;
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

fn load_stalls_from_file(path: &Path) -> GivenStalls {
    let data = fs::read(path).unwrap();
    serde_json::from_str(&String::from_utf8_lossy(&data)).unwrap()
}

fn main() {
    let given_stalls = load_stalls_from_file(Path::new("stalls.json"));
    let population = Population::init(&given_stalls, 10);

    let mut optimizer = Optimizer::new(population)
        .with_crossover_rate(0.8)
        .with_mutation_rate(0.3)
        .with_max_step(1000);

    println!("{:?}", optimizer);
    let start_time = Instant::now();

    loop {
        let step = optimizer.step();

        match step {
            Step::Intermediate {
                best: _best,
                best_2: _best_2,
                population: _population,
                best_as_stalls: _best_as_stalls,
            } => {}

            Step::Final {
                best: _best,
                best_as_stalls: _best_as_stalls,
            } => {
                let duration = start_time.elapsed();

                println!(
                    "Reached final step. Best stall configuration with {} fitness score:\n",
                    _best.fitness(&given_stalls)
                );

                for s in _best_as_stalls {
                    println!("{} : c{}", s.name, s.category);
                }

                println!("\nSolution found in {} steps", optimizer.num_step_taken());
                println!("Time elapsed: {:?}", duration);
                break;
            }
        }
    }
}
