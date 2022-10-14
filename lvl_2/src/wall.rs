use macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};
use crate::{PROJECTILE_COLOR, Vec2, Vec4, WALL_BLOCK_COLOR};

pub struct WallBlock {
    texture: Texture2D,
    position: Vec2,
    rotation: f32,
    size: Vec2,
}

impl WallBlock {
    pub fn new(
        texture: Texture2D,
        position: Vec2,
        rotation: f32,
    ) -> Self {
        Self {
            texture,
            position,
            rotation,
            size: Vec2::new(texture.width(), texture.height()),
        }
    }

    pub fn draw(&self) {
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
