use hecs::World;

use super::{state::State, systems};

pub fn step(ecs: &mut World, state: &mut State) {
    systems::controlling::control_player(ecs, state);
    systems::physics::step_physics(ecs, state);
}
