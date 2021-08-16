mod algo;
mod error;
mod phenotype;
mod population;
mod stall;

use crate::{
    algo::{Optimizer, Step},
    error::StrrangeError,
    population::Population,
    stall::GivenStalls,
};
use std::{fs, path::Path, process::exit, time::Instant};

fn load_stalls_from_file(path: &Path) -> Result<GivenStalls, StrrangeError> {
    let data = match fs::read(path) {
        Ok(v) => v,
        Err(e) => return Err(StrrangeError::Str(e.to_string())),
    };

    match serde_json::from_str(&String::from_utf8_lossy(&data)) {
        Ok(s) => Ok(s),
        Err(e) => Err(StrrangeError::Str(e.to_string())),
    }
}

fn run() -> Result<(), StrrangeError> {
    let given_stalls = load_stalls_from_file(Path::new("stalls.json"))?;
    let population = Population::init(&given_stalls, 10);

    let mutation_rate = 0.25;
    let crossover_rate = 0.7;
    let max_step = 1000;

    let mut optimizer = Optimizer::new(population)
        .with_mutation_rate(mutation_rate)
        .with_crossover_rate(crossover_rate)
        .with_max_step(max_step);

    println!(
        "Starting optimization with {} crossover rate, {} mutation rate, and {} max step...\n",
        crossover_rate, mutation_rate, max_step
    );

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
                best,
                best_as_stalls,
                avg_fitness,
            } => {
                let duration = start_time.elapsed();

                println!(
                    "Reached final step.\nBest fitness score: {}\nAvg fitness score: {}\n",
                    best.fitness(&given_stalls),
                    avg_fitness,
                );

                for s in best_as_stalls {
                    println!("{} : c{}", s.name, s.category);
                }

                println!("\nSolution found in {} steps", optimizer.num_step_taken());
                println!("Time elapsed: {:?}", duration);
                break;
            }
        }
    }

    Ok(())
}

fn main() {
    let result = run();
    if let Err(e) = result {
        println!("{}", e);
        exit(1);
    }
}
