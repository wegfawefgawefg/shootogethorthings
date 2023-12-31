use common::client_to_server::{ClientToServerMessage, ClientToServerMessageData};

use client::{
    event_processing::process_events_and_input, message_processing::process_message_queue,
    state::State,
};
mod client;
mod common;
mod server;

pub const FRAMES_PER_SECOND: u32 = 60;
const TIMESTEP: f32 = 1.0 / FRAMES_PER_SECOND as f32;

const POSITION_TRANSMIT_FREQUENCY: u32 = 1;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let result = client::udp_networking::init_connection().await;
    if let Err(e) = result {
        eprintln!("Error connecting to server: {:?}", e);
        return Ok(());
    }

    // request a new player
    if client::udp_networking::OUTBOUND_MESSAGE_QUEUE
        .push(ClientToServerMessage::new(
            ClientToServerMessageData::RequestToSpawnPlayer,
        ))
        .is_err()
    {
        eprintln!("Outbound message queue full: dropping message");
    }

    // request all players
    if client::udp_networking::OUTBOUND_MESSAGE_QUEUE
        .push(ClientToServerMessage::new(
            ClientToServerMessageData::RequestAllPlayers,
        ))
        .is_err()
    {
        eprintln!("Outbound message queue full: dropping message");
    }

    let (mut rl, mut rlt, mut render_texture) = client::graphics::init_graphics();

    ////////////////    MAIN LOOP    ////////////////
    let mut state = client::state::State::new();
    let mut current_frame: u32 = 0;

    while !rl.window_should_close() {
        process_events_and_input(&mut rl, &mut state);
        process_message_queue(&mut state).await;

        interval_transmit_position(current_frame, POSITION_TRANSMIT_FREQUENCY, &state);

        let dt = rl.get_frame_time();
        state.time_since_last_update += dt;
        while state.time_since_last_update > TIMESTEP {
            state.time_since_last_update -= TIMESTEP;

            client::game::step(&mut state);
            current_frame += 1;
        }

        client::graphics::render(&mut rl, &mut rlt, &mut render_texture, &state);

        if !state.running {
            break;
        }
    }
    Ok(())
}

pub fn interval_transmit_position(current_frame: u32, interval: u32, state: &State) {
    if current_frame % interval != 0 {
        return;
    }
    for player in state.players.values() {
        if let Some(client_id) = state.client_id {
            if player.owner_client_id == client_id
                && client::udp_networking::OUTBOUND_MESSAGE_QUEUE
                    .push(ClientToServerMessage::new(
                        ClientToServerMessageData::EntityPosition {
                            entity_id: player.entity_id,
                            pos: player.pos,
                        },
                    ))
                    .is_err()
            {
                eprintln!("Outbound message queue full: dropping message");
            }
        }
    }
}
