mod map;
mod player;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
    pub use crate::player::*;
}

use prelude::*;

struct State {
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        Self {
            map: Map::new(),
            player: Player::new(Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2)),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Dungeon Crawl")
        .with_fps_cap(30.0)
        .build()?;

    main_loop(ctx, State::new())
}
