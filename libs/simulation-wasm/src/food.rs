use crate::*;

#[derive(Clone, Debug, Serialize)]
pub struct Food {
    pub x: f32,
    pub y: f32,
}

impl<T> From<&T> for Food
where
    T: sim::Food,
{
    fn from(food: &T) -> Self {
        Self {
            x: food.pos().x,
            y: food.pos().y,
        }
    }
}
