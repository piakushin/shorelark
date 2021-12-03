use crate::*;

#[derive(Debug)]
pub struct World {
    crate animals: Vec<Animal>,
    crate foods: Vec<Food>,
}

impl World {
    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }

    pub fn birds(&self) -> impl Iterator<Item = &Animal> + '_ {
        self.animals.iter().filter(|a| a.is_prey())
    }

    pub fn predators(&self) -> impl Iterator<Item = &Animal> + '_ {
        self.animals.iter().filter(|a| a.is_predator())
    }
}

impl World {
    crate fn random(config: &Config, rng: &mut dyn RngCore) -> Self {
        let mut animals: Vec<Animal> = (0..config.world_animals)
            .map(|_| Animal::random(config, rng, Role::Prey))
            .collect();

        animals.extend((0..10).map(|_| Animal::random(config, rng, Role::Predator)));

        let foods = (0..config.world_foods).map(|_| Food::random(rng)).collect();

        Self { animals, foods }
    }
}
