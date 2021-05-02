use crate::stall::Stall;

type Genotype = Vec<u8>;
type GivenStalls = Vec<Stall>;

trait GetId {
    fn get_ids(&self) -> Vec<u8>;
}

impl GetId for GivenStalls {
    fn get_ids(&self) -> Vec<u8> {
        self.iter().map(|s| s.id()).collect()
    }
}

pub struct Phenotype {
    given_stalls: GivenStalls,
    genotype: Genotype,
}

impl Phenotype {
    pub fn new(given_stalls: Vec<Stall>, genotype: Genotype) -> Phenotype {
        Phenotype {
            given_stalls,
            genotype,
        }
    }

    pub fn fitness(&self) -> u8 {
        self.genotype
            .iter()
            .map(|p| self.given_stalls.iter().find(|s| &s.id() == p).unwrap())
            .collect::<Vec<_>>()
            .windows(2)
            .map(|pair| pair[0].category() == pair[1].category())
            .fold(0, |mut sum, b| {
                if b == true {
                    sum += 1;
                }
                sum
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let pheno = Phenotype::new(given_stalls, ids);
        let fitness = pheno.fitness();

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
        let pheno = Phenotype::new(given_stalls, ids);
        let fitness = pheno.fitness();

        assert_eq!(fitness, 2);
    }
}
