use raylib::prelude::*;

use super::state::State;

pub fn draw(state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("Multiplayer!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    // render all players
    for (_, player) in state.players.iter() {
        d.draw_circle(player.pos.x as i32, player.pos.y as i32, 6.0, Color::BLUE);
    }
}
