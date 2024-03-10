pub mod Firing {
    use macroquad::color;
    use macroquad::input;
    use macroquad::math::*;
    use macroquad::shapes;
    use macroquad::text;
    use macroquad::texture::*;
    use macroquad::window::*;

    pub struct FireBlast {
        texture: Texture2D,
        pub active: bool,
        pub x_position: f32,
        pub y_position: f32,
    }

    impl FireBlast {
        pub fn create(texture: Texture2D, x_position: f32, y_position: f32) -> FireBlast {
            FireBlast {
                texture,
                active: true,
                x_position,
                y_position,
            }
        }

        pub fn fire(&mut self) {
            if self.y_position > 0. {
                draw_texture_ex(
                    &self.texture,
                    self.x_position,
                    self.y_position,
                    color::RED,
                    DrawTextureParams {
                        dest_size: Some(Vec2 { x: 40., y: 70. }),
                        source: Some(Rect {
                            x: 39.,
                            y: 0.,
                            w: 5.,
                            h: 15.,
                        }),
                        ..Default::default()
                    },
                );
                self.y_position -= 10.;
            } else {
                self.active = false;
            }
        }
    }
}
