use hecs::World;

use crate::client::{
    components::{Physics, Transform},
    state::State,
};

pub fn step_physics(ecs: &mut World, state: &mut State) {
    for (_, (transform, physics)) in ecs.query::<(&mut Transform, &mut Physics)>().iter() {
        transform.pos += physics.vel;
    }
}
