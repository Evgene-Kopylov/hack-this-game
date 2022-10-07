use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::*;

mod settings;
use settings::*;

mod projectile;

mod main_unit;
mod target_unit;
mod scene;

use main_unit::*;
use crate::scene::Scene;
use crate::target_unit::TargetUnit;


#[macroquad::main("breakout")]
async fn main() {
    let main_unit_texture: Texture2D = load_texture(MAIN_UNIT_TEXTURE_PATH).await.unwrap();
    let projectile_texture = load_texture(PROJECTILE_TEXTURE_PATH).await.unwrap();
    let shoot_sound: Sound = load_sound(MAIN_UNIT_SHOOT_SOUND_ASSET).await.unwrap();
    let target_impact_sound: Sound = load_sound(TARGET_UNIT_IMPACT_SOUND).await.unwrap();
    let spawn_position = (screen_width() * 0.5, screen_height() * 0.8);
    let target_unit_texture = load_texture(TARGET_UNIT_TEXTURE_PATH).await.unwrap();
    let target_unit_shadow_texture = load_texture(TARGET_UNIT_SHADOW_TEXTURE_PATH).await.unwrap();
    let target_unit_position = (screen_width() * 0.5, 160.);

    let mut scene: Scene = Scene::new(
        // main_unit
        main_unit_texture,
        projectile_texture,
        shoot_sound,
        target_impact_sound,
        spawn_position,
        //target_unit
        target_unit_texture,
        target_unit_shadow_texture,
        target_unit_position,
    );

    loop {
        let mouse_position: Vec2 = mouse_position().into();
        scene.update(get_frame_time(), mouse_position);
        clear_background(GROUND_COLOR);
        scene.draw();
        next_frame().await
    }
}
