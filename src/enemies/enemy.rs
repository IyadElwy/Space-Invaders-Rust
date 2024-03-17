pub mod EnemyMod {
    use macroquad::color;
    use macroquad::input;
    use macroquad::math::*;
    use macroquad::shapes;
    use macroquad::text;
    use macroquad::texture::*;
    use macroquad::window::*;

    pub struct Enemy {
        figure_texture: Texture2D,
        pub x_position: f32,
        pub y_position: f32,
        pub animation_state: bool,
    }

    impl Enemy {
        pub fn create(figure_texture: Texture2D, x_position: f32, y_position: f32) -> Enemy {
            Enemy { figure_texture: figure_texture, x_position, y_position, animation_state: true }
        }

        pub fn draw(&mut self) {
            let mut src_rect = Rect { x: 0., y: 0., w: 15., h: 15. };
            if self.animation_state {
                src_rect.x = 15.;
                src_rect.w = 16.;
            } else {
                src_rect.x = 0.;
                src_rect.w = 15.;
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
    }
}
