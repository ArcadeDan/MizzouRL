use bracket_lib::color::*;
use bracket_lib::prelude::{BTerm, Console, RGB};
use specs::{prelude, Join, World, WorldExt};

use crate::ecs::component::{CombatStats, Name, Player};
use crate::game::gamelog::GameLog;
use crate::generation::map::Map;

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));


    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();

    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!("HP: {} / {}", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, RGB::named(YELLOW), RGB::named(BLACK), &health);
        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(RED), RGB::named(BLACK));
    }


    let log = ecs.fetch::<GameLog>();
    let mut y = 44;
    
    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y,  s);
        }
        y += 1;
    }

    let mouse_pos = ctx.mouse_pos();
    ctx.set_bg(mouse_pos.0, mouse_pos.1, RGB::named(MAGENTA));
    draw_tooltips(ecs, ctx);


    // ctx.print_color(
    //     12,
    //     43,
    //     RGB::named(YELLOW),
    //     RGB::named(BLACK),
    //     "Halls of Laffere",
    // );
}

fn draw_tooltips(ecs: &World, ctx: &mut bracket_lib::prelude::BTerm) {
    let map = ecs.fetch::<Map>();
    let names = ecs.read_storage::<Name>();
}
