use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::info;
use macroquad::texture::{load_texture, Texture2D};


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
        let main_unit_texture: Texture2D = load_texture(
            "../assets/pointer/pointer_3.png").await.unwrap();
        let main_unit_shoot_sound: Sound = load_sound(
            "../assets/sound/4 XSA_Weapon.wav").await.unwrap();
        let target_impact_sound: Sound = load_sound(
            "../assets/sound/hit-with-something.wav").await.unwrap();
        let target_unit_texture = load_texture(
            "../assets/pointer/target_unit_3_2.png").await.unwrap();
        let target_unit_shadow_texture = load_texture(
            "../assets/pointer/target_unit_3_shadow.png").await.unwrap();
        let projectile_texture = load_texture(
            "../assets/pointer/projectile_glow_7.png").await.unwrap();
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