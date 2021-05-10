mod algo;
mod phenotype;
mod population;
mod stall;

use algo::Optimizer;
use population::Population;
use stall::GivenStalls;
use std::fs;
use std::path::Path;

fn load_stalls_from_file(path: &Path) -> GivenStalls {
    let data = fs::read(path).unwrap();
    serde_json::from_str(&String::from_utf8_lossy(&data)).unwrap()
}

fn main() {
    let given_stalls = load_stalls_from_file(Path::new("stalls.json"));
    let population = Population::init(given_stalls, 10);

    let mut optimizer = Optimizer::new(population)
        .with_crossover_rate(0.8)
        .with_mutation_rate(0.3)
        .with_max_step(1000);

    println!("{:?}", optimizer);
    optimizer.step();
    println!("{:?}", optimizer);
}
