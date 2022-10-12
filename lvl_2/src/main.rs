use macroquad::prelude::*;

mod settings;
use settings::*;

mod projectile;

mod main_unit;
mod target_unit;
mod scene;
mod utils;

use main_unit::*;
use crate::scene::Scene;
use crate::target_unit::TargetUnit;


#[macroquad::main("breakout")]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        let mouse_position: Vec2 = mouse_position().into();
        scene.update(get_frame_time(), mouse_position);
        clear_background(GROUND_COLOR);
        scene.draw();
        next_frame().await
    }
}
