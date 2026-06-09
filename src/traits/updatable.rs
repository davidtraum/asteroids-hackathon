use crate::structs::context::Context;

pub trait Updatable {
    fn update(&mut self, context: &Context);
}
