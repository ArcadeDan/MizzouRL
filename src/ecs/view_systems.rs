use crate::generation::map::{Map, TileType, draw_map};
use bracket_lib::pathfinding::field_of_view;
use bracket_lib::prelude::{BTerm, Point, RGB, to_cp437};
use specs::prelude::*;

use super::component::{Player, Position, Renderable, Viewshed};
use crate::Camera;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        WriteExpect<'a, Point>, // Player position for camera centering
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player, mut player_pos) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                viewshed
                    .visible_tiles
                    .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

                // If this is the player, reveal what they can see
                let _p: Option<&Player> = player.get(ent);
                if let Some(_p) = _p {
                    // Update player_pos for camera system
                    *player_pos = Point::new(pos.x, pos.y);

                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.revealed_tiles[idx] = true;
                        map.visible_tiles[idx] = true;
                    }
                }
            }
        }
    }
}

// Add CameraSystem directly to view_systems.rs
pub struct CameraSystem {}

impl<'a> System<'a> for CameraSystem {
    type SystemData = (
        WriteExpect<'a, Camera>,
        ReadExpect<'a, Point>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut camera, player_pos) = data;
        camera.center_on_point(*player_pos);
    }
}

// Create a separate render system that is not a specs System
// This is a standalone struct that will handle rendering
pub struct RenderSystem {}

impl RenderSystem {
    pub fn render(&self, ecs: &World, ctx: &mut BTerm) {
        let positions = ecs.read_storage::<Position>();
        let renderables = ecs.read_storage::<Renderable>();
        let map = ecs.fetch::<Map>();
        let camera = ecs.fetch::<Camera>();

        // Draw map
        draw_map(ecs, ctx, &camera);

        // Draw entities
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                if let Some((screen_x, screen_y)) = camera.world_to_screen(Point::new(pos.x, pos.y)) {
                    ctx.set(screen_x, screen_y, render.fg, render.bg, render.glyph);
                }
            }
        }
    }
}

fn get_tile_glyph(idx: usize, map: &Map) -> (bracket_lib::prelude::FontCharType, RGB, RGB) {
    let black = RGB::named(bracket_lib::prelude::BLACK);

    match map.tiles[idx] {
        crate::generation::map::TileType::Floor => (to_cp437('.'), RGB::named(bracket_lib::prelude::GRAY), black),
        crate::generation::map::TileType::Wall => (to_cp437('#'), RGB::named(bracket_lib::prelude::GREEN), black),
    }
}