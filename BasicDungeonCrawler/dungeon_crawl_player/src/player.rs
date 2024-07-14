use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        // ctx.set_active_console(1);
        ctx.set_fancy(
            PointF::new(self.position.x as f32, self.position.y as f32),
            1,
            Degrees::new(0.0),
            PointF::new(5.0, 5.0),
            WHITE,
            BLACK,
            64,
        );
        // ctx.set(
        //     self.position.x,
        //     self.position.y,
        //     WHITE,
        //     BLACK,
        //     to_cp437('@'),
        // );
        // ctx.set_active_console(0);
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };
            let new_position = self.position + delta;

            if map.can_enter_tile(new_position) {
                self.position = new_position;
            }
        }
    }
}
