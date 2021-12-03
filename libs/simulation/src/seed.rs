use crate::*;

#[derive(Debug)]
pub struct Seed {
    crate position: na::Point2<f32>,
}

impl Seed {
    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

impl Seed {
    crate fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }
}
