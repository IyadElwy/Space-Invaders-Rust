use macroquad::color;
use macroquad::input;
use macroquad::math::*;
use macroquad::miniquad::window;
use macroquad::text;
use macroquad::texture::*;
use macroquad::window::*;
mod ship;
use ship::ship_mod::Ship;
mod enemies;
mod firing;
use enemies::enemy_factory::enemy_factory_mod::EnemyFactory;
mod load_level_settings;
use load_level_settings::load_level_settings::read_settings;

#[macroquad::main("Space Invaders")]
async fn main() {
    window::set_window_size(640, 680);
    let level_data = read_settings();
    let texture: Texture2D = load_texture("assets/SpaceInvaders.png").await.unwrap();
    let mut ship = Ship::create(texture.weak_clone(), 3);
    let mut enemy_factory: EnemyFactory = EnemyFactory::create(texture.weak_clone(), level_data);
    let mut level: u8 = 0;
    let mut score: u32 = 0;
    let mut lives: u8 = 3;
    loop {
        clear_background(color::BLACK);
        text::draw_text(
            &format!("Level: {}\t\tScore: {}", level, score),
            0.,
            30.,
            40.,
            color::WHITE,
        );
        (1..=lives).into_iter().for_each(|i| {
            draw_texture_ex(
                &texture,
                screen_width() - ((i as f32) * 50.),
                0.,
                color::GREEN,
                DrawTextureParams {
                    dest_size: Some(Vec2 { x: 50., y: 50. }),
                    source: Some(Rect { x: 68., y: 0., w: 15., h: 15. }),
                    ..Default::default()
                },
            );
        });

        if input::get_keys_down().contains(&input::KeyCode::Right) {
            ship.move_right();
        }
        if input::get_keys_down().contains(&input::KeyCode::Left) {
            ship.move_left();
        }
        if input::get_keys_pressed().contains(&input::KeyCode::Space) {
            ship.fire();
        }

        enemy_factory.create_wave(&mut level, false);
        ship.draw();
        enemy_factory.draw();
        enemy_factory.detect_enemy_collision(&mut ship.active_fire_blasts, &mut score);
        ship.detect_enemy_collision(&mut enemy_factory.enemies, &mut lives);

        if lives <= 0 {
            level = 0;
            score = 0;
            lives = 3;
            enemy_factory.create_wave(&mut level, true);
        }
        next_frame().await;
    }
}
