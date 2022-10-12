use macroquad::math::Vec2;



pub struct Order {
    pub shoot: bool,
    pub wasd: Vec2,  // смещение
}

impl Order {
    pub fn new() -> Self {
        Self {
            shoot: false,
            wasd: Vec2::new(0., 0.),
        }
    }
}