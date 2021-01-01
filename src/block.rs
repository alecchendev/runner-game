use super::utils::Vec3;
use super::utils::AABB;

#[derive(Clone)]
pub struct Block {
    pub origin: Vec3,
    pub dims: Vec3,
}

impl Block {
    pub fn new(origin: Vec3, dims: Vec3) -> Self {
        Self {
            origin,
            dims,
        }
    }
}

impl AABB for Block {
    fn min(&self) -> Vec3 {
        self.origin
    }

    fn max(&self) -> Vec3 {
        self.origin + self.dims
    }
}