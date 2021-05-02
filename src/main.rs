mod algo;
mod stall;

use algo::{GivenStalls, Phenotype};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::path::Path;

fn load_stalls_from_file(path: &Path) -> GivenStalls {
    let data = fs::read(path).unwrap();
    serde_json::from_str(&String::from_utf8_lossy(&data)).unwrap()
}

fn init(given_stalls: GivenStalls) {
    let mut rng = thread_rng();
    let mut genotype: Vec<u8> = given_stalls.iter().map(|s| s.id()).collect();
    genotype.shuffle(&mut rng);
    let pheno = Phenotype::new(genotype);
    println!("{}", pheno.fitness(given_stalls))
}

fn main() {
    let path = Path::new("stalls.json");
    let given_stalls = load_stalls_from_file(path);
    println!("from file = {:?}", given_stalls);
    init(given_stalls);
}
