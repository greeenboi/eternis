use rltk::{GameState, Rltk, RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};
use specs_derive::Component;

struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = VecStoraage<Self>;
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World
}

let mut gs = State {
    ecs: World::new()
}

gs.ecs.register::<Position>();
gs.ecs.register::<Renderable>();
for i in 0..15 {
    gs.ecs
        .create_entity()
        .with(Position { x: i*7, y: i*4 })
        .with(Renderable {
            glyph: rltk::tocp436('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .build();
}

let positions = self.ecs.read_storage::<Position>();
let renderables = self.ecs.read_storage::<Renderable>();

for (pos, render) in (&positions, &renderables).join() {
    ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
}
