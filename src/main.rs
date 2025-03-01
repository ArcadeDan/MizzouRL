use std::cmp::{max, min};

use bracket_lib::prelude::{GameState, Tile};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}
#[derive(Component)]
struct Renderable {
    glyph: bracket_lib::prelude::FontCharType,
    fg: bracket_lib::prelude::RGB,
    bg: bracket_lib::prelude::RGB,
}
#[derive(Component, Debug)]
struct Player {}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut bracket_lib::prelude::BTerm) {
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

fn player_input(gs: &mut State, ctx: &mut bracket_lib::prelude::BTerm) {
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

struct State {
    ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();
        
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    let mut gs = State { ecs: World::new() };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    gs.ecs.insert(new_map());

    let context = bracket_lib::prelude::BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .build()
        .unwrap();

    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('@'),
            fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::YELLOW),
            bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
        })
        .build();

    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: bracket_lib::prelude::to_cp437('☺'),
                fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::RED),
                bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
            })
            .with(Player {})
            .build();
    }

    bracket_lib::prelude::main_loop(context, gs);

    println!("Hello, world!");
}
