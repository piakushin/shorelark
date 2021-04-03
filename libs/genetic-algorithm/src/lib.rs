use std::collections::BTreeMap;

use rand::{prelude::SliceRandom, Rng, RngCore, SeedableRng};

pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn evolve<I>(&self, population: &[I]) -> Vec<I> {
        assert!(!population.is_empty());

        (0..population.len()).map(|_| todo!()).collect()
    }
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("got an empty population")
    }
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

pub trait Individual {
    fn fitness(&self) -> f32;
}

#[test]
fn test() {
    use rand_chacha::ChaCha8Rng;
    use std::iter::FromIterator;

    let method = RouletteWheelSelection::new();
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let population = vec![
        TestIndividual::new(2.0),
        TestIndividual::new(1.0),
        TestIndividual::new(4.0),
        TestIndividual::new(3.0),
    ];

    let mut actual_histogram = BTreeMap::new();

    for _ in 0..1000 {
        let fitness = method.select(&mut rng, &population).fitness as i32;
        *actual_histogram.entry(fitness).or_insert(0) += 1;
    }

    let expected_histogram = maplit::btreemap! {
        1 => 98,
        2 => 202,
        3 => 278,
        4 => 422,
    };

    assert_eq!(actual_histogram, expected_histogram);
}
