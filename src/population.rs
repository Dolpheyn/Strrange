use crate::phenotype::Phenotype;
use crate::stall::GivenStalls;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Population {
    pub given_stalls: GivenStalls,
    pub population: Vec<Phenotype>,
    pub size: usize,
}

impl Population {
    pub fn init(given_stalls: GivenStalls, size: usize) -> Population {
        let mut geno = given_stalls.iter().map(|s| s.id).collect::<Vec<_>>();
        let mut population: Vec<Phenotype> = Vec::with_capacity(size);

        for _ in 0..size {
            let mut rng = thread_rng();
            geno.shuffle(&mut rng);
            population.push(Phenotype::new(geno.clone()));
        }

        Population {
            given_stalls,
            population,
            size,
        }
    }
}
