use bracket_lib::prelude::BTerm;

use crate::ecs::component::{RunState, State};

pub fn main_menu(gs: &mut State, ctx: &mut BTerm) {
    let runstate = gs.ecs.fetch::<RunState>();

}