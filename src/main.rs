use bracket_lib::prelude::Point;
use ecs::component::{BlocksTile, CombatStats, Monster, Name, Player, Position, Renderable, RunState, State, SufferDamage, Viewshed, WantsToMelee};
use generation::map::{new_map_rooms_and_corridors, Map};
use specs::prelude::*;

mod ecs;
mod game;
mod generation;
mod ui;

fn main() -> bracket_lib::prelude::BError {
    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    

    let map: Map = new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();


    // monsters
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let glyph: bracket_lib::prelude::FontCharType;
        let name: String;

        let roll = bracket_lib::prelude::RandomNumberGenerator::new().roll_dice(1, 2);
        match roll {
            1 => {
                glyph = bracket_lib::prelude::to_cp437('g');
                name = "Goblin".to_string();
            }
            _ => {
                glyph = bracket_lib::prelude::to_cp437('o');
                name = "Orc".to_string();
            }
        }

        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph: glyph,
                fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::RED),
                bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .with(CombatStats {
                max_hp: 16,
                hp: 16,
                defense: 1,
                power: 4,
            })
            .with(BlocksTile {})
            .build();
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y)); // player position

    let context = bracket_lib::prelude::BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .with_fitscreen(true)
        .build()
        .unwrap();


    // player placdement
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('@'),
            fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::YELLOW),
            bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Name { name: "Player".to_string() })
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build();

    bracket_lib::prelude::main_loop(context, gs);

    // println!("Hello, world!");
    Ok(())
}
