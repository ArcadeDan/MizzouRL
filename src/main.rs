use bracket_lib::prelude::{self, main_loop, BError, BTermBuilder, GameState};


struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut prelude::BTerm) {
        ctx.print(1, 1, "Hello World");
    }
}

fn main() -> BError{
    let context = BTermBuilder::simple80x50()
        .with_title("Mizzou Roguelike")
        .build()?;

    let gs: State = State {};
    main_loop(context, gs)
}
