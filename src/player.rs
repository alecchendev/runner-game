

pub struct Player {
    look_spd: f32,
    move_spd: f32,
    jump_spd: f32,

	position: Vec3,
	velocity: Vec3,
	h_vel: f32,
	d_vel: f32,
    
    on_ground: f32, // set to false each update, and set true if it is colliding with something below it

    grapple: Option<Grapple>,
    pulling: bool,
}