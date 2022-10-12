use macroquad::audio::Sound;
use macroquad::prelude::{Texture2D, Vec2};
use crate::{MainUnit, TargetUnit};

pub struct Scene {
    main_unit: MainUnit,
    target_unit: TargetUnit,
}

impl Scene {
    pub fn new(
        main_unit_texture: Texture2D,
        projectile_texture: Texture2D,
        shoot_sound: Sound,
        target_impact_sound: Sound,
        spawn_position: (f32, f32),

        target_unit_texture: Texture2D,
        target_unit_shadow_texture: Texture2D,
        target_unit_position: (f32, f32),
    ) -> Self {
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