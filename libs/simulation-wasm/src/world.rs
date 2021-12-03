use crate::*;

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub birds: Vec<Animal>,
    pub eagles: Vec<Animal>,
    pub seeds: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let birds = world.birds().iter().map(Animal::from).collect();
        let eagles = world.eagles().iter().map(Animal::from).collect();
        let seeds = world.seeds().iter().map(Food::from).collect();

        Self {
            birds,
            eagles,
            seeds,
        }
    }
}
