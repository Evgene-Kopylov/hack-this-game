use macroquad::prelude::Color;


pub const GROUND_COLOR: Color = Color::new(0.77, 0.8, 0.8, 1.00);
pub const UNIT_COLOR: Color = Color::new(0.94, 0.94, 0.94, 1.);
pub const PROJECTILE_TEXTURE_PATH: &str = "../assets/pointer/projectile_glow_7.png";
pub const PROJECTILE_COLOR: Color = Color::new(1.00, 0.96, 0.84, 1.00);
pub const MAIN_UNIT_SPEED: f32 = 300.;
pub const MAIN_UNIT_SHOOT_DELAY: f32 = 0.1;
pub const MAIN_UNIT_SHOOT_RANGE: f32 = 3000.;
pub const MAIN_UNIT_SHOOT_SOUND_VOLUME: f32 = 0.16;
// pub const MAIN_UNIT_SHOOT_SOUND_ASSET: &str = "../assets/sound/XAL_Weapon.wav";
pub const MAIN_UNIT_SHOOT_SOUND_ASSET: &str = "../assets/sound/4 XSA_Weapon.wav";
pub const MAIN_UNIT_TEXTURE_PATH: &str = "../assets/pointer/pointer_3.png";
pub const TARGET_UNIT_TEXTURE_PATH: &str = "../assets/pointer/target_unit_3_2.png";
pub const TARGET_UNIT_SHADOW_TEXTURE_PATH: &str = "../assets/pointer/target_unit_3_shadow.png";
// pub const TARGET_UNIT_IMPACT_SOUND: &str = "../assets/sound/4 Interface.wav";
pub const TARGET_UNIT_IMPACT_SOUND: &str = "../assets/sound/hit-with-something.wav";
