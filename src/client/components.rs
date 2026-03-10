use glam::Vec2;
use hecs::Entity;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub pos: Vec2,
}

pub struct OwnedByClient {
    pub client_id: u32,
}

pub struct Player;

pub struct InputControlled;

pub struct Health {
    pub hp: u32,
}

#[derive(Clone, Copy)]
pub struct Shape {
    pub dims: Vec2,
}

#[derive(Clone, Copy)]
pub struct Physics {
    pub vel: Vec2,
}

pub struct CaptureInPlayField;

pub struct FreeToLeavePlayField;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Score {
    pub owner: Entity,
    pub score: u32,
}

pub struct OwnedBy {
    pub owner: Entity,
}

pub struct AttachedTo {
    pub entity: Entity,
    pub offset: Vec2,
}

#[derive(Clone, Copy)]
pub struct GrabZone {
    pub radius: f32,
}

pub struct Attachable;

pub struct WantsToGoTo {
    pub pos: Vec2,
}

pub struct LookAt {
    pub entity: Entity,
}

#[derive(Clone, Copy)]
pub struct Enemy;

pub struct Wall;
