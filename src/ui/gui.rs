use bracket_lib::prelude::{RGB, BTerm, Console};
use bracket_lib::color::*;
use specs::{prelude, World};

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));
    //ctx.draw_bar_horizontal(28, 43, 51, 1, 100, RGB::named(RED), RGB::named(BLACK));
    ctx.print_color(12, 43, RGB::named(YELLOW), RGB::named(BLACK), "Halls of Laffere");
}