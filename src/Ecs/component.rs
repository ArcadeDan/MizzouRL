use bracket_lib::prelude::GameState;
use specs::prelude::*;
use specs_derive::Component;

use crate::Generation::map::{draw_map, player_input, TileType};

use super::view_systems::VisibilitySystem;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
#[derive(Component)]
pub struct Renderable {
    pub glyph: bracket_lib::prelude::FontCharType,
    pub fg: bracket_lib::prelude::RGB,
    pub bg: bracket_lib::prelude::RGB,
}
#[derive(Component, Debug)]
pub struct Player {}

pub struct State {
    pub ecs: World,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<bracket_lib::prelude::Point>,
    pub range: i32,
    pub dirty: bool,
}

impl State {
    pub fn run_systems(&mut self) {
        let mut vis = VisibilitySystem{};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();


        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}
