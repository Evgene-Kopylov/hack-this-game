use macroquad::input::{is_key_down, KeyCode};
use macroquad::prelude::{mouse_position, screen_height, screen_width, Vec2};
use macroquad::time::get_frame_time;
use crate::{MainUnit, TargetUnit};
use crate::projectile::Projectile;
use crate::assets::Assets;
use crate::order::Order;

pub struct Scene {
    main_unit: MainUnit,
    target_unit: TargetUnit,
    projectiles: Vec<Projectile>,
    mouse_position: Vec2,
    dt: f32,
    assets: Assets,
    order: Order,
}

impl Scene {
    pub async fn new() -> Self {
        let spawn_position = (screen_width() * 0.5, screen_height() * 0.8);
        let target_unit_position = (screen_width() * 0.5, 160.);


        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();
        let assets = Assets::new().await;

        Self {
            main_unit: MainUnit::new(
                assets.main_unit_texture,
                assets.main_unit_shoot_sound,
                spawn_position
            ),
            target_unit: TargetUnit::new(
                assets.target_unit_texture,
                assets.target_unit_shadow_texture,
                assets.target_impact_sound,
                target_unit_position
            ),
            projectiles: Vec::new(),
            mouse_position,
            dt,
            assets,
            // wasd: Vec2::new(0., 0.),
            order: Order::new(),
        }
    }


    /// Зафиксировать активность пользователя.
    fn update_order_from_user_input(&mut self) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D){
            x_move += 1f32;
        }

        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up) || is_key_down(KeyCode::W) {
            y_move -= 1f32;
        }
        if is_key_down(KeyCode::Down) || is_key_down(KeyCode::S) {
            y_move += 1f32;
        }

        if self.main_unit.position.0 < 1f32 {
            x_move = 1f32;
        }
        if self.main_unit.position.0 > screen_width() {
            x_move = -1f32;
        }

        if self.main_unit.position.1 < 1f32 {
            y_move = 1f32;
        }
        if self.main_unit.position.1 > screen_height() {
            y_move = -1f32;
        }
        self.order.wasd = Vec2::new(x_move, y_move);
    }

    pub fn update_order_from_url_query(&mut self) {

    }

    pub fn update(&mut self) {
        self.update_order_from_user_input();

        self.update_order_from_url_query();

        self.dt = get_frame_time();
        self.target_unit.shift = (0., 0.);
        self.mouse_position = mouse_position().into();


        let shoot= self.main_unit.update(
            self.dt,
            self.mouse_position,
            &self.order,
        );
        if shoot {
            let position = (  // точка появления выстрела
                self.main_unit.position.0 + 65. * (self.main_unit.rotation - f32::to_radians(90.)).cos(),
                self.main_unit.position.1 + 65. * (self.main_unit.rotation - f32::to_radians(90.)).sin()
            );

            let projectile = Projectile::new(
                self.assets.projectile_texture,
                self.main_unit.rotation,
                position,
                self.main_unit.speed * 3.,
            );
            self.projectiles.push(projectile);
        }

        // // удаление снарядов на отлете
        self.projectiles.retain(|p|
                ((p.start_position.0 - p.position.0).powf(2f32)
                    + (p.start_position.1 - p.position.1).powf(2f32)
                    < self.main_unit.shoot_range.powf(2f32)) && p.alive);

        for i in 0..self.projectiles.len() {
            if (self.projectiles[i].position.0 - self.target_unit.position.0).powf(2f32) +
                (self.projectiles[i].position.1 - self.target_unit.position.1).powf(2f32)
                < self.target_unit.radius.powf(2f32) {
                self.projectiles[i].alive = false;
                self.target_unit.update(
                    true,
                    self.projectiles[i].rotation,
                );
            }

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