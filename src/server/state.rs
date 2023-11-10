use std::collections::HashMap;

use crate::common::game_objects::Player;

pub struct State {
    pub time_since_last_update: f32,
    pub next_id: u32,
    pub next_eid: u32,
    pub players: HashMap<u32, Player>,
}

impl State {
    pub fn new() -> Self {
        Self {
            time_since_last_update: 0.0,
            next_id: 0,
            next_eid: 0,
            players: HashMap::new(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
