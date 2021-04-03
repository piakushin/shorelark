use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;

use crate::{Individual, RouletteWheelSelection, SelectionMethod};

#[derive(Debug, Clone)]
pub struct TestIndividual {
    fitness: f32,
}

impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
}

#[test]
fn test() {
    let method = RouletteWheelSelection::new();
    let mut rng = ChaCha8Rng::from_seed(Default::default());

    let population = vec![
        TestIndividual::new(2.0),
        TestIndividual::new(1.0),
        TestIndividual::new(4.0),
        TestIndividual::new(3.0),
    ];

    let actual_histogram: BTreeMap<i32, _> = (0..1000)
        .map(|_| method.select(&mut rng, &population))
        .fold(Default::default(), |mut histogram, individual| {
            *histogram.entry(individual.fitness() as _).or_default() += 1;
            histogram
        });

    let expected_histogram = maplit::btreemap! {
        1 => 98,
        2 => 202,
        3 => 278,
        4 => 422,
    };

    assert_eq!(actual_histogram, expected_histogram);
}
