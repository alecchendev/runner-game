use wasm_bindgen::prelude::*;
use super::utils::Vec3;
use super::grapple::Grapple;
use super::block::Block;
use super::utils::AABB;
use super::log;

pub struct Player {
    look_spd: f32,
    move_acc: f32,
    move_dec: f32,
    move_spd: f32,
    jump_spd: f32,

	pub position: Vec3,
	velocity: Vec3,
	h_vel: f32,
    d_vel: f32,
    theta: f32,
    phi: f32,
    
    dims: Vec3,
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
    pub fn new() -> Self {
        log("Created Player!");
        Self {
            look_spd: 0.0008,
            move_acc: 0.045,
            move_dec: 0.045,
            move_spd: 0.09,
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

    pub fn cast_grapple(&mut self) {
        match &mut self.grapple {
            None => {
                self.grapple = Some(Grapple::new(self.position.clone(), self.position.clone() + Vec3::new(self.theta.sin(), -self.phi.sin(), self.theta.cos())));
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

    pub fn update(&mut self, blocks: &Vec<Block>, gravity: f32) {

        match &mut self.grapple {
            None => self.pulling = false,
            Some(grapple) => {
                if grapple.hooked {
                    if self.pulling {
                        self.velocity += (grapple.end - self.position).unit() * grapple.pull;
                    }
                } else {
                    if (grapple.end - self.position).length() > grapple.length {
                        self.grapple = None;
                    } else {
                        grapple.cast(blocks);
                        log(&format!("{}", grapple.end)[..]);
                    }
                }
            }
        }

        self.on_ground = false;

        let h_dir = Vec3::new(self.theta.cos(), 0., -self.theta.sin());
        let d_dir = Vec3::new(self.theta.sin(), 0., self.theta.cos());
        let h_vel = self.velocity.project_onto(&h_dir);
        let d_vel = self.velocity.project_onto(&d_dir);

        let mut movement = Vec3::new(0., 0., 0.);
        if (h_vel + h_dir * self.h_vel).length() < self.move_spd {
            movement += h_dir * self.h_vel;
        } else if self.h_vel != 0. {
            movement += (h_dir * self.move_spd * self.h_vel.signum()) - h_vel;
        }
        if (d_vel + d_dir * self.d_vel).length() < self.move_spd {
            movement += d_dir * self.d_vel;
        } else if self.d_vel != 0. {
            movement += (d_dir * self.move_spd * self.d_vel.signum()) - d_vel;
        }
        if h_vel.length() > 0. && self.h_vel == 0. {
            if h_vel.length() <= self.move_dec {
                movement -= h_vel;
            } else {
                movement -= h_dir * h_vel.dot(&h_dir).signum() * self.move_dec;
            }
        }
        if d_vel.length() > 0. && self.d_vel == 0. {
            if d_vel.length() <= self.move_dec {
                movement -= d_vel;
            } else {
                movement -= d_dir * d_vel.dot(&d_dir).signum() * self.move_dec;
            }
        }

        self.velocity += movement + Vec3::new(0., gravity, 0.);

        /*let del_x = self.theta.sin() * self.d_vel + self.theta.cos() * self.h_vel;
        let del_z = self.theta.cos() * self.d_vel + -self.theta.sin() * self.h_vel;
        let del_y = self.velocity.y + gravity;

        self.velocity.x = del_x;
        self.velocity.y = del_y;
        self.velocity.z = del_z;*/

        for block in blocks {

            let collision_dir = self.collision(block, &self.velocity);
            self.velocity = self.velocity + Vec3::new(collision_dir.x * self.velocity.x, collision_dir.y * self.velocity.y, collision_dir.z * self.velocity.z);

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

        self.position = self.position + self.velocity;
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