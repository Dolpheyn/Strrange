mod algo;
mod stall;

use algo::Genotype;
use stall::Stall;
use std::fs;
use std::path::Path;

fn load_stalls_from_file(path: &Path) -> Vec<Stall> {
    let data = fs::read(path).unwrap();
    serde_json::from_str(&String::from_utf8_lossy(&data)).unwrap()
}

fn main() {
    let path = Path::new("stalls.json");
    let from_file = load_stalls_from_file(path);
    println!("from file = {:?}", from_file);

    let geno = Genotype::new(from_file);
    println!("{}", geno.fitness())
}
