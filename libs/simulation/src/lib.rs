#![feature(crate_visibility_modifier)]

pub use self::{animal::*, brain::*, config::*, eye::*, food::*, role::*, statistics::*, world::*};

mod animal;
mod animal_individual;
mod brain;
mod config;
mod eye;
mod food;
mod role;
mod statistics;
mod world;

use self::animal_individual::*;
use lib_genetic_algorithm as ga;
use lib_neural_network as nn;
use nalgebra as na;
use rand::{Rng, RngCore};
use serde::{Deserialize, Serialize};
use std::f32::consts::*;

pub struct Simulation {
    config: Config,
    world: World,
    age: usize,
    generation: usize,
}

impl Simulation {
    pub fn random(config: Config, rng: &mut dyn RngCore) -> Self {
        let world = World::random(&config, rng);

        Self {
            config,
            world,
            age: 0,
            generation: 0,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
        self.try_evolving(rng)
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> Statistics {
        loop {
            if let Some(statistics) = self.step(rng) {
                return statistics;
            }
        }
    }
}

impl Simulation {
    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for bird in &mut self.world.birds {
            for food in &mut self.world.foods {
                let distance = na::distance(&bird.position, &food.position);

                if distance <= self.config.food_size {
                    bird.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
        // for eagle in &mut self.world.eagles {
        //     for bird in &mut self.world.birds {
        //         let distance = na::distance(&eagle.position, &bird.position);

        //         if distance <= self.config.food_size {
        //             eagle.satiation += bird.satiation;
        //             bird.position = rng.gen();
        //         }
        //     }
        // }
    }

    fn process_brains(&mut self) {
        for bird in &mut self.world.birds {
            bird.process_brain(&self.config, &self.world.foods);
        }
    }

    fn process_movements(&mut self) {
        for bird in &mut self.world.birds {
            bird.process_movement();
        }
        for eagle in &mut self.world.eagles {
            eagle.process_movement();
        }
    }

    fn try_evolving(&mut self, rng: &mut dyn RngCore) -> Option<Statistics> {
        self.age += 1;

        if self.age > self.config.sim_generation_length {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> Statistics {
        self.age = 0;
        self.generation += 1;

        let mut individuals_birds: Vec<_> = self
            .world
            .birds
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        let mut individuals_eagles: Vec<_> = self
            .world
            .eagles
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        if self.config.ga_reverse == 1 {
            let max_satiation_birds = self
                .world
                .birds
                .iter()
                .map(|bird| bird.satiation)
                .max()
                .unwrap_or_default();

            for individual in &mut individuals_birds {
                individual.fitness = (max_satiation_birds as f32) - individual.fitness;
            }

            let max_satiation_eagles = self
                .world
                .eagles
                .iter()
                .map(|eagle| eagle.satiation)
                .max()
                .unwrap_or_default();

            for individual in &mut individuals_eagles {
                individual.fitness = (max_satiation_eagles as f32) - individual.fitness;
            }
        }

        let ga = ga::GeneticAlgorithm::new(
            ga::RouletteWheelSelection::default(),
            ga::UniformCrossover::default(),
            ga::GaussianMutation::new(self.config.ga_mut_chance, self.config.ga_mut_coeff),
        );

        let (individuals_birds, statistics_birds) = ga.evolve(rng, &individuals_birds);
        let (individuals_eagles, statistics_eagles) = ga.evolve(rng, &individuals_eagles);

        self.world.birds = individuals_birds
            .into_iter()
            .map(|i| i.into_animal(&self.config, rng, Role::Prey))
            .collect();

        self.world.eagles = individuals_eagles
            .into_iter()
            .map(|i| i.into_animal(&self.config, rng, Role::Predator))
            .collect();

        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        Statistics {
            generation: self.generation - 1,
            ga_birds: statistics_birds,
            ga_eagles: statistics_eagles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    #[ignore]
    fn test() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let mut sim = Simulation::random(Default::default(), &mut rng);

        let avg_fitness = (0..10)
            .map(|_| sim.train(&mut rng).ga_birds.avg_fitness())
            .sum::<f32>()
            / 10.0;

        approx::assert_relative_eq!(31.944998, avg_fitness);
    }
}
