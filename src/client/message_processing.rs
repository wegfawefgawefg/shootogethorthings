use glam::Vec2;

use crate::common::{game_objects::Player, server_to_client::ServerToClientMessage};

use super::{state::State, udp_networking::INCOMING_MESSAGE_QUEUE};

pub async fn process_message_queue(state: &mut State) {
    while let Some(message) = INCOMING_MESSAGE_QUEUE.pop() {
        match message {
            ServerToClientMessage::ClientIDAssignment { new_client_id } => {
                state.client_id = Some(new_client_id);
                println!("new id assigned: {}", new_client_id);
            }
            ServerToClientMessage::Welcome { server_message } => {
                println!("Server says: {}", server_message);
            }
            ServerToClientMessage::ClientJoined { id } => {
                println!("Client {} joined", id);
            }
            ServerToClientMessage::ClientLeft { id } => {
                println!("Client {} left", id);
            }
            ServerToClientMessage::ChatMessage { from, message } => {
                println!("{} says: {}", from, message);
            }
            ServerToClientMessage::SpawnPlayer {
                owner_client_id,
                entity_id,
                pos,
            } => {
                state.players.insert(
                    entity_id,
                    Player {
                        owner_client_id,
                        entity_id,
                        pos,
                        vel: Vec2::new(0.0, 0.0),
                    },
                );

                println!("player spawned {}", entity_id);
            }
            ServerToClientMessage::EntityPosition { entity_id, pos } => {
                if let Some(player) = state.players.get_mut(&entity_id) {
                    player.pos = pos;
                }
            }
            ServerToClientMessage::AllPlayers { players } => {
                for player in players {
                    state.players.insert(
                        player.entity_id,
                        Player {
                            owner_client_id: player.owner_client_id,
                            entity_id: player.entity_id,
                            pos: player.pos,
                            vel: Vec2::new(0.0, 0.0),
                        },
                    );
                }
            }
        }
    }
}
