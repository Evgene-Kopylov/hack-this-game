use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::{load_texture, screen_height, screen_width, Texture2D, Vec2};
use crate::{MainUnit, TargetUnit};
use crate::settings::*;

pub struct Scene {
    main_unit: MainUnit,
    target_unit: TargetUnit,
}

impl Scene {
    pub async fn new() -> Self {
        // info!("WASM LOG: Начало загрузки текстур");
        let main_unit_texture: Texture2D = load_texture(MAIN_UNIT_TEXTURE_PATH).await.unwrap();
        let projectile_texture = load_texture(PROJECTILE_TEXTURE_PATH).await.unwrap();
        let shoot_sound: Sound = load_sound(MAIN_UNIT_SHOOT_SOUND_ASSET).await.unwrap();
        let target_impact_sound: Sound = load_sound(TARGET_UNIT_IMPACT_SOUND).await.unwrap();
        let spawn_position = (screen_width() * 0.5, screen_height() * 0.8);
        let target_unit_texture = load_texture(TARGET_UNIT_TEXTURE_PATH).await.unwrap();
        let target_unit_shadow_texture = load_texture(TARGET_UNIT_SHADOW_TEXTURE_PATH).await.unwrap();
        let target_unit_position = (screen_width() * 0.5, 160.);
        // info!("WASM LOG: Текстуры загружены");

        Self {
            main_unit: MainUnit::new(
                main_unit_texture,
                projectile_texture,
                shoot_sound,
                target_impact_sound,
                spawn_position
            ),
            target_unit: TargetUnit::new(
                target_unit_texture,
                target_unit_shadow_texture,
                target_unit_position
            )
        }
    }

    pub fn update(&mut self, dt: f32, mouse_position: Vec2) {
        self.target_unit.shift = (0., 0.);
        let (target_impact, impact_angle) = self.main_unit.update(
            dt,
            mouse_position,
            self.target_unit.position,
            self.target_unit.texture.width() / 2.
        );
        if target_impact {
            let shift = 5.;
            self.target_unit.shift = (shift * impact_angle.sin(), shift * impact_angle.cos());
        }
    }

    pub fn draw(&self) {
        self.target_unit.draw_shadow();
        self.main_unit.draw();
        self.target_unit.draw();
    }

}