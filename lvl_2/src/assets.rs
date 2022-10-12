use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::info;
use macroquad::texture::{load_texture, Texture2D};
use crate::settings::*;

pub(crate) struct Assets {
    pub(crate) main_unit_texture: Texture2D,
    pub(crate) main_unit_shoot_sound: Sound,
    pub(crate) target_impact_sound: Sound,
    pub(crate) target_unit_texture: Texture2D,
    pub(crate) target_unit_shadow_texture: Texture2D,
    pub(crate) projectile_texture: Texture2D,
}

impl Assets {
    pub async fn new() -> Self {
        info!("WASM LOG: Начало загрузки текстур");
        let main_unit_texture: Texture2D = load_texture(MAIN_UNIT_TEXTURE_PATH).await.unwrap();
        let main_unit_shoot_sound: Sound = load_sound(MAIN_UNIT_SHOOT_SOUND_ASSET).await.unwrap();
        let target_impact_sound: Sound = load_sound(TARGET_UNIT_IMPACT_SOUND).await.unwrap();
        let target_unit_texture = load_texture(TARGET_UNIT_TEXTURE_PATH).await.unwrap();
        let target_unit_shadow_texture = load_texture(TARGET_UNIT_SHADOW_TEXTURE_PATH).await.unwrap();
        let projectile_texture = load_texture(PROJECTILE_TEXTURE_PATH).await.unwrap();
        info!("WASM LOG: Текстуры загружены");

        Self {
            main_unit_texture,
            main_unit_shoot_sound,
            target_impact_sound,
            target_unit_texture,
            target_unit_shadow_texture,
            projectile_texture,
        }
    }
}