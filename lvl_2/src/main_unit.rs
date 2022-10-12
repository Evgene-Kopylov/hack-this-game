use macroquad::prelude::*;
use crate::order::Order;
use crate::settings::*;



pub struct MainUnit {
    pub texture: Texture2D,
    pub size: (f32, f32),
    pub scale: f32,
    pub radius: f32,
    pub rotation: f32,
    pub position: (f32, f32),
    pub speed: f32,
    pub shoot_timer: f32,
    shoot_delay: f32,
    pub shoot_range: f32,
    auto_aim: bool,
}


impl MainUnit {
    pub fn new(
        texture: Texture2D,
        position: (f32, f32),
    ) -> Self {
        Self {
            texture,
            position,
            size: (texture.width(), texture.height()),
            scale: 1.,
            radius: f32::max(texture.width(), texture.height()),
            rotation: 0.,
            speed: MAIN_UNIT_SPEED,
            shoot_timer: 0.,
            shoot_delay: MAIN_UNIT_SHOOT_DELAY,
            shoot_range: MAIN_UNIT_SHOOT_RANGE,
            auto_aim: false,
        }
    }

    // Возвращает сигнал о попадании в цель
    pub fn update(&mut self, dt: f32, mouse_position: Vec2, order: &mut Order) {
        self.shoot_timer += dt;

        self.position.0 += order.wasd.x * dt * self.speed;
        self.position.1 += order.wasd.y * dt * self.speed;

        if order.wasd.x != 0. || order.wasd.y != 0. || is_mouse_button_down(MouseButton::Left) {
            self.auto_aim = false;
        }

        // поворот в сторону курсора
        self.rotation = self.rotation % f32::to_radians(360.);
        let mut dx = self.position.0 - mouse_position.x;
        if dx == 0f32 { dx += 1f32; };

        let mut dy = self.position.1 - mouse_position.y;
        if dy == 0f32 { dy += 1f32; };

        if order.shoot && !(is_mouse_button_down(MouseButton::Left)) {
            self.rotation = order.rotation;
            self.auto_aim = true;
        } else {
            if !self.auto_aim {
                if dx >= 0f32 {
                    self.rotation = (dy / dx).atan() - f32::to_radians(90.);
                } else {
                    self.rotation = (dy / dx).atan() - f32::to_radians(270.);
                }
            }
        }

        if (is_mouse_button_down(MouseButton::Left) || order.shoot)
            && self.shoot_timer >= self.shoot_delay {
            self.shoot_timer = 0.;
            order.shoot = true;
        } else {
            order.shoot = false;
        }
    }

    pub fn draw(&self) {
        // тень
        draw_texture_ex(
            self.texture,
            self.position.0 - self.size.0 * 0.5 + 3.,
            self.position.1 - self.size.1 * 0.5 + 4.,
            DARKGRAY,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size .0, self.size.1)),
                rotation: self.rotation,
                ..Default::default()
            }
        );
        // сам юнит
        draw_texture_ex(
            self.texture,
            self.position.0 - self.size.0 * 0.5,
            self.position.1 - self.size.1 * 0.5,
            UNIT_COLOR,
            DrawTextureParams {
                dest_size: Some(Vec2::new(self.size .0, self.size.1)),
                rotation: self.rotation,
                ..Default::default()
            }
        );

    }
}
