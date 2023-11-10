use super::event_processing::PlayingInputs;

pub struct State {
    pub running: bool,
    pub time_since_last_update: f32,
    // pub client_id: Option<u32>,
    pub players: Vec<u32>,

    pub playing_inputs: PlayingInputs,
}

impl State {
    pub fn new() -> Self {
        Self {
            running: true,
            time_since_last_update: 0.0,

            players: Vec::new(),

            playing_inputs: PlayingInputs::new(),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}
