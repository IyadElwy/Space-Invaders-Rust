use std::io::empty;

use macroquad::color;
use macroquad::input;
use macroquad::math::*;
use macroquad::miniquad::window;
use macroquad::shapes;
use macroquad::text;
use macroquad::texture::*;
use macroquad::window::*;

mod ship;
use ship::ShipMod::Ship;

mod firing;

mod enemies;
use enemies::enemy_factory::EnemyFactoryMod::EnemyFactory;

#[macroquad::main("Space Invaders")]
async fn main() {
    window::set_window_size(640, 680);
    let texture: Texture2D = load_texture("assets/SpaceInvaders.png").await.unwrap();
    let mut ship = Ship::create(texture.weak_clone(), 3);
    let mut enemy_factory = EnemyFactory::create(texture.weak_clone());
    enemy_factory.create_wave();
    loop {
        clear_background(color::BLACK);

        if input::get_keys_down().contains(&input::KeyCode::Right) {
            ship.move_right();
        }
        if input::get_keys_down().contains(&input::KeyCode::Left) {
            ship.move_left();
        }
        if input::get_keys_pressed().contains(&input::KeyCode::Space) {
            ship.fire();
        }

        ship.draw();
        enemy_factory.draw();
        next_frame().await;
    }
}
