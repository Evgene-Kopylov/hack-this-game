use std::process::Command;
use macroquad::audio::{load_sound, Sound};
use macroquad::prelude::{
    info,
    load_texture,
    mouse_position,
    screen_height,
    screen_width,
    Texture2D,
    Vec2
};
use macroquad::time::get_frame_time;
use crate::{MainUnit, TargetUnit};
use crate::projectile::Projectile;
use crate::settings::*;

pub struct Scene {
    main_unit: MainUnit,
    target_unit: TargetUnit,
    projectiles: Vec<Projectile>,
    mouse_position: Vec2,
    dt: f32,

}

impl Scene {
    pub async fn new() -> Self {
        info!("WASM LOG: Начало загрузки текстур");
        let main_unit_texture: Texture2D = load_texture(MAIN_UNIT_TEXTURE_PATH).await.unwrap();
        let shoot_sound: Sound = load_sound(MAIN_UNIT_SHOOT_SOUND_ASSET).await.unwrap();
        let target_impact_sound: Sound = load_sound(TARGET_UNIT_IMPACT_SOUND).await.unwrap();
        let spawn_position = (screen_width() * 0.5, screen_height() * 0.8);
        let target_unit_texture = load_texture(TARGET_UNIT_TEXTURE_PATH).await.unwrap();
        let target_unit_shadow_texture = load_texture(TARGET_UNIT_SHADOW_TEXTURE_PATH).await.unwrap();
        let target_unit_position = (screen_width() * 0.5, 160.);
        info!("WASM LOG: Текстуры загружены");

        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();

        Self {
            main_unit: MainUnit::new(
                main_unit_texture,
                shoot_sound,
                target_impact_sound,
                spawn_position
            ),
            target_unit: TargetUnit::new(
                target_unit_texture,
                target_unit_shadow_texture,
                target_unit_position
            ),
            projectiles: Vec::new(),
            mouse_position,
            dt,
        }
    }

    pub async fn update(&mut self) {
        self.dt = get_frame_time();
        self.target_unit.shift = (0., 0.);
        self.mouse_position = mouse_position().into();
        let soot= self.main_unit.update(
            self.dt,
            self.mouse_position,
            self.target_unit.position,
            self.target_unit.texture.width() / 2.
        );
        if soot {
            info!("Shoot");
            let position = (  // точка появления выстрела
                self.main_unit.position.0 + 65. * (self.main_unit.rotation - f32::to_radians(90.)).cos(),
                self.main_unit.position.1 + 65. * (self.main_unit.rotation - f32::to_radians(90.)).sin()
            );

            let projectile = Projectile::new(
                self.main_unit.rotation,
                position,
                self.main_unit.speed * 3.,
            ).await;
            self.projectiles.push(projectile);
        }

        for i in 0..self.projectiles.len() {
            self.projectiles[i].update(self.dt);
        }

    }

    pub fn draw(&self) {
        self.target_unit.draw_shadow();
        self.main_unit.draw();
        for i in 0..self.projectiles.len() {
            self.projectiles[i].draw();
        }
        self.target_unit.draw();
    }

}