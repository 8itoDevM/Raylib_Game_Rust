use vector2::Vector2;
use raylib::prelude::*;

fn main(){
    //let mut direction = Vector2::new(0,0);
}

pub fn move_character(direction: &Vector2) {
    println!("{}", direction);
}

pub fn draw_player(d: &mut RaylibDrawHandle, player_position: &Vector2){
    d.draw_circle(
        player_position.x as i32,
        player_position.y as i32,
        10.0,
        Color::RED,
    );
}