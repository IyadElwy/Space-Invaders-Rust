use macroquad::color;
use macroquad::input;
use macroquad::math::*;
use macroquad::shapes;
use macroquad::text;
use macroquad::texture::*;
use macroquad::window::*;

mod ship;
use ship::Ship::Ship;

mod firing;
use firing::Firing::FireBlast;

#[macroquad::main("Space Invaders")]
async fn main() {
    let texture: Texture2D = load_texture("assets/SpaceInvaders.png").await.unwrap();
    let mut ship = Ship::create(texture.weak_clone());
    const FIRINGLIMIT: usize = 3;
    let mut active_fire_blasts: Vec<FireBlast> = Vec::with_capacity(3);

    loop {
        clear_background(color::BLACK);
        ship.draw();

        if input::get_keys_down().contains(&input::KeyCode::Right) {
            ship.move_right();
        }
        if input::get_keys_down().contains(&input::KeyCode::Left) {
            ship.move_left();
        }

        if input::get_keys_pressed().contains(&input::KeyCode::Space) {
            if active_fire_blasts.len() < FIRINGLIMIT {
                let blast = FireBlast::create(
                    texture.weak_clone(),
                    ship.x_position + 11.,
                    screen_height() - 88.0,
                );
                active_fire_blasts.push(blast);
            }
        }

        let mut new_active_fire_blasts = Vec::with_capacity(active_fire_blasts.len());
        for mut blast in active_fire_blasts.into_iter() {
            if blast.active {
                blast.fire();
                new_active_fire_blasts.push(blast);
            }
        }
        active_fire_blasts = new_active_fire_blasts;

        next_frame().await;
    }
}
