use macroquad::audio;
use macroquad::audio::{PlaySoundParams, Sound};
use macroquad::prelude::*;
use crate::settings::*;

pub struct Projectile {
    pub texture: Texture2D,
    pub rotation: f32,
    pub start_position: Vec2,
    pub position: Vec2,
    pub size: Vec2,
    pub speed: f32,
    pub alive: bool,

}


impl Projectile {
    pub fn new(
        texture: Texture2D,
        shoot_sound: Sound,
        rotation: f32,
        position: Vec2,
        speed: f32
    ) -> Self {

        let mut sound_params: PlaySoundParams = PlaySoundParams::default();
        sound_params.volume = MAIN_UNIT_SHOOT_SOUND_VOLUME;
        audio::play_sound(shoot_sound, sound_params);

        let size = Vec2::new(texture.width(), texture.height());
        Self {
            texture,
            rotation,
            start_position: position,
            position,
            size,
            speed,
            alive: true,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.position.x += dt * self.speed * (self.rotation - f32::to_radians(90.)).cos();
        self.position.y += dt * self.speed * (self.rotation - f32::to_radians(90.)).sin();
    }
    
    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.x - self.size.x * 0.50,
            self.position.y - self.size.y * 0.50,
            PROJECTILE_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
    }
}