use glam::Vec2;
use raylib::prelude::*;

use super::state::State;

const PLAYER_SPEED: f32 = 1.0;

pub fn process_events_and_input(rl: &mut RaylibHandle, state: &mut State) {
    if rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ESCAPE) {
        state.running = false;
    }

    let mouse_pos_rl = rl.get_mouse_position();
    let _mouse_pos = Vec2::new(mouse_pos_rl.x, mouse_pos_rl.y);

    let mut inputs = PlayingInputs::new();

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W) {
        inputs.up = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A) {
        inputs.left = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S) {
        inputs.down = true;
    }
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D) {
        inputs.right = true;
    }

    if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_BUTTON_LEFT) {
        inputs.shoot = true;
    }

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE) {
        inputs.confirm = true;
    }

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_ONE) {
        inputs.weapon_1 = true;
    }

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_TWO) {
        inputs.weapon_2 = true;
    }

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_THREE) {
        inputs.weapon_3 = true;
    }

    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_FOUR) {
        inputs.weapon_4 = true;
    }

    state.playing_inputs = inputs;
}

pub struct PlayingInputs {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,

    pub shoot: bool,
    pub confirm: bool,

    pub weapon_1: bool,
    pub weapon_2: bool,
    pub weapon_3: bool,
    pub weapon_4: bool,
}
impl PlayingInputs {
    pub fn new() -> PlayingInputs {
        PlayingInputs {
            left: false,
            right: false,
            up: false,
            down: false,

            shoot: false,

            confirm: false,

            weapon_1: false,
            weapon_2: false,
            weapon_3: false,
            weapon_4: false,
        }
    }
}
