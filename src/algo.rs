use crate::phenotype::{AsStalls, Phenotype};
use crate::population::Population;
use crate::stall::Stall;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct Optimizer {
    the_population: Population,
    crossover_rate: f32,
    mutation_rate: f32,
    cur_step: u32,
    max_step: u32,
}

#[derive(Debug)]
pub enum Step {
    Intermediate {
        best: Phenotype,
        best_2: Phenotype,
        best_as_stalls: Vec<Stall>,
        population: Vec<Phenotype>,
    },
    Final {
        best: Phenotype,
        best_as_stalls: Vec<Stall>,
        avg_fitness: u8,
    },
}

impl Optimizer {
    pub fn new(population: Population) -> Optimizer {
        Optimizer {
            the_population: population,
            crossover_rate: 0.0,
            mutation_rate: 0.0,
            cur_step: 0,
            max_step: 0,
        }
    }

    pub fn num_step_taken(&self) -> u32 {
        self.cur_step
    }

    pub fn with_crossover_rate(mut self, crossover_rate: f32) -> Self {
        self.crossover_rate = crossover_rate;
        self
    }

    pub fn with_mutation_rate(mut self, mutation_rate: f32) -> Self {
        self.mutation_rate = mutation_rate;
        self
    }

    pub fn with_max_step(mut self, max_step: u32) -> Self {
        self.max_step = max_step;
        self
    }

    fn reached_max_step(&self) -> bool {
        self.cur_step == self.max_step
    }

    fn crossover(&self, best: &Phenotype, second_best: &Phenotype) -> Vec<u8> {
        let given_stalls = &self.the_population.given_stalls;

        let mut rng = thread_rng();
        let (l_idx, r_idx) = {
            let rand_pair = (
                rng.gen_range(0..given_stalls.len()),
                rng.gen_range(0..given_stalls.len()),
            );

            if rand_pair.0 > rand_pair.1 {
                (rand_pair.1, rand_pair.0)
            } else {
                (rand_pair.0, rand_pair.1)
            }
        };

        let mut child = vec![0; given_stalls.len()];

        // Insert into child the genotype within random range l_idx..r_idx
        // from best chromosome.
        for i in l_idx..=r_idx {
            let g = &best.genotype;
            child[i] = g[i];
        }

        // Fill the leftover empty genes with second best chromosome, preserving the order of the
        // second best chromosome's gene.
        for i in 0..given_stalls.len() {
            if i >= l_idx && i <= r_idx {
                continue;
            }

            let g = &second_best.genotype;
            if child.iter().any(|&c| c == g[i]) {
                continue;
            }

            child[i] = g[i];
        }

        child
    }

    fn mutate(&self, mut child: Vec<u8>) -> Vec<u8> {
        let given_stalls = &self.the_population.given_stalls;
        let chance = rand::random::<f32>;

        for i in 0..given_stalls.len() - 1 {
            if chance() > self.mutation_rate {
                let (a, b) = (child[i], child[i + 1]);
                child[i] = b;
                child[i + 1] = a;
            }
        }

        child
    }

    pub fn step(&mut self) -> Step {
        let given_stalls = &self.the_population.given_stalls;
        let mut population = self.the_population.population.clone();

        // Selection
        population.sort_by(|a, b| a.fitness(given_stalls).cmp(&b.fitness(given_stalls))); // Sort by fitness, lower = better.
        let best = population[0].clone();
        let best_2 = population[1].clone();

        // Termination Case
        if best.fitness(given_stalls) == 0 || self.reached_max_step() {
            return Step::Final {
                avg_fitness: self.the_population.avg_fitness(),
                best: best.clone(),
                best_as_stalls: best.as_stalls(given_stalls),
            };
        }

        let mut new_population = Vec::new();

        // Preserve 2 best phenotype
        new_population.push(best.clone());
        new_population.push(best_2.clone());

        // Crossover and Mutation
        for p in population.iter().skip(2) {
            let mut child_geno = p.genotype.clone();
            let chance = rand::random::<f32>;

            if chance() > self.crossover_rate {
                child_geno = self.crossover(&best, &best_2);
            }

            if chance() > self.mutation_rate {
                child_geno = self.mutate(child_geno);
            }

            new_population.push(Phenotype::new(child_geno));
        }
        assert_eq!(new_population.len(), self.the_population.size);

        self.the_population.population = new_population.clone();
        self.cur_step += 1;

        Step::Intermediate {
            best_as_stalls: best.as_stalls(given_stalls),
            best,
            best_2,
            population: new_population,
        }
    }
}
