use crate::population::Population;
use crate::stall::Stall;
use rand::{thread_rng, Rng};

struct Optimizer {
    the_population: Population,
    crossover_rate: f32,
    mutation_rate: f32,
    prev_step: Step,
}

impl Optimizer {
    pub fn step(&self) -> Step {
        let given_stalls = &self.the_population.given_stalls;
        let population = &self.the_population.population;

        // Selection
        let mut pop_fitness = population
            .iter()
            .map(|pheno| pheno.fitness(given_stalls))
            .enumerate()
            .collect::<Vec<_>>();

        pop_fitness.sort_by(|a, b| a.1.cmp(&b.1)); // Lower = better.
        let p1_idx = pop_fitness[0].0; // Best chromosome
        let p2_idx = pop_fitness[1].0; // Second best chromosome
        let p1 = (p1_idx, population[p1_idx].clone());
        let p2 = (p2_idx, population[p2_idx].clone());

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

        // Cross over
        let mut child = vec![0; given_stalls.len()];

        // Insert into child 1 the genotype within random range l_idx..r_idx
        // from best chromosome.
        for i in l_idx..=r_idx {
            let g = &p1.1.genotype;
            child[i] = g[i];
        }

        // Fill other genes with second best chromosome, preserving the order.
        for i in 0..given_stalls.len() {
            if i >= l_idx && i <= r_idx {
                continue;
            }

            let g = &p2.1.genotype;
            if child.iter().any(|&c| c == g[i]) {
                continue;
            }

            child[i] = g[i];
        }

        // Mutation
        for i in 0..given_stalls.len() - 1 {
            let chance = rand::random::<f32>();

            if chance > self.mutation_rate {
                let (a, b) = (child[i], child[i + 1]);
                child[i] = b;
                child[i + 1] = a;
            }
        }

        todo!();
        //Step {
        //    population: self.the_population.clone(),
        //    step_type: StepType::Intermediate,
        //}
    }
}

struct Step {
    population: Population,
    step_type: StepType,
}

enum StepType {
    Intermediate,
    Final,
}
