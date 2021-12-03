use crate::*;

#[derive(Clone, Debug, Serialize)]
pub struct World {
    pub birds: Vec<Animal>,
    pub eagles: Vec<Animal>,
    pub foods: Vec<Food>,
}

impl From<&sim::World> for World {
    fn from(world: &sim::World) -> Self {
        let birds = world.birds().iter().map(Animal::from).collect();
        let eagles = world.eagles().iter().map(Animal::from).collect();
        let foods = world.foods().iter().map(Food::from).collect();

        Self {
            birds,
            eagles,
            foods,
        }
    }
}
