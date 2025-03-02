use std::cmp::{max, min};

use specs::{Join, World, WorldExt};

use crate::Ecs::component::{Player, Position, State, TileType};

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn try_to_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut bracket_lib::prelude::BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            bracket_lib::prelude::VirtualKeyCode::Left => try_to_move_player(-1, 0, &mut gs.ecs),
            bracket_lib::prelude::VirtualKeyCode::Right => try_to_move_player(1, 0, &mut gs.ecs),
            bracket_lib::prelude::VirtualKeyCode::Up => try_to_move_player(0, -1, &mut gs.ecs),
            bracket_lib::prelude::VirtualKeyCode::Down => try_to_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; 80 * 50];

    for x in 0..80 {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, 49)] = TileType::Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(79, y)] = TileType::Wall;
    }

    let mut rng = bracket_lib::prelude::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut bracket_lib::prelude::BTerm) {
    let mut x = 0;
    let mut y = 0;
    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(
                    x,
                    y,
                    bracket_lib::color::RGB::from_f32(0.5, 0.5, 0.5),
                    bracket_lib::color::RGB::from_f32(0., 0., 0.),
                    bracket_lib::prelude::to_cp437('.'),
                );
            }
            TileType::Wall => {
                ctx.set(
                    x,
                    y,
                    bracket_lib::color::RGB::from_f32(0.0, 1.0, 0.0),
                    bracket_lib::color::RGB::from_f32(0., 0., 0.),
                    bracket_lib::prelude::to_cp437('#'),
                );
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
