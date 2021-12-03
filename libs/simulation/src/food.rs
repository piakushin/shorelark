use crate::*;

pub trait Food {
    fn pos(&self) -> na::Point2<f32>;
}

impl Food for Seed {
    fn pos(&self) -> na::Point2<f32> {
        self.position
    }
}

impl Food for Animal {
    fn pos(&self) -> na::Point2<f32> {
        if matches!(self.role, Role::Predator) {
            panic!("predator can't be food");
        }
        self.position
    }
}
