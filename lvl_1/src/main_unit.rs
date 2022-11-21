use macroquad::prelude::*;
use crate::order::Order;
use crate::settings::*;



pub struct MainUnit {
    pub texture: Texture2D,
    pub size: Vec2,
    pub scale: f32,
    pub radius: f32,
    pub rotation: f32,
    pub position: Vec2,
    pub speed: f32,
    pub shoot_timer: f32,
    shoot_delay: f32,
    pub shoot_range: f32,
    pub auto_aim: bool,
    bullet_load: u8,
}


impl MainUnit {
    pub fn new(
        texture: Texture2D,
        position: Vec2,
    ) -> Self {
        Self {
            texture,
            position,
            size: Vec2::new(texture.width(), texture.height()),
            scale: 1.,
            radius: f32::max(texture.width(), texture.height()),
            rotation: 0.,
            speed: MAIN_UNIT_SPEED,
            shoot_timer: 0.,
            shoot_delay: MAIN_UNIT_SHOOT_DELAY,
            shoot_range: MAIN_UNIT_SHOOT_RANGE,
            auto_aim: false,
            bullet_load: 0,
        }
    }

    // Возвращает сигнал о попадании в цель
    pub fn update(&mut self, dt: f32, mouse_position: Vec2, order: &mut Order) {
        self.shoot_timer += dt;

        self.position.x += order.wasd.x * dt * self.speed;
        self.position.y += order.wasd.y * dt * self.speed;

        if order.wasd.x != 0. || order.wasd.y != 0. || is_mouse_button_down(MouseButton::Left) {
            self.auto_aim = false;
        }

        // поворот в сторону курсора
        self.rotation = self.rotation % f32::to_radians(360.);
        let mut dx = self.position.x - mouse_position.x;
        if dx == 0f32 { dx += 1f32; };

        let mut dy = self.position.y - mouse_position.y;
        if dy == 0f32 { dy += 1f32; };

        if self.auto_aim {
            self.rotation = order.rotation;
        } else {
            if !self.auto_aim {
                if dx >= 0f32 {
                    self.rotation = (dy / dx).atan() - f32::to_radians(90.);
                } else {
                    self.rotation = (dy / dx).atan() - f32::to_radians(270.);
                }
            }
        }

        // Управление огнем
        if self.shoot_timer >= self.shoot_delay {
            if is_mouse_button_down(MouseButton::Left) {  // ЛКМ
                order.shoot = true;
                self.bullet_load = 0;

            } else if order.shoot { // команда
                order.shoot = true;
                self.bullet_load = 0;  // выстрелов на 1 больше

            } else if self.bullet_load > 0 { // очередь
                order.shoot = true;
                self.bullet_load -= 1;
            }

        } else {
            order.shoot = false;
        }

        if order.shoot {
            self.shoot_timer = 0.;
        }
    }

    pub fn draw(&self) {
        // тень
        draw_texture_ex(
            self.texture,
            self.position.x - self.size.x * 0.5 + 3.,
            self.position.y - self.size.y * 0.5 + 4.,
            DARKGRAY,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
        // сам юнит
        draw_texture_ex(
            self.texture,
            self.position.x - self.size.x * 0.5,
            self.position.y - self.size.y * 0.5,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size.x, self.size.y)),
                rotation: self.rotation,
                ..Default::default()
            }
        );

    }
}
