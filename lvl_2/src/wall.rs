use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::color::{BLACK, Color};
use macroquad::texture::{draw_texture_ex, DrawTextureParams, Texture2D};
use crate::{Vec2, WALL_BLOCK_COLOR, WALL_BLOCK_IMPACT_SOUND_VOLUME};

pub struct WallBlock {
    texture: Texture2D,
    pub(crate) position: Vec2,
    rotation: f32,
    pub(crate) size: Vec2,
    shadow_color: Color,
    impact_sound: Sound,
}

impl WallBlock {
    pub fn new(
        texture: Texture2D,
        impact_sound: Sound,
        position: Vec2,
        rotation: f32,
    ) -> Self {
        let mut shadow_color = BLACK;
        shadow_color.a = 0.45;

        Self {
            texture,
            impact_sound,
            position,
            rotation,
            size: Vec2::new(texture.width(), texture.height()),
            shadow_color,
        }
    }

    pub fn update(&mut self, impact: bool) {
        if impact {
            let mut sound_params: PlaySoundParams = PlaySoundParams::default();
            sound_params.volume = WALL_BLOCK_IMPACT_SOUND_VOLUME * 0.35;
            audio::play_sound(self.impact_sound, sound_params);
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
