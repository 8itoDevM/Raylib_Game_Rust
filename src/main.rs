use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Test game")
        .build();

    rl.set_target_fps(0);

    let mut counter: i32 = 0;

    while !rl.window_should_close() {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            println!("Space key pressed");
        }

        if rl.is_key_down(KeyboardKey::KEY_C){
            counter += 1;
            println!("Counter: {}", counter);
        }

        let mouse = rl.get_mouse_position();
        let scroll = rl.get_mouse_wheel_move();

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hey, im 8ito", 20, 20, 20, Color::BLACK);

        let frame_rate: f32 = 1.00/&d.get_frame_time();

        d.draw_text(&frame_rate.to_string(), 20, 50, 20, Color::BLACK);

        d.draw_text(&format!("Mouse: ({:.0}, {:.0}), Scroll {:.1}", mouse.x, mouse.y, scroll), 20, 80, 20, Color::BLACK);
        d.draw_text(".", (mouse.x - 8.0) as i32, (mouse.y - 133.0) as i32, 180, Color::BLACK);

        d.draw_rectangle(150, 150, 50, 50, Color::BLUE);        
    }
}