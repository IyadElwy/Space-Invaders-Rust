pub mod ShipMod {
    use crate::firing::Firing::FireBlast;
    use macroquad::color;
    use macroquad::input;
    use macroquad::math::*;
    use macroquad::shapes;
    use macroquad::text;
    use macroquad::texture::*;
    use macroquad::window::*;

    pub struct Ship {
        figure_texture: Texture2D,
        x_position: f32,
        firing_limit: usize,
        pub active_fire_blasts: Vec<FireBlast>,
    }

    impl Ship {
        pub fn create(figure_texture: Texture2D, firing_limit: usize) -> Ship {
            let active_fire_blasts: Vec<FireBlast> = Vec::with_capacity(firing_limit);

            Ship {
                figure_texture: figure_texture,
                x_position: screen_width() / 2.,
                firing_limit,
                active_fire_blasts,
            }
        }

        pub fn draw(&mut self) {
            draw_texture_ex(
                &self.figure_texture,
                self.x_position,
                screen_height() - 50.,
                color::GREEN,
                DrawTextureParams {
                    dest_size: Some(Vec2 { x: 50., y: 50. }),
                    source: Some(Rect { x: 68., y: 0., w: 15., h: 15. }),
                    ..Default::default()
                },
            );
            self.draw_blasts();
        }

        pub fn move_right(&mut self) {
            self.x_position = ((640. - 35.) as f32).min(self.x_position + 12.);
        }
        pub fn move_left(&mut self) {
            self.x_position = (10. as f32).max(self.x_position - 12.);
        }

        pub fn fire(&mut self) {
            if self.active_fire_blasts.len() < self.firing_limit {
                let blast = FireBlast::create(
                    self.figure_texture.weak_clone(),
                    self.x_position + 11.,
                    screen_height() - 88.0,
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
