use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{Color, draw_texture_ex, DrawTextureParams, Texture2D};
use crate::{MAIN_UNIT_SHOOT_SOUND_VOLUME, Vec2};


pub struct TargetUnit {
    pub texture: Texture2D,
    pub shadow_texture: Texture2D,
    color: Color,
    pub position: Vec2,
    pub radius: f32,
    pub shift: Vec2,
    impact_sound: Sound,
}

impl TargetUnit {
    pub fn new(texture: Texture2D, shadow_texture: Texture2D, impact_sound: Sound, spawn_position: Vec2) -> Self {
        let mut color = BLACK;
        color.a = 0.45;

        Self {
            texture,
            shadow_texture,
            color,
            position: spawn_position,
            radius: texture.width() * 0.5,
            shift: Vec2::new(0., 0.),
            impact_sound
        }
    }

    pub fn update(&mut self, impact: bool, impact_angle: f32) {
        if impact {
            let shift = 5.;
            self.shift = Vec2::new(shift * impact_angle.sin(), shift * impact_angle.cos());

            let mut sound_params: PlaySoundParams = PlaySoundParams::default();
            sound_params.volume = MAIN_UNIT_SHOOT_SOUND_VOLUME * 0.35;
            audio::play_sound(self.impact_sound, sound_params);

        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.x - self.texture.width() * 0.5 + self.shift.x,
            self.position.y - self.texture.height() * 0.5 - self.shift.y,
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
            self.position.x - self.texture.width() * 0.5 + 3. * height,
            self.position.y - self.texture.height() * 0.5 + 4. * height,
            self.color,
            DrawTextureParams {
                ..Default::default()
            }
        );
    }

}