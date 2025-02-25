use bracket_lib::prelude::GameState;
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

struct State {
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut bracket_lib::prelude::BTerm) {
        ctx.cls();
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}


fn main() {


    let mut gs = State {
        ecs: World::new()
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();


    let context = bracket_lib::prelude::BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .build()
        .unwrap();

    gs.ecs.create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: bracket_lib::prelude::to_cp437('@'),
            fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::YELLOW),
            bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
        })
        .build();


    for i in 0..10 {
        gs.ecs.create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: bracket_lib::prelude::to_cp437('☺'),
                fg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::RED),
                bg: bracket_lib::prelude::RGB::named(bracket_lib::prelude::BLACK),
            })
            .build();
    }

    bracket_lib::prelude::main_loop(context, gs);

    println!("Hello, world!");
}
