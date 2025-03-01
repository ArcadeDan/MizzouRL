use bracket_lib::prelude::{FontCharType, GameState, RGB};

use specs::{Component, World, DenseVecStorage};
use specs_derive::Component;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub fg: RGB,
    pub bg: RGB,
}

pub struct State {
    pub ecs: World,
}
