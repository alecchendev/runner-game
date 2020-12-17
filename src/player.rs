pub struct Player {
    look_spd: f32,
    move_spd: f32,
    jump_spd: f32,

	position: Vec3,
	velocity: Vec3,
	h_vel: f32,
    d_vel: f32,
    theta: f32,
    phi: f32,
    
    dims: Vec3
    on_ground: f32, // set to false each update, and set true if it is colliding with something below it

    grapple: Option<Grapple>,
    pulling: bool,
}

impl AABB for Player {
    fn min(&self) -> Vec3 {
        self.position - self.dims / 2.
    }

    fn max(&self) -> Vec3 {
        self.position + self.dims / 2.
    }
}

#[wasm_bindgen]
pub enum Go {
    Left = 0,
    Forward = 1,
    Right = 2,
    Back = 3,
    Jump = 4,
}

#[wasm_bindgen]
impl Player {
    pub fn new() -> Self {
        log("Created Player!");
        Self {
            look_spd: 0.0008,
            move_spd: 0.1,
            jump_spd: 0.25,

            position: Vec3::new(2., 1.5, -5.),
            velocity: Vec3::new(0., 0., 0.),
            h_vel: 0.,
            d_vel: 0.,
            theta: 0.,
            phi: 0.,
            
            dims: Vec3::new(0.5, 2., 0.5),
            on_ground: false,

            grapple: None,
            pulling: false,
        }
    }

    pub fn update(&mut self, gravity: f32) {

        let del_x = self.theta.sin() * self.move_velz + self.theta.cos() * self.move_velh;
        let del_z = self.theta.cos() * self.move_velz + -self.theta.sin() * self.move_velh;
        let del_y = self.velocity.y + gravity;

        self.velocity.x = delx;
        self.velocity.y = del_y;
        self.velocity.z = del_z;

        for block in &self.universe.blocks {

            let collision_dir = self.collision(block, &self.velocity);
            self.velocity += Vec3::new(collision_dir.x * self.velocity.x, collision_dir.y * self.velocity.y, collision_dir.z * self.velocity.z);

            if collision_dir.y.abs() == 1. {
                self.on_ground = true;
            }

        }

        self.position = self.position + self.velocity;
    }

    pub fn go(&mut self, go: Go) {
        match go {
            Go::Left => self.h_vel = -self.move_spd,
            Go::Forward => self.d_vel = self.move_spd,
            Go::Right => self.h_vel = self.move_spd,
            Go::Back => self.d_vel = -self.move_spd,
            Go::Jump => if self.on_ground { self.velocity.y = self.jump_speed; self.on_ground = false; } else { },
        }
    }

    pub fn stop(&mut self, go: Go) {
        match go {
            Go::Left => {
                if self.h_vel == -self.move_spd {
                    self.h_vel = 0.;
                }
            },
            Go::Forward => {
                if self.d_vel == self.move_spd {
                    self.d_vel = 0.;
                }
            },
            Go::Right => {
                if self.h_vel == self.move_spd {
                    self.h_vel = 0.;
                }
            },
            Go::Back => {
                if self.d_vel == -self.move_spd {
                    self.d_vel = 0.;
                }
            },
            _ => (),
        }
    }

    pub fn mouse_look(&mut self, movement_x: f32, movement_y: f32) {
        let del_theta = movement_x * self.look_spd;
        let del_phi = movement_y * self.look_spd;
        self.theta += del_theta;
        self.phi = {
            if (self.phi + del_phi).abs() < std::f32::consts::PI / 2. {
                self.phi + del_phi
            } else {
                self.phi
            }
        };
    }

    pub fn position(&self) -> Vec<f32> {
        self.position.to_vec()
    }

    pub fn theta(&self) -> f32 {
        self.theta
    }

    pub fn phi(&self) -> f32 {
        self.phi
    }

    // for visualizing third person perspective
    pub fn vel(&self) -> Vec<f32> {
        self.velocity.to_vec()
    }
}