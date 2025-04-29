use bracket_lib::color::*;
use bracket_lib::prelude::{BTerm, Console, Point, RGB};
use specs::{prelude, Join, World, WorldExt};

use crate::ecs::component::{CombatStats, Name, Player, Position};
use crate::game::gamelog::GameLog;
use crate::generation::map::Map;
use crate::ui::camera::Camera;

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    // Draw the UI box at the bottom of the screen
    ctx.draw_box(0, 43, 79, 6, RGB::named(WHITE), RGB::named(BLACK));

    // Display player health
    let combat_stats = ecs.read_storage::<CombatStats>();
    let players = ecs.read_storage::<Player>();

    for (_player, stats) in (&players, &combat_stats).join() {
        let health = format!("HP: {} / {}", stats.hp, stats.max_hp);
        ctx.print_color(12, 43, RGB::named(YELLOW), RGB::named(BLACK), &health);
        ctx.draw_bar_horizontal(28, 43, 51, stats.hp, stats.max_hp, RGB::named(RED), RGB::named(BLACK));
    }

    // Display game log
    let log = ecs.fetch::<GameLog>();
    let mut y = 44;

    for s in log.entries.iter().rev() {
        if y < 49 {
            ctx.print(2, y, s);
        }
        y += 1;
    }

    // Handle mouse hover tooltips
    let mouse_pos = ctx.mouse_pos();
    draw_tooltips(ecs, ctx, mouse_pos);
}

fn draw_tooltips(ecs: &World, ctx: &mut BTerm, mouse_pos: (i32, i32)) {
    // Only process mouse hover if it's in the main map area (not in the UI)
    if mouse_pos.1 < 43 {
        // Convert screen coordinates to world coordinates
        let camera = ecs.fetch::<Camera>();
        let map = ecs.fetch::<Map>();
        let world_pos = camera.screen_to_world(mouse_pos.0, mouse_pos.1);

        // Indicate mouse position with highlighted background
        if let Some((screen_x, screen_y)) = camera.world_to_screen(world_pos) {
            ctx.set_bg(screen_x, screen_y, RGB::named(MAGENTA));
        }

        // Check if the mouse is hovering over an entity
        if world_pos.x >= 0 && world_pos.x < map.width &&
            world_pos.y >= 0 && world_pos.y < map.height &&
            map.visible_tiles[map.xy_idx(world_pos.x, world_pos.y)] {

            let names = ecs.read_storage::<Name>();
            let positions = ecs.read_storage::<Position>();

            // Find entities at the mouse position
            let mut tooltip: Vec<String> = Vec::new();
            for (name, pos) in (&names, &positions).join() {
                if pos.x == world_pos.x && pos.y == world_pos.y {
                    tooltip.push(name.name.clone());
                }
            }

            // Display tooltip if there are entities
            if !tooltip.is_empty() {
                let width = tooltip.iter().map(|s| s.len()).max().unwrap_or(0) + 3;
                let bg_color = RGB::named(GREY);
                let fg_color = RGB::named(WHITE);

                // Choose position for tooltip - either above or below mouse
                let tooltip_y = if mouse_pos.1 > 25 { mouse_pos.1 - tooltip.len() as i32 - 1 } else { mouse_pos.1 + 1 };

                // Draw tooltip background
                for (i, _) in tooltip.iter().enumerate() {
                    ctx.draw_box(mouse_pos.0, tooltip_y + i as i32, width as i32, 1, fg_color, bg_color);
                }

                // Draw tooltip text
                for (i, s) in tooltip.iter().enumerate() {
                    ctx.print_color(mouse_pos.0 + 1, tooltip_y + i as i32, fg_color, bg_color, s);
                }
            }
        }
    }
}