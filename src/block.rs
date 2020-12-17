pub struct Block {
    origin: Vec3,
    dims: Vec3,
}

impl Block {
    pub fn new(origin: Vec3, dims: Vec3) -> Self {
        let bounding_box = BoundingBox::new(origin, dims);
        Self {
            origin,
            dims,
            bounding_box,
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