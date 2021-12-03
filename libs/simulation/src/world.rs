use crate::*;

#[derive(Debug)]
pub struct World {
    crate eagles: Vec<Animal>,
    crate birds: Vec<Animal>,
    crate foods: Vec<Food>,
}

impl World {
    pub fn eagles(&self) -> &[Animal] {
        &self.eagles
    }

    pub fn birds(&self) -> &[Animal] {
        &self.birds
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl World {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let birds = (0..config.world_animals)
            .map(|_| Animal::random(config, rng, Role::Prey))
            .collect();

        let eagles = (0..10)
            .map(|_| Animal::random(config, rng, Role::Predator))
            .collect();

        let foods = (0..config.world_foods).map(|_| Food::random(rng)).collect();

        Self {
            eagles,
            birds,
            foods,
        }
    }
}
