mod algo;
mod phenotype;
mod population;
mod stall;

use population::Population;
use stall::GivenStalls;
use std::fs;
use std::path::Path;

fn load_stalls_from_file(path: &Path) -> GivenStalls {
    let data = fs::read(path).unwrap();
    serde_json::from_str(&String::from_utf8_lossy(&data)).unwrap()
}

fn main() {
    let path = Path::new("stalls.json");
    let given_stalls = load_stalls_from_file(path);
    println!("from file = {:?}", given_stalls);

    let pop = Population::init(given_stalls, 10);
    // let optimizer = Optimizer::init(pop);
    println!("Population {:?}", pop);
}
