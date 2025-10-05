use std::time::Instant;

use crate::{
    math::{Real, Vec3},
    particle::Particle,
};
use raylib::prelude::*;

pub struct BallisticApp {
    pub shot: AmmoRound,
}

impl<'a> BallisticApp {
    pub fn new() -> Self {
        Self {
            shot: AmmoRound::new(),
        }
    }

    pub fn fire(&mut self) {
        self.shot.particle.position = Vec3::new(0.0, 1.5, 0.0);
        self.shot.particle.set_mass(2.0);
        self.shot.particle.velocity = Vec3::new(0.0, 0.0, 35.0);
        self.shot.particle.acceleration = Vec3::new(0.0, -1.0, 0.0);
        self.shot.particle.damping = Real(0.99);
        self.shot.start_time = Some(Instant::now());
        self.shot.shot_type = ShotType::PISTOL;

        self.shot.particle.clear_accumulator();
    }

    pub fn update(&mut self, dt: Real) {
        self.shot.particle.integrate(dt);
    }

    pub fn display(&mut self, d: &RaylibMode3D<'a, RaylibDrawHandle<'a>>) {
        self.shot.render(d);
    }
}

#[derive(Debug)]
pub enum ShotType {
    UNUSED,
    PISTOL,
    ARTILLERY,
    LASER,
}

#[derive(Debug)]
pub struct AmmoRound {
    pub particle: Particle,
    pub shot_type: ShotType,
    pub start_time: Option<Instant>,
}

impl AmmoRound {
    pub fn new() -> Self {
        Self {
            particle: Particle::new(),
            shot_type: ShotType::PISTOL,
            start_time: None,
        }
    }
}

impl<'a> AmmoRound {
    pub fn render(&self, mut d: &RaylibMode3D<'a, RaylibDrawHandle<'a>>) {
        // d.draw_sphere(self.particle.position, 0.3, Color::BLACK);
    }
}
