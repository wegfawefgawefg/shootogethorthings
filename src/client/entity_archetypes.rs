use glam::Vec2;
use hecs::World;

use super::{
    components::{Health, InputControlled, OwnedByClient, Physics, Player, Shape, Transform},
    graphics::DIMS,
    state::State,
    udp_networking::CLIENT_ID,
};

pub const PLAYER_SHAPE: Vec2 = Vec2::new(16.0, 16.0);
pub fn spawn_player(ecs: &mut World, _state: &mut State, owner_client_id: u32) {
    let player_entity = ecs.spawn((
        Player,
        Transform {
            pos: DIMS.as_vec2() / 2.0,
        },
        Physics { vel: Vec2::ZERO },
        Shape { dims: PLAYER_SHAPE },
        Health { hp: 100 },
        OwnedByClient {
            client_id: owner_client_id,
        },
    ));

    {
        let client_id = CLIENT_ID.load(std::sync::atomic::Ordering::SeqCst);
        if client_id == owner_client_id {
            let _ = ecs.insert_one(player_entity, InputControlled);
        }
    }
}
