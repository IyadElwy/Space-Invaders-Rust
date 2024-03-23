pub mod enemy_mod {
    use crate::enemies::firing::firing::FireBlast;
    use macroquad::color;
    use macroquad::math::*;
    use macroquad::texture::*;

    pub enum EnemyType {
        Type1(u8, u32),
        Type2(u8, u32),
        Type3(u8, u32),
        Type4(u8, u32),
        Type5(u8, u32),
    }

    impl EnemyType {
        pub fn enemy_data(&self) -> (u8, u32) {
            match self {
                EnemyType::Type1(health, score) => (*health, *score),
                EnemyType::Type2(health, score) => (*health, *score),
                EnemyType::Type3(health, score) => (*health, *score),
                EnemyType::Type4(health, score) => (*health, *score),
                EnemyType::Type5(health, score) => (*health, *score),
            }
        }
    }

    pub struct Enemy {
        figure_texture: Texture2D,
        enemy_type: EnemyType,
        pub x_position: f32,
        pub y_position: f32,
        pub width: f32,
        pub height: f32,
        pub animation_state: bool,
        pub is_alive: bool,
        firing_limit: usize,
        pub active_fire_blasts: Vec<FireBlast>,
        pub cur_health: u8,
        pub hit_score: u32,
    }

    impl Enemy {
        pub fn create(
            figure_texture: Texture2D,
            enemy_type_nr: &str,
            x_position: f32,
            y_position: f32,
        ) -> Enemy {
            let active_fire_blasts: Vec<FireBlast> = Vec::with_capacity(1);

            let enemy_type: EnemyType = match enemy_type_nr {
                "1" => EnemyType::Type1(1, 10),
                "2" => EnemyType::Type2(1, 12),
                "3" => EnemyType::Type3(2, 15),
                "4" => EnemyType::Type4(2, 20),
                "5" => EnemyType::Type5(3, 50),
                _ => panic!("Unkown type"),
            };
            let enemy_data = enemy_type.enemy_data();
            Enemy {
                figure_texture: figure_texture,
                enemy_type,
                x_position,
                y_position,
                width: 15.,
                height: 15.,
                animation_state: true,
                is_alive: true,
                firing_limit: 1,
                active_fire_blasts,
                cur_health: enemy_data.0,
                hit_score: enemy_data.1,
            }
        }

        pub fn draw(&mut self) {
            let mut src_rect = Rect { x: 0., y: 0., w: self.width, h: self.height };
            src_rect.y = match self.enemy_type {
                EnemyType::Type1(_, _) => 0.,
                EnemyType::Type2(_, _) => 16.,
                EnemyType::Type3(_, _) => 32.,
                EnemyType::Type4(_, _) => 48.,
                EnemyType::Type5(_, _) => 64.,
            };

            if self.animation_state {
                src_rect.x = 15.;
                src_rect.w = 16.;
            } else {
                src_rect.x = 0.;
                src_rect.w = 15.;
            }

            if !self.is_alive {
                src_rect.x = 30.;
                src_rect.y = 50.;
                src_rect.w = 18.;
            }

            draw_texture_ex(
                &self.figure_texture,
                self.x_position,
                self.y_position,
                color::WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2 { x: 50., y: 50. }),
                    source: Some(src_rect),
                    ..Default::default()
                },
            );
            self.draw_blasts();
        }
        pub fn move_horizontal(&mut self, move_right: bool) {
            if move_right {
                self.x_position += 10.;
            } else {
                self.x_position -= 10.;
            }
        }
        pub fn move_down(&mut self) {
            self.y_position += 40.;
        }

        pub fn kill(&mut self) {
            self.is_alive = false;
        }

        pub fn fire(&mut self) {
            if self.active_fire_blasts.len() < self.firing_limit {
                let blast = FireBlast::create(
                    self.figure_texture.weak_clone(),
                    self.x_position + 26.,
                    self.y_position + 10.,
                );
                self.active_fire_blasts.push(blast);
            }
        }

        fn draw_blasts(&mut self) {
            let mut new_active_fire_blasts = Vec::with_capacity(self.active_fire_blasts.len());
            for mut blast in self.active_fire_blasts.drain(..) {
                if blast.active {
                    blast.fire();
                    new_active_fire_blasts.push(blast);
                }
            }
            self.active_fire_blasts = new_active_fire_blasts;
        }
    }
}
