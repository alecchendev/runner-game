pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[derive(Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
        }
    }

    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        if self.length() == 0. {
            return self.clone()
        }
        self.clone() / self.length()
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn project_onto(&self, other: &Vec3) -> Vec3 {
        if other.length() == 0. {
            return other.clone()
        }
        other.clone() * (self.dot(other) / other.length().powi(2))
    }

    pub fn to_vec(&self) -> Vec<f32> {
        vec![self.x, self.y, self.z]
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

// collision trait
pub trait AABB {
    fn min(&self) -> Vec3;
    
    fn max(&self) -> Vec3;

    fn collision(&self, other: &impl AABB, vel: &Vec3) -> Vec3 {

        let a_min = self.min();
        let a_max = self.max();
        let b_min = other.min();
        let b_max = other.max();

        let a_min_next = a_min + *vel;
        let a_max_next = a_max + *vel;

        let x_overlap = (a_min_next.x <= b_max.x) && (a_max_next.x >= b_min.x);
        let y_overlap = (a_min_next.y <= b_max.y) && (a_max_next.y >= b_min.y);
        let z_overlap = (a_min_next.z <= b_max.z) && (a_max_next.z >= b_min.z);
        let will_collide = x_overlap && y_overlap && z_overlap;

        if will_collide {
            let x_overlap = (a_min.x <= b_max.x) && (a_max.x >= b_min.x);
            let y_overlap = (a_min.y <= b_max.y) && (a_max.y >= b_min.y);
            let z_overlap = (a_min.z <= b_max.z) && (a_max.z >= b_min.z);

            if !x_overlap && y_overlap && z_overlap {
                return Vec3::new(-1. * vel.x.signum(), 0., 0.)
            } else if x_overlap && !y_overlap && z_overlap {
                return Vec3::new(0., -1. * vel.y.signum(), 0.)
            } else if x_overlap && y_overlap && !z_overlap {
                return Vec3::new(0., 0., -1. * vel.z.signum())
            } else {
                let x_time_collide = (a_min.x - b_max.x).abs().min((a_max.x - b_min.x).abs()) / vel.x;
                let y_time_collide = (a_min.y - b_max.y).abs().min((a_max.y - b_min.y).abs()) / vel.y;
                let z_time_collide = (a_min.z - b_max.z).abs().min((a_max.z - b_min.z).abs()) / vel.z;
                if x_time_collide <= y_time_collide && x_time_collide <= z_time_collide {
                    return Vec3::new(-1. * vel.x.signum(), 0., 0.)
                }
                if y_time_collide <= x_time_collide && y_time_collide <= z_time_collide {
                    return Vec3::new(0., -1. * vel.y.signum(), 0.)
                }
                if z_time_collide <= x_time_collide && z_time_collide <= y_time_collide {
                    return Vec3::new(0., 0., -1. * vel.z.signum())
                }
            }
        }
        Vec3::new(0., 0., 0.)
    }
}