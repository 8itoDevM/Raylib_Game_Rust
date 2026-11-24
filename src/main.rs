use raylib::prelude::*;
mod player;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Test game")
        .build();

    let mut player = player::Player::new();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let delta_time: f32 = rl.get_frame_time();
        let mut direction = Vector2::zero();

        if rl.is_key_down(KeyboardKey::KEY_UP) {
            direction.y -= 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            direction.y += 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            direction.x -= 1.0;
        }

        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            direction.x += 1.0;
        }
        
        player.move_player(direction, delta_time);

        let mut d = rl.begin_drawing(&thread);   

        d.clear_background(Color::WHITE);
        player.draw(&mut d);
    }
}