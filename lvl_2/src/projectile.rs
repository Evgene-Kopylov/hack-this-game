use macroquad::prelude::*;
use crate::settings::*;

pub struct Projectile {
    pub texture: Texture2D,
    pub rotation: f32,
    pub start_position: (f32, f32),
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub speed: f32,
    pub alive: bool,

}


impl Projectile {
    pub async fn new(
        rotation: f32,
        position: (f32, f32),
        speed: f32
    ) -> Self {
        let texture = load_texture(PROJECTILE_TEXTURE_PATH).await.unwrap();

        let size = (texture.width(), texture.height());
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
        self.position.0 += dt * self.speed * (self.rotation - f32::to_radians(90.)).cos();
        self.position.1 += dt * self.speed * (self.rotation - f32::to_radians(90.)).sin();
    }
    
    pub fn draw(&self) {
        draw_texture_ex(
            self.texture,
            self.position.0 - self.size.0 * 0.50,
            self.position.1 - self.size.1 * 0.50,
            PROJECTILE_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size .0, self.size.1)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
    }
}