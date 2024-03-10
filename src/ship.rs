pub mod Ship {
    use macroquad::color;
    use macroquad::input;
    use macroquad::math::*;
    use macroquad::shapes;
    use macroquad::text;
    use macroquad::texture::*;
    use macroquad::window::*;

    pub struct Ship {
        figure_texture: Texture2D,
        pub x_position: f32,
    }

    impl Ship {
        pub fn create(figure_texture: Texture2D) -> Ship {
            Ship {
                figure_texture: figure_texture,
                x_position: screen_width() / 2.,
            }
        }

        pub fn draw(&self) {
            draw_texture_ex(
                &self.figure_texture,
                self.x_position,
                screen_height() - 50.,
                color::BLUE,
                DrawTextureParams {
                    dest_size: Some(Vec2 { x: 50., y: 50. }),
                    source: Some(Rect {
                        x: 68.,
                        y: 0.,
                        w: 15.,
                        h: 15.,
                    }),
                    ..Default::default()
                },
            );
        }

        pub fn move_right(&mut self) {
            self.x_position = (screen_width() - 35.).min(self.x_position + 12.);
        }
        pub fn move_left(&mut self) {
            self.x_position = (10. as f32).max(self.x_position - 12.);
        }
    }
}
