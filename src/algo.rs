use crate::stall::Stall;

pub struct Genotype {
    phenotype: Vec<Stall>,
}

impl Genotype {
    pub fn new(phenotype: Vec<Stall>) -> Genotype {
        Genotype { phenotype }
    }

    pub fn fitness(&self) -> u8 {
        self.phenotype
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

pub struct Arrange {
    population: Vec<Genotype>,
}

impl Arrange {}

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
        let stalls = serde_json::from_str(&data).unwrap();
        let geno = Genotype::new(stalls);
        let fitness = geno.fitness();

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
        let stalls = serde_json::from_str(&data).unwrap();
        let geno = Genotype::new(stalls);
        let fitness = geno.fitness();

        assert_eq!(fitness, 2);
    }
}
