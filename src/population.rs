use crate::phenotype::Phenotype;
use crate::stall::GivenStalls;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
pub struct Population {
    given_stalls: GivenStalls,
    population: Vec<Phenotype>,
    size: usize,
}

impl Population {
    pub fn init(given_stalls: &GivenStalls, size: usize) -> Population {
        let mut geno = given_stalls.iter().map(|s| s.id).collect::<Vec<_>>();
        let mut population: Vec<Phenotype> = Vec::with_capacity(size);

        for _ in 0..size {
            let mut rng = thread_rng();
            geno.shuffle(&mut rng);
            population.push(Phenotype::new(geno.clone()));
        }

        Population {
            given_stalls: given_stalls.to_owned(),
            population,
            size,
        }
    }

    pub fn avg_fitness(&self) -> usize {
        let total_fitness: usize = self
            .population
            .iter()
            .map(|p| p.fitness(&self.given_stalls))
            .sum();

        total_fitness / self.population.len()
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn given_stalls(&self) -> &GivenStalls {
        &self.given_stalls
    }

    pub fn get(&self) -> &Vec<Phenotype> {
        &self.population
    }

    pub fn set(&mut self, p: &Vec<Phenotype>) {
        self.population = p.clone();
    }
}
