pub mod firing {
    use macroquad::color;
    use macroquad::math::*;
    use macroquad::texture::*;

    pub struct FireBlast {
        texture: Texture2D,
        pub active: bool,
        pub x_position: f32,
        pub y_position: f32,
        pub width: f32,
        pub height: f32,
    }

    impl FireBlast {
        pub fn create(texture: Texture2D, x_position: f32, y_position: f32) -> FireBlast {
            FireBlast { texture, active: true, x_position, y_position, width: 5., height: 15. }
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
                        source: Some(Rect { x: 39., y: 0., w: self.width, h: self.height }),
                        ..Default::default()
                    },
                );
                self.y_position -= 10.;
            } else {
                self.active = false;
            }
        }

        pub fn deactivate(&mut self) {
            self.active = false;
        }
    }
}
