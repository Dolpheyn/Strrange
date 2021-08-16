use crate::stall::{GivenStalls, Stall};

type Genotype = Vec<u8>;

#[derive(Debug, Clone)]
pub struct Phenotype {
    pub genotype: Genotype,
}

impl Phenotype {
    pub fn new(genotype: Genotype) -> Phenotype {
        Phenotype { genotype }
    }

    pub fn fitness(&self, given_stalls: &GivenStalls) -> usize {
        self.genotype
            .iter()
            .map(|p| given_stalls.iter().find(|s| &s.id == p).unwrap())
            .collect::<Vec<_>>()
            .windows(2)
            .map(|pair| pair[0].category == pair[1].category)
            .filter(|&b| b)
            .count()
    }
}

pub trait AsStalls {
    fn as_stalls(&self, gs: &GivenStalls) -> Vec<Stall>;
}

impl AsStalls for Phenotype {
    fn as_stalls(&self, gs: &GivenStalls) -> Vec<Stall> {
        self.genotype
            .iter()
            .map(|&g| gs.iter().find(|s| s.id == g).unwrap().to_owned())
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
