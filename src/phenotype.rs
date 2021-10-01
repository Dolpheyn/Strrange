use crate::stall::{GivenStalls, Stall};

type Genotype = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Phenotype {
    // Each genotype is a permutation list of stalls' id.
    pub genotype: Genotype,
}

impl Phenotype {
    pub fn new(genotype: Genotype) -> Phenotype {
        Phenotype { genotype }
    }

    // Calculate how many adjacent stalls with the same category(i.e. stalls that are next to
    // eachother with the same category). Lower = better.
    pub fn fitness(&self, given_stalls: &GivenStalls) -> usize {
        self.as_stalls(given_stalls)
            // Take 2 stalls at a time
            .windows(2)
            // If the 2 stalls has the same category, map to true, else false
            .map(|pair| pair[0].category == pair[1].category)
            // Get only the stalls with same category
            .filter(|&same| same)
            // Get the count
            .count()
    }
}

pub trait AsStalls {
    fn as_stalls(&self, given_stalls: &GivenStalls) -> Vec<Stall>;
}

impl AsStalls for Phenotype {
    fn as_stalls(&self, given_stalls: &GivenStalls) -> Vec<Stall> {
        self.genotype
            .iter()
            .map(|g| given_stalls.iter().find(|s| s.id == *g).unwrap().to_owned())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stall::GetId;

    #[test]
    fn test_fitness() {
        let data = r#"
            [
                {"id":0,"name":"Air Katira","category":0},
                {"id":1,"name":"Murtabak Singapore","category":1},
                {"id":2,"name":"Milo Es","category":0}
            ]
        "#;
        let given_stalls: GivenStalls = serde_json::from_str(&data).unwrap();
        let ids = given_stalls.get_ids();
        let pheno = Phenotype::new(ids);
        let fitness = pheno.fitness(&given_stalls);

        assert_eq!(fitness, 0);
    }

    #[test]
    fn test_fitness_1() {
        let data = r#"
            [
                {"id":0,"name":"Air Katira","category":0},
                {"id":1,"name":"Teh Ais Pyorr","category":0},
                {"id":2,"name":"Milo Es","category":0}
            ]
        "#;
        let given_stalls: GivenStalls = serde_json::from_str(&data).unwrap();
        let ids = given_stalls.get_ids();
        let pheno = Phenotype::new(ids);
        let fitness = pheno.fitness(&given_stalls);

        assert_eq!(fitness, 2);
    }
}
