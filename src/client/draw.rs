use hecs::World;
use raylib::prelude::{Color, RaylibDraw, RaylibDrawHandle, RaylibTextureMode};

use super::{
    components::{Shape, Transform},
    state::State,
};

pub fn draw(ecs: &World, state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    d.draw_text("Multiplayer!", 12, 12, 12, Color::WHITE);
    let mouse_pos = d.get_mouse_position();
    d.draw_circle(mouse_pos.x as i32, mouse_pos.y as i32, 6.0, Color::GREEN);

    draw_players(ecs, state, d);
}

pub fn draw_players(ecs: &World, _state: &State, d: &mut RaylibTextureMode<RaylibDrawHandle>) {
    for (_, (transform, shape)) in ecs.query::<(&Transform, &Shape)>().iter() {
        d.draw_circle(
            transform.pos.x as i32,
            transform.pos.y as i32,
            shape.dims.x as f32,
            Color::BLUE,
        );
    }
}
