use crate::*;

#[derive(Debug)]
pub struct World {
    crate eagles: Vec<Animal>,
    crate birds: Vec<Animal>,
    crate seeds: Vec<Seed>,
}

impl World {
    pub fn eagles(&self) -> &[Animal] {
        &self.eagles
    }

    pub fn birds(&self) -> &[Animal] {
        &self.birds
    }

    pub fn seeds(&self) -> &[Seed] {
        &self.seeds
    }
}

impl World {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let birds = (0..config.world_animals)
            .map(|_| Animal::random(config, rng, Role::Prey))
            .collect();

        let eagles = (0..config.world_animals / 10)
            .map(|_| Animal::random(config, rng, Role::Predator))
            .collect();

        let seeds = (0..config.world_foods).map(|_| Seed::random(rng)).collect();

        Self {
            eagles,
            birds,
            seeds,
        }
    }
}
