use bracket_lib::prelude::{RGB, BTerm, Console};
use bracket_lib::color::*;
use specs::{prelude, World};

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), BLACK);
}