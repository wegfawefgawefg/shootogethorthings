use std::{collections::HashMap, time::Instant};

use crate::common::client_to_server::{ClientToServerMessage, ClientToServerMessageData};

use super::{
    message_processing::process_message_queue, state::State, udp_networking::INCOMING_MESSAGE_QUEUE,
};

pub const FRAMES_PER_SECOND: u32 = 60;
const TIMESTEP: f32 = 1.0 / FRAMES_PER_SECOND as f32;

pub async fn main_loop(state: &mut State) {
    let mut previous_time = Instant::now();
    loop {
        process_message_queue(state).await;

        let current_time = Instant::now();
        let dt = (current_time - previous_time).as_secs_f32();
        previous_time = current_time;

        state.time_since_last_update += dt;
        while state.time_since_last_update > TIMESTEP {
            state.time_since_last_update -= TIMESTEP;

            step(state);
        }
    }
}

pub fn step(state: &mut State) {
    for (_, player) in state.players.iter_mut() {
        player.step();
    }
}
