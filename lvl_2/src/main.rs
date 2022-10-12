use macroquad::prelude::*;

mod settings;
use settings::*;

mod projectile;

mod main_unit;
mod target_unit;
mod scene;
mod utils;
mod assets;

use main_unit::*;
use crate::scene::Scene;
use crate::target_unit::TargetUnit;


#[macroquad::main("breakout")]
async fn main() {
    let mut scene = Scene::new().await;

    loop {
        scene.update().await;
        clear_background(GROUND_COLOR);
        scene.draw();
        next_frame().await
    }
}
