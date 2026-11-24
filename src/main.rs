use vector2::Vector2;
use raylib::prelude::*;
mod player;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Test game")
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            player::move_character(&Vector2::new(0.0, -1.0));
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            player::move_character(&Vector2::new(-1.0, 0.0));
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            player::move_character(&Vector2::new(1.0, 0.0));
        }

        let mut d = rl.begin_drawing(&thread);   

        d.clear_background(Color::BLACK);
        
        player::draw_player(&mut d, &Vector2::new(320.0, 240.0));
    }
}