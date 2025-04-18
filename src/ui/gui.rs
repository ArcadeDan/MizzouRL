// ui/gui.rs
use bracket_lib::prelude::*;
use specs::prelude::*;
use crate::game::GameLog; // Removed unused LogLevel

pub fn draw_log(ctx: &mut BTerm, log: &GameLog, y_pos: i32, height: i32) {
    let mut y = y_pos;
    let max_display = height as usize;

    // Get the last 'max_display' entries
    let display_count = std::cmp::min(max_display, log.entries.len());
    let display_entries = log.entries.iter().rev().take(display_count).collect::<Vec<_>>();

    // Display entries in reverse order (newest at the bottom)
    for entry in display_entries.iter().rev() {
        // Get terminal height directly through ctx.height_pixels or hardcode based on your setup
        let term_height = 50; // Since you used simple80x50 in your BTermBuilder
        if y < term_height {
            ctx.print_color(
                2,
                y,
                log.get_color(&entry.level),
                RGB::named(BLACK),
                &entry.text
            );
            y += 1;
        }
    }
}

pub fn draw_ui(ecs: &World, ctx: &mut BTerm) {
    // Draw a box for the log
    ctx.draw_box(
        0,
        43,
        79,
        6,
        RGB::named(WHITE),
        RGB::named(BLACK)
    );

    ctx.print_color(
        12,
        43,
        RGB::named(YELLOW),
        RGB::named(BLACK),
        "Halls of Laffere"
    );

    // Get log and display it
    let log = ecs.fetch::<GameLog>();
    draw_log(ctx, &log, 44, 5); // 5 lines of message space
}