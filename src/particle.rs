use crate::math::{Real, Vec3};

#[derive(Debug)]
pub struct Particle {
    /// The position of the particle in 3D space.
    pub position: Vec3,
    /// The velocity of the particle, representing its speed and direction.
    pub velocity: Vec3,
    /// The acceleration of the particle, which is updated by forces.
    pub acceleration: Vec3,
    /// Damping is a measure of how much a particle is slowed down over time,
    /// similar to air resistance or friction. It's a value between 0.0 and 1.0,
    /// but is usually close to 1.0.
    ///
    /// To apply damping, the particle's velocity is multiplied by the damping
    /// value in each simulation step. For frame-rate independent damping, you
    /// should use `damping.powf(duration)`, where `duration` is the time elapsed
    /// in the step.
    ///
    /// - A damping of `1.0` means the particle retains all of its velocity (no damping).
    /// - A damping of `0.99` means the particle loses 1% of its velocity over a
    ///   certain time period (e.g., per second if applied correctly with duration).
    ///
    /// In a physics simulation, small errors from calculations can build up,
    /// causing objects to gain energy and move unrealistically fast. Damping
    /// counteracts this by removing a small amount of energy in each step,
    /// making the simulation more stable.
    pub damping: Real,
    pub inverse_mass: Real,
    pub force_accum: Vec3,
}

impl Particle {
    pub fn new() -> Self {
        Self {
            position: Vec3::ZERO,
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            damping: Real(0.0),
            inverse_mass: Real(0.0),
            force_accum: Vec3::ZERO,
        }
    }

    pub fn has_finite_mass(&self) -> bool {
        self.inverse_mass >= 0.0
    }

    pub fn set_mass(&mut self, mass: f32) {
        assert!(mass > 0.0);
        self.inverse_mass = Real(1.0) / mass
    }

    pub fn mass(&self) -> Real {
        if self.inverse_mass == 0.0 {
            Real::MAX
        } else {
            Real(1.0) / self.inverse_mass
        }
    }

    pub fn integrate(&mut self, duration: Real) {
        // We don't integrate things with zero mass.
        if self.inverse_mass <= 0.0 {
            return;
        }

        assert!(duration > 0.0);

        // Update linear position
        self.position.add_scaled(self.velocity, duration);

        // Work out the acceleration from the force.
        let mut resulting_acc: Vec3 = self.acceleration;
        resulting_acc.add_scaled(self.force_accum, self.inverse_mass);

        // Update linear velocity from the acceleration.
        self.velocity.add_scaled(resulting_acc, duration);

        // Impose drag.
        self.velocity *= self.damping.pow(duration);

        // Clear the forces.
        self.clear_accumulator();
    }

    pub fn clear_accumulator(&mut self) {
        self.force_accum.clear();
    }

    pub fn add_force(&mut self, force: &Vec3) {
        self.force_accum += *force;
    }
}
