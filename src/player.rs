use wasm_bindgen::prelude::*;
use super::utils::Vec3;
use super::grapple::Grapple;
use super::block::Block;
use super::utils::AABB;
use super::log;

pub struct Player {
    look_spd: f32,
    move_acc: f32,
    friction: f32,
    air_res: f32,
    move_spd: f32,
    term_spd: f32,
    jump_spd: f32,

	pub position: Vec3,
	pub velocity: Vec3,
	h_vel: f32,
    d_vel: f32,
    theta: f32,
    phi: f32,
    
    pub dims: Vec3,
    on_ground: bool, // set to false each update, and set true if it is colliding with something below it

    pub grapple: Option<Grapple>,
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

impl Player {
    pub fn new(position: Vec3) -> Self {
        log("Created Player!");
        Self {
            look_spd: 0.0008,
            move_acc: 0.04,
            friction: 0.03,
            air_res: 0.0015,
            move_spd: 0.1,
            term_spd: 0.5,
            jump_spd: 0.2,

            position,
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

    pub fn cast_grapple(&mut self) {
        match &mut self.grapple {
            None => {
                let cast_dir = Vec3::new(self.theta.sin() * self.phi.cos(), self.phi.sin(), self.theta.cos() * self.phi.cos());
                self.grapple = Some(Grapple::new(self.position.clone(), self.position.clone() + cast_dir));
                log("Created grapple!");
            },
            Some(_) => {
                self.grapple = None;
                log("Destroyed grapple!");
            },
        }
    }

    pub fn pull_grapple(&mut self) {
        self.pulling = true;
        log("Pulling grapple!");
    }

    pub fn release_grapple(&mut self) {
        self.pulling = false;
        log("Released grapple!");
    }

    pub fn update(&mut self, blocks: &Vec<Block>, gravity: f32, elapsed_time: f32) {
        let fps_throttle = 1000. / 60.;
        let time_step = elapsed_time / fps_throttle;

        // FRICTION AND AIR RES

        let hd_vel = Vec3::new(self.velocity.x, 0., self.velocity.z);

        if hd_vel.length() > 0. {
            if self.on_ground {
                if hd_vel.length() <= self.friction {
                    self.velocity.x = 0.;
                    self.velocity.z = 0.;
                } else {
                    self.velocity -= hd_vel.unit() * self.friction * time_step;
                }
            } else {
                if hd_vel.length() <= self.air_res {
                    self.velocity.x = 0.;
                    self.velocity.z = 0.;
                } else {
                    self.velocity -= hd_vel.unit() * self.air_res * time_step;
                }
            }
        }

        // USER MOVEMENT

        let hd_vel = Vec3::new(self.velocity.x, 0., self.velocity.z);
        let h_dir = Vec3::new(self.theta.cos(), 0., -self.theta.sin());
        let d_dir = Vec3::new(self.theta.sin(), 0., self.theta.cos());

        let move_dir = (h_dir * self.h_vel + d_dir * self.d_vel).unit();
        let move_acc = move_dir * self.move_acc;

        if hd_vel.length() <= self.move_spd + 0.001 { // ApproxEq
            if (hd_vel + move_acc).length() >= self.move_spd {
                self.velocity.x = ((hd_vel + move_acc).unit() * self.move_spd).x;
                self.velocity.z = ((hd_vel + move_acc).unit() * self.move_spd).z;
            } else {
                self.velocity += move_acc * time_step; // ADFSDFSDD
            }
        }

        // GRAVITY
        self.velocity += Vec3::new(0., gravity, 0.) * time_step;

        // GRAPPLE

        match &mut self.grapple {
            None => self.pulling = false,
            Some(grapple) => {
                if grapple.hooked {
                    let grapple_dir = (grapple.end - self.position).unit();
                    if self.velocity.dot(&grapple_dir) < 0. {
                        self.velocity = self.velocity - self.velocity.project_onto(&grapple_dir); // project onto the plane of the normal
                    }
                    if self.pulling {
                        self.velocity += grapple_dir * grapple.pull * time_step;
                    }
                } else {
                    if (grapple.end - self.position).length() > grapple.length {
                        self.grapple = None;
                    } else {
                        grapple.cast(blocks, time_step);
                    }
                }
            }
        }

        // COLLISIONS

        self.on_ground = false;

        for block in blocks {

            let collision_dir = self.collision(block, &(self.velocity * time_step));

            if collision_dir.x.abs() != 0. {
                self.velocity.x = 0.;
            }
            if collision_dir.y.abs() == 1. {
                self.velocity.y = 0.;
                self.on_ground = true;
            }
            if collision_dir.z.abs() != 0. {
                self.velocity.z = 0.;
            }

        }

        // TERMINAL VELOCITY
        if self.velocity.length() > self.term_spd {
            self.velocity = self.velocity.unit() * self.term_spd;
        }

        // MOVEMENT
        self.position = self.position + self.velocity * time_step;
    }

    pub fn go(&mut self, go: Go) {
        match go {
            Go::Left => self.h_vel = -self.move_acc,
            Go::Forward => self.d_vel = self.move_acc,
            Go::Right => self.h_vel = self.move_acc,
            Go::Back => self.d_vel = -self.move_acc,
            Go::Jump => if self.on_ground { self.velocity.y = self.jump_spd; self.on_ground = false; } else { },
        }
    }

    pub fn stop(&mut self, go: Go) {
        match go {
            Go::Left => {
                if self.h_vel == -self.move_acc {
                    self.h_vel = 0.;
                }
            },
            Go::Forward => {
                if self.d_vel == self.move_acc {
                    self.d_vel = 0.;
                }
            },
            Go::Right => {
                if self.h_vel == self.move_acc {
                    self.h_vel = 0.;
                }
            },
            Go::Back => {
                if self.d_vel == -self.move_acc {
                    self.d_vel = 0.;
                }
            },
            _ => (),
        }
    }

    pub fn mouse_look(&mut self, movement_x: f32, movement_y: f32) {
        let movement_y = -movement_y;
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
}