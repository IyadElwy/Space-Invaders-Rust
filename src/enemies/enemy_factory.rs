pub mod EnemyFactoryMod {

    use crate::enemies::enemy::EnemyMod::Enemy;
    use macroquad::color;
    use macroquad::experimental::animation;
    use macroquad::input;
    use macroquad::math::*;
    use macroquad::shapes;
    use macroquad::text;
    use macroquad::texture::*;
    use macroquad::time::get_frame_time;
    use macroquad::window::*;

    pub struct EnemyFactory {
        figure_texture: Texture2D,
        enemies: Vec<Enemy>,
        move_timer: f32,
        move_interval: f32,
        move_right: bool,
    }

    impl EnemyFactory {
        pub fn create(figure_texture: Texture2D) -> EnemyFactory {
            EnemyFactory {
                figure_texture,
                enemies: Vec::new(),
                move_timer: 0.0,
                move_interval: 1.2,
                move_right: true,
            }
        }

        pub fn create_wave(&mut self) {
            let mut last_x_pos: f32 = 0.;
            let mut last_y_pos: f32 = 0.;
            self.enemies = (0..55)
                .map(|i| {
                    last_x_pos += 50.;
                    if i % 11 == 0 {
                        last_y_pos += 40.;
                        last_x_pos = 0.;
                    }
                    Enemy::create(self.figure_texture.weak_clone(), last_x_pos, last_y_pos)
                })
                .collect();
        }

        pub fn draw(&mut self) {
            self.update();
            self.enemies.iter_mut().for_each(|e| {
                e.draw();
            });
        }

        fn update(&mut self) {
            self.move_timer += get_frame_time();

            let change_direction = self.enemies.iter().any(|e| {
                if (self.move_right && e.x_position >= 590.)
                    || (!self.move_right && e.x_position <= 0.)
                {
                    return true;
                }
                false
            });

            if self.move_timer >= self.move_interval {
                if change_direction {
                    self.move_right = !self.move_right;
                    for enemy in &mut self.enemies {
                        enemy.animation_state = !enemy.animation_state;
                        enemy.move_down();
                    }
                } else {
                    for enemy in &mut self.enemies {
                        enemy.move_diagonal(self.move_right);
                        enemy.animation_state = !enemy.animation_state;
                    }
                }
                self.move_timer = 0.0;
            }
        }
    }
}
