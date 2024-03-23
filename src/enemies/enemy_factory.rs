pub mod EnemyFactoryMod {

    use crate::enemies::enemy::EnemyMod::Enemy;
    use crate::firing::Firing::FireBlast;
    use crate::load_level_settings::LoadLevelSettings::LevelData;
    use macroquad::math::*;
    use macroquad::texture::*;
    use macroquad::time::get_frame_time;
    use rand::Rng;
    use std::collections::HashMap;

    pub struct EnemyFactory {
        figure_texture: Texture2D,
        pub enemies: Vec<Enemy>,
        move_timer: f32,
        move_interval: f32,
        move_right: bool,
        level_data: HashMap<String, LevelData>,
        cur_level: u8,
    }

    impl EnemyFactory {
        pub fn create(
            figure_texture: Texture2D,
            level_data: HashMap<String, LevelData>,
        ) -> EnemyFactory {
            EnemyFactory {
                figure_texture,
                enemies: Vec::new(),
                move_timer: 0.0,
                move_interval: 1.5,
                move_right: true,
                level_data,
                cur_level: 0,
            }
        }

        pub fn create_wave(&mut self, level: &mut u8, restart: bool) {
            if self.enemies.len() == 0 || restart {
                *level = *level + 1;
                self.cur_level = *level;
                let curr_level: &LevelData = self.level_data.get(&level.to_string()).unwrap();
                self.move_interval = curr_level.speed.parse::<f32>().unwrap();

                let mut last_x_pos: f32 = 0.;
                let mut last_y_pos: f32 = 0.;
                let mut enemies: Vec<Enemy> = Vec::with_capacity(55);
                for (enemy_type_nr, enemy_count) in curr_level.types.iter() {
                    let enemy_type_nr = enemy_type_nr.as_str();
                    let enemy_count = enemy_count.parse::<i32>().unwrap();
                    (0..enemy_count).into_iter().for_each(|i| {
                        last_x_pos += 50.;
                        if ((enemies.len() as i32) + i) % 11 == 0 {
                            last_y_pos += 40.;
                            last_x_pos = 0.;
                        }
                        enemies.push(Enemy::create(
                            self.figure_texture.weak_clone(),
                            enemy_type_nr,
                            last_x_pos,
                            last_y_pos,
                        ));
                    });
                }
                self.enemies = enemies;
            }
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
                        enemy.move_horizontal(self.move_right);
                        enemy.animation_state = !enemy.animation_state;
                    }
                }
                self.enemies = self.enemies.drain(..).filter(|e| e.is_alive).collect();

                if self.enemies.len() > 0 {
                    let curr_level: &LevelData =
                        self.level_data.get(&self.cur_level.to_string()).unwrap();
                    let number_of_fires = curr_level.fires.parse::<i32>().unwrap();
                    let mut rng = rand::thread_rng();
                    (0..number_of_fires).into_iter().for_each(|_| {
                        let firing_enemy_index = rng.gen_range(0..self.enemies.len());
                        self.enemies.get_mut(firing_enemy_index).unwrap().fire();
                    });
                }
                self.move_timer = 0.0;
            }
        }

        pub fn detect_enemy_collision(
            &mut self,
            incoming_fire_blasts: &mut Vec<FireBlast>,
            score: &mut u32,
        ) {
            let len_enemies = self.enemies.len();
            for b in incoming_fire_blasts.iter_mut() {
                for e in self.enemies.iter_mut() {
                    if b.x_position < e.x_position + e.width + 23.
                        && b.x_position + b.width > e.x_position
                        && b.y_position < e.y_position + e.height
                        && b.y_position + b.height > e.y_position
                    {
                        *score = *score + e.hit_score;
                        if e.cur_health >= 1 {
                            e.cur_health -= 1;
                        }
                        if e.cur_health == 0 {
                            e.kill();
                        }
                        b.deactivate();
                        if len_enemies == 5 && self.move_interval >= 0.8 {
                            self.move_interval -= 0.4;
                        }
                    }
                }
            }
        }
    }
}
