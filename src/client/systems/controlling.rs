use hecs::World;

use crate::client::{
    components::{InputControlled, Physics},
    state::State,
};

pub const PLAYER_SPEED: f32 = 2.0;
pub fn control_player(ecs: &mut World, state: &mut State) {
    for (_, physics) in ecs
        .query::<&mut Physics>()
        .with::<&InputControlled>()
        .iter()
    {
        if state.playing_inputs.up {
            physics.vel.y = -PLAYER_SPEED;
        } else if state.playing_inputs.down {
            physics.vel.y = PLAYER_SPEED;
        } else {
            physics.vel.y = 0.0;
        }

        if state.playing_inputs.left {
            physics.vel.x = -PLAYER_SPEED;
        } else if state.playing_inputs.right {
            physics.vel.x = PLAYER_SPEED;
        } else {
            physics.vel.x = 0.0;
        }
    }
}
