use macroquad::input::{is_key_down, KeyCode};
use macroquad::miniquad::info;
use macroquad::prelude::{mouse_position, screen_height, screen_width, Vec2};
use macroquad::time::get_frame_time;
use quad_url::set_program_parameter;
use crate::{MainUnit, TargetUnit};
use crate::projectile::Projectile;
use crate::assets::Assets;
use crate::order::Order;
use crate::utils::get_parameter_value;
use crate::wall::WallBlock;

pub struct Scene {
    main_unit: MainUnit,
    target_unit: TargetUnit,
    projectiles: Vec<Projectile>,
    wall_block: WallBlock,
    mouse_position: Vec2,
    dt: f32,
    assets: Assets,
    order: Order,
    tick: f32,
}

impl Scene {
    pub async fn new() -> Self {
        let spawn_position = Vec2::new(screen_width() * 0.5, screen_height() * 0.8);
        let target_unit_position = Vec2::new(screen_width() * 0.5, 160.);


        let mouse_position: Vec2 = mouse_position().into();
        let dt = get_frame_time();
        let assets = Assets::new().await;

        Self {
            main_unit: MainUnit::new(
                assets.main_unit_texture,
                spawn_position
            ),
            target_unit: TargetUnit::new(
                assets.target_unit_texture,
                assets.target_unit_shadow_texture,
                assets.target_impact_sound,
                target_unit_position
            ),
            projectiles: Vec::new(),
            wall_block: WallBlock::new(
                assets.wall_block_texture,
                Vec2::new(screen_width() * 0.5, screen_height() * 0.5),
                0.,
            ),
            mouse_position,
            dt,
            assets,
            order: Order::new(),
            tick: 1000.,  // большое число, чтобы сразу срабатывало
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

        if self.main_unit.position.x < 1f32 {
            x_move = 1f32;
        }
        if self.main_unit.position.x > screen_width() {
            x_move = -1f32;
        }

        if self.main_unit.position.y < 1f32 {
            y_move = 1f32;
        }
        if self.main_unit.position.y > screen_height() {
            y_move = -1f32;
        }
        self.order.wasd = Vec2::new(x_move, y_move);
    }

    fn update_order_from_url_query(&mut self) {
        match get_parameter_value("command") == String::from("Shoot") {
            true => {
                self.order.shoot = true;
                set_program_parameter("command", "");
                self.main_unit.shoot_timer = 1.;  // чтобы получить выстрел с минимальной задержкой
                self.main_unit.auto_aim = true;
            }
            false => {}
        }

        match get_parameter_value("rotation").parse::<f32>() {
            Ok(a) => {
                self.order.rotation = a.to_radians();
            }
            Err(e) => {
                info!("{}", e);
            }
        }

    }

    fn set_parameters_to_url_query(&mut self) {
        let line = format!("({}, {})", self.target_unit.position.x as i32, self.target_unit.position.y as i32);
        set_program_parameter("target_pos", line.as_str());
        let line = format!("({}, {})", self.main_unit.position.x as i32, self.main_unit.position.y as i32);
        set_program_parameter("unit_pos", line.as_str());
    }

    pub fn update(&mut self) {
        self.tick += self.dt;
        self.update_order_from_user_input();

        if self.tick >= 1. {
            self.tick = 0.0;
            self.set_parameters_to_url_query();
            self.update_order_from_url_query();
        }
        self.dt = get_frame_time();
        self.target_unit.shift = Vec2::new(0., 0.);
        self.mouse_position = mouse_position().into();

        self.main_unit.update(
            self.dt,
            self.mouse_position,
            &mut self.order,
        );
        if self.order.shoot {
            let position = Vec2::new(  // точка появления выстрела
                self.main_unit.position.x + 65. * (self.main_unit.rotation - f32::to_radians(90.)).cos(),
                self.main_unit.position.y + 65. * (self.main_unit.rotation - f32::to_radians(90.)).sin()
            );

            let projectile = Projectile::new(
                self.assets.projectile_texture,
                self.assets.main_unit_shoot_sound,
                self.main_unit.rotation,
                position,
                self.main_unit.speed * 3.,
            );
            self.projectiles.push(projectile);
        }

        // удаление снарядов на отлете
        self.projectiles.retain(|p|
                ((p.start_position.x - p.position.x).powf(2f32)
                    + (p.start_position.y - p.position.y).powf(2f32)
                    < self.main_unit.shoot_range.powf(2f32)) && p.alive);

        for i in 0..self.projectiles.len() {
            let p = &mut self.projectiles[i];

            if (p.position.x - self.target_unit.position.x).powf(2f32) +
                (p.position.y - self.target_unit.position.y).powf(2f32)
                < self.target_unit.radius.powf(2f32) {
                p.alive = false;
                self.target_unit.update(
                    true,
                    p.rotation,
                );
            }

            p.update(self.dt);
        }
    }

    pub fn draw(&self) {
        self.target_unit.draw_shadow();
        self.wall_block.draw_shadow();
        self.main_unit.draw();
        for i in 0..self.projectiles.len() {
            self.projectiles[i].draw();
        }
        self.target_unit.draw();
        self.wall_block.draw()
    }

}