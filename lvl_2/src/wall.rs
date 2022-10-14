use macroquad::color::{BLACK, Color, DARKGRAY};
use macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};
use crate::{PROJECTILE_COLOR, Vec2, Vec4, WALL_BLOCK_COLOR};

pub struct WallBlock {
    texture: Texture2D,
    position: Vec2,
    rotation: f32,
    size: Vec2,
    shadow_color: Color,
}

impl WallBlock {
    pub fn new(
        texture: Texture2D,
        position: Vec2,
        rotation: f32,
    ) -> Self {
        let mut shadow_color = BLACK;
        shadow_color.a = 0.45;

        Self {
            texture,
            position,
            rotation,
            size: Vec2::new(texture.width(), texture.height()),
            shadow_color,
        }
    }

    pub fn draw_shadow(&self) {
        // Тень
        let height = 2.3;
        draw_texture_ex(
            self.texture,
            self.position.x - self.texture.width() * 0.5 + 3. * height,
            self.position.y - self.texture.height() * 0.5 + 4. * height,
            self.shadow_color,
            DrawTextureParams {
                ..Default::default()
            }
        );
    }

    pub fn draw(&self) {
        // object
        draw_texture_ex(
            self.texture,
            self.position.x - self.size.x * 0.50,
            self.position.y - self.size.y * 0.50,
            WALL_BLOCK_COLOR,
            DrawTextureParams {
                // dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
    }
}
