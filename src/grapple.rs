use super::utils::Vec3;
use super::block::Block;
use super::utils::AABB;
use super::log;

pub struct Grapple {
    pub end: Vec3,
    cast_vel: Vec3,

    pub hooked: bool,
    pub length: f32,
    
    pub pull: f32, // magnitude of force
}

impl Grapple {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self {
            end,
            cast_vel: (end - start).unit() * 0.4,

            hooked: false,
            length: 20.,
            
            pull: 0.05,
        }
    }

    pub fn cast(&mut self, blocks: &Vec<Block>, time_step: f32) {
        self.end = self.end + self.cast_vel * time_step;
        for block in blocks {
            let min = block.min();
            let max = block.max();
            let x_collide = self.end.x >= min.x && self.end.x <= max.x;
            let y_collide = self.end.y >= min.y && self.end.y <= max.y;
            let z_collide = self.end.z >= min.z && self.end.z <= max.z;
            let collide = x_collide && y_collide && z_collide;
            if collide {
                self.hooked = true;
                log("Hooked!");
            }
        }
    }
}