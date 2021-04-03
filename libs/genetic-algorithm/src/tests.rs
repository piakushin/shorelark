use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::BTreeMap;

use crate::{Chromosome, Individual, RouletteWheelSelection, SelectionMethod};

fn default_rng() -> impl RngCore {
    ChaCha8Rng::from_seed(Default::default())
}

#[derive(Debug, Clone, PartialEq)]
pub enum TestIndividual {
    WithChromosome { chromosome: Chromosome },
    WithFitness { fitness: f32 },
}

impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self::WithFitness { fitness }
    }
}

impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        match self {
            Self::WithChromosome { chromosome } => chromosome.iter().sum(),
            Self::WithFitness { fitness } => *fitness,
        }
    }

    fn chromosome(&self) -> &Chromosome {
        match self {
            Self::WithChromosome { chromosome } => chromosome,
            Self::WithFitness { .. } => {
                panic!("not supported");
            }
        }
    }

    fn create(chromosome: Chromosome) -> Self {
        Self::WithChromosome { chromosome }
    }
}

mod population {
    use crate::{GaussianMutation, GeneticAlgorithm, Individual, UniformCrossover};

    use super::*;

    fn individual(genes: &[f32]) -> TestIndividual {
        let chromosome = genes.iter().copied().collect();
        TestIndividual::create(chromosome)
    }

    #[test]
    fn test() {
        let mut rng = default_rng();

        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]), // fitness = 0.0
            individual(&[1.0, 1.0, 1.0]), // fitness = 3.0
            individual(&[1.0, 2.0, 1.0]), // fitness = 4.0
            individual(&[1.0, 2.0, 4.0]), // fitness = 7.0
        ];

        for _ in 0..10 {
            population = ga.evolve(&mut rng, &population);
        }

        let expected_population = vec![
            individual(&[0.447_694_9, 2.064_835_8, 4.305_813_3]),
            individual(&[1.212_686_7, 1.553_877_7, 2.886_911]),
            individual(&[1.061_767_8, 2.265_739, 4.428_764]),
            individual(&[0.959_096_85, 2.461_878_8, 4.024_733]),
        ];

        assert_eq!(population, expected_population);
    }
}

#[test]
fn test() {
    let method = RouletteWheelSelection::new();
    let mut rng = default_rng();

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

fn chromosome() -> Chromosome {
    Chromosome {
        genes: vec![3.0, 1.0, 2.0],
    }
}

mod len {
    use super::chromosome;

    #[test]
    fn test() {
        assert_eq!(chromosome().len(), 3);
    }
}

mod iter {
    use super::chromosome;

    #[test]
    fn test() {
        let chromosome = chromosome();
        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_eq!(*genes[0] as u32, 3);
        assert_eq!(*genes[1] as u32, 1);
        assert_eq!(*genes[2] as u32, 2);
    }
}

mod iter_mut {
    use super::chromosome;

    #[test]
    fn test() {
        let mut chromosome = chromosome();

        chromosome.iter_mut().for_each(|gene| {
            *gene *= 10.0;
        });

        let genes: Vec<_> = chromosome.iter().collect();

        assert_eq!(genes.len(), 3);
        assert_eq!(*genes[0] as u32, 30);
        assert_eq!(*genes[1] as u32, 10);
        assert_eq!(*genes[2] as u32, 20);
    }
}

mod index {
    use super::Chromosome;

    #[test]
    fn test() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        assert_eq!(chromosome[0] as u32, 3);
        assert_eq!(chromosome[1] as u32, 1);
        assert_eq!(chromosome[2] as u32, 2);
    }
}

mod from_iterator {
    use super::Chromosome;

    #[test]
    fn test() {
        let chromosome: Chromosome = vec![3.0, 1.0, 2.0].into_iter().collect();

        assert_eq!(chromosome[0] as u32, 3);
        assert_eq!(chromosome[1] as u32, 1);
        assert_eq!(chromosome[2] as u32, 2);
    }
}

mod into_iterator {
    use super::Chromosome;

    #[test]
    fn test() {
        let chromosome = Chromosome {
            genes: vec![3.0, 1.0, 2.0],
        };

        let genes: Vec<_> = chromosome.into_iter().collect();

        assert_eq!(genes.len(), 3);
        assert_eq!(genes[0] as u32, 3);
        assert_eq!(genes[1] as u32, 1);
        assert_eq!(genes[2] as u32, 2);
    }
}

mod crossover {
    use super::*;
    use crate::{CrossoverMethod, UniformCrossover};

    #[test]
    fn test() {
        let mut rng = default_rng();
        let parent_a = (1..=100).map(|n| n as f32).collect();
        let parent_b = (1..=100).map(|n| -n as f32).collect();

        let child = UniformCrossover::new().crossover(&mut rng, &parent_a, &parent_b);

        let diff_a = child
            .iter()
            .zip(parent_a)
            .filter(|(c, p)| **c as i32 != *p as i32)
            .count();

        let diff_b = child
            .iter()
            .zip(parent_b)
            .filter(|(c, p)| **c as i32 != *p as i32)
            .count();
        assert_eq!(diff_a, 49);
        assert_eq!(diff_b, 51);
    }
}
mod mutation {
    use super::*;
    use crate::{GaussianMutation, MutationMethod};

    fn actual(chance: f32, coeff: f32) -> Vec<f32> {
        let mut child = vec![1.0, 2.0, 3.0, 4.0, 5.0].into_iter().collect();

        let mut rng = default_rng();

        GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);

        child.into_iter().collect()
    }

    mod given_zero_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.0, coeff)
        }

        mod and_zero_coefficient {
            use super::actual;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }

        mod and_nonzero_coefficient {
            use super::actual;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
            }
        }
    }

    mod given_fifty_fifty_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(0.5, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn slightly_changes_the_original_chromosome() {
                let actual = actual(0.5);
                let expected = vec![1.0, 1.7756249, 3.0, 4.1596804, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }
    }

    mod given_max_chance {
        fn actual(coeff: f32) -> Vec<f32> {
            super::actual(1.0, coeff)
        }

        mod and_zero_coefficient {
            use super::*;

            #[test]
            fn does_not_change_the_original_chromosome() {
                let actual = actual(0.0);
                let expected = vec![1.0, 2.0, 3.0, 4.0, 5.0];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }

        mod and_nonzero_coefficient {
            use super::*;

            #[test]
            fn entirely_changes_the_original_chromosome() {
                let actual = actual(0.5);

                let expected = vec![1.4545316, 2.1162078, 2.7756248, 3.9505124, 4.638691];

                approx::assert_relative_eq!(actual.as_slice(), expected.as_slice(),);
            }
        }
    }
}
