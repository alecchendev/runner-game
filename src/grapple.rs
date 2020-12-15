

pub struct Grapple {
    end: Vec3,
    cast_vel: Vec3,

    hooked: bool,
    length: f32,
    
    pull: f32, // magnitude of force
}