use raylib::prelude::*;

pub struct Player {
    pub position: Vector2,
    pub speed: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position: Vector2::new(320.0, 240.0),
            speed: 200.0,
        }
    }

    pub fn move_player(&mut self, direction: Vector2, delta_time: f32) {
        self.position.x += direction.x * self.speed * delta_time;
        self.position.y += direction.y * self.speed * delta_time;
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        d.draw_circle(
            self.position.x as i32,
            self.position.y as i32,
            10.0,
            Color::RED,
        );
    }
}