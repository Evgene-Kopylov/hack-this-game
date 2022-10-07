use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{Color, draw_texture_ex, DrawTextureParams, Texture2D};


pub struct TargetUnit {
    pub texture: Texture2D,
    pub shadow_texture: Texture2D,
    color: Color,
    pub position: (f32, f32),
    pub shift: (f32, f32),
}

impl TargetUnit {
    pub fn new(texture: Texture2D, shadow_texture: Texture2D, spawn_position: (f32, f32)) -> Self {
        let mut color = BLACK;
        color.a = 0.45;
        Self {
            texture,
            shadow_texture,
            color,
            position: spawn_position,
            shift: (0., 0.),
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.0 - self.texture.width() * 0.5 + self.shift.0,
            self.position.1 - self.texture.height() * 0.5 - self.shift.1,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            }
        );
    }

    pub fn draw_shadow(&self) {
        // тень
        let height = 3.;
        draw_texture_ex(
            self.shadow_texture,
            self.position.0 - self.texture.width() * 0.5 + 3. * height,
            self.position.1 - self.texture.height() * 0.5 + 4. * height,
            self.color,
            DrawTextureParams {
                ..Default::default()
            }
        );
    }

}