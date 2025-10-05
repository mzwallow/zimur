use std::cell::RefCell;
use std::rc::Rc;

use crate::math::{Real, Vec3};
use crate::particle::Particle;

/// A trait for objects that can apply a force to one or more particles.
///
/// This is the basic interface for all force generators. Implementors of this
/// trait can be registered with a `ParticleForceRegistry` to have their
/// forces applied to particles.
pub trait ParticleForceGenerator {
    /// Calculates and applies the force to the given particle.
    ///
    /// This function is called for every particle that this force generator
    /// is registered with.
    ///
    /// # Parameters
    /// - `particle`: The particle to apply the force to.
    /// - `duration`: The duration of the simulation frame in seconds. This can
    ///   be used for forces that are time-dependent, though not all force
    ///   generators will use it.
    fn update_force(&mut self, particle: &mut Particle, duration: Real);
}

/// A struct that links a particle to a force generator.
///
/// This is the core data structure in the `ParticleForceRegistry`. It holds
/// a reference-counted pointer to a `Particle` and a boxed trait object
/// for a `ParticleForceGenerator`. This design allows the registry to
/// manage many-to-many relationships between particles and forces in a
/// flexible and memory-safe way.
struct ParticleForceRegistration {
    particle: Rc<RefCell<Particle>>,
    force_generator: Box<dyn ParticleForceGenerator>,
}

/// A registry that holds all the force generators and the particles they apply to.
pub struct ParticleForceRegistry {
    registrations: Vec<ParticleForceRegistration>,
}

impl ParticleForceRegistry {
    /// Registers that the given force generator applies to the given particle.
    pub fn add(particle: &Particle, fg: impl ParticleForceGenerator) {
        todo!()
    }

    /// Removes the registration of the given force generator from the given particle.
    pub fn remove(particle: &Particle, fg: impl ParticleForceGenerator) {
        todo!()
    }

    /// Clears all registrations from the registry.
    pub fn clear() {
        todo!()
    }

    /// Calls all the force generators to update the forces of their
    /// corresponding particles.
    pub fn update_forces(&mut self, duration: Real) {
        for registration in self.registrations.iter_mut() {
            let mut particle = registration.particle.borrow_mut();

            registration
                .force_generator
                .update_force(&mut particle, duration);
        }
    }
}

// --- Force Generators ---

/// A force generator that applies a constant gravitational force to a particle.
///
/// This is one of the simplest and most common forces in a physics simulation.
pub struct ParticleGravity {
    /// The acceleration due to gravity.
    ///
    /// This is a vector representing the direction and magnitude of the
    /// gravitational acceleration (e.g., `(0, -9.81, 0)` for Earth's gravity).
    gravity: Vec3,
}

impl ParticleGravity {
    /// Creates a new gravity force generator.
    pub fn new(gravity: Vec3) -> Self {
        Self { gravity }
    }
}

impl ParticleForceGenerator for ParticleGravity {
    /// Applies the gravitational force to the given particle.
    ///
    /// The force applied is calculated using Newton's second law: **F = m * a**,
    /// where:
    /// - **F** is the force vector.
    /// - **m** is the mass of the particle.
    /// - **a** is the acceleration due to gravity (`self.gravity`).
    ///
    /// This implementation calculates `gravity * mass` and adds it to the
    /// particle's accumulated force. It also checks that the particle has
    /// finite mass before applying the force.
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        // Check that we do not have infinite mass.
        if !particle.has_finite_mass() {
            return;
        }

        // Apply the mass-scaled force to the particle.
        particle.add_force(&(self.gravity * particle.mass()));
    }
}

/// A force generator that applies a drag force to a particle.
///
/// Drag is a force that opposes motion through a fluid (like air or water).
/// This implementation models drag using a simplified equation that includes
/// both linear and quadratic components.
pub struct ParticleDrag {
    /// The drag coefficient for the linear component of drag.
    /// This represents drag that is proportional to velocity (laminar flow).
    k1: Real,
    /// The drag coefficient for the quadratic component of drag.
    /// This represents drag that is proportional to the square of the velocity (turbulent flow).
    k2: Real,
}

impl ParticleForceGenerator for ParticleDrag {
    /// Applies the drag force to the given particle.
    ///
    /// The drag force **F_drag** is calculated using the formula:
    ///
    /// **F_drag = -v_hat** * (k1 * |v| + k2 * |v|^2)
    ///
    /// Where:
    /// - **v** is the particle's velocity vector.
    /// - **|v|** is the speed of the particle (the magnitude of the velocity).
    /// - **v_hat** is the normalized velocity vector, representing the direction of motion.
    /// - **k1** is the linear drag coefficient (`self.k1`).
    /// - **k2** is the quadratic drag coefficient (`self.k2`).
    ///
    /// The force acts in the opposite direction to the particle's velocity,
    /// slowing it down.
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        let mut force = particle.velocity;

        // Calculate the speed of the particle.
        let speed = force.magnitude();
        if speed <= 0.0 {
            return;
        }

        // Calculate the total drag coefficient.
        let drag_coeff = self.k1 * speed + self.k2 * speed * speed;

        // Calculate the final force and apply it.
        // The force is in the opposite direction of the velocity.
        force.normalize();
        force *= -drag_coeff;
        particle.add_force(&force);
    }
}

pub struct ParticleSpring {
    /// The particle at the other end of the spring.
    other: Rc<RefCell<Particle>>,
    /// Holds the spring constant.
    spring_constant: Real,
    /// Holds the rest length of the spring.
    rest_length: Real,
}

impl ParticleSpring {
    pub fn new(other: &Rc<RefCell<Particle>>, spring_constant: Real, rest_length: Real) -> Self {
        Self {
            other: other.clone(),
            spring_constant,
            rest_length,
        }
    }
}

impl ParticleForceGenerator for ParticleSpring {
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        let mut force = particle.position - self.other.borrow().position;

        // Calculate the magnitude of the force.
        let mut magnitude = force.magnitude();
        if magnitude <= 0.0 {
            return;
        }
        magnitude = magnitude - self.rest_length; // NOTE: Original code has abs()
        magnitude *= self.spring_constant;

        // Calculate the final force and apply it.
        // The force is applied along the line connecting the two particles.
        force.normalize();
        force *= -magnitude;
        particle.add_force(&force);
    }
}

/// A force generator that applied a spring force, where one end is attached
/// to a fixed point in space.
pub struct ParticleAnchoredSpring {
    /// The location of the achored end of the spring.
    anchor: Rc<RefCell<Vec3>>,
    /// Holds the spring constant.
    spring_constant: Real,
    /// Holds the rest length of the spring.
    rest_length: Real,
}

impl ParticleAnchoredSpring {
    pub fn new(anchor: &Rc<RefCell<Vec3>>, spring_constant: Real, rest_length: Real) -> Self {
        Self {
            anchor: anchor.clone(),
            spring_constant,
            rest_length,
        }
    }
}

impl ParticleForceGenerator for ParticleAnchoredSpring {
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        // Calculate the vector of the spring.
        let mut force = particle.position - *self.anchor.borrow();

        // Calculate the magnitude of the force.
        let mut magnitude = force.magnitude();
        if magnitude <= 0.0 {
            return;
        }
        magnitude = self.spring_constant * (self.rest_length - magnitude);

        // Calculate the final force and apply it.
        force.normalize();
        force *= magnitude;
        particle.add_force(&force);
    }
}

/// A force generator that applies a spring force only when extended.
pub struct ParticleBungee {
    /// The particle at the other end of the spring.
    other: Rc<RefCell<Particle>>,
    /// Holds the spring constant.
    spring_constant: Real,
    /// Holds the rest length of the spring.
    rest_length: Real,
}

impl ParticleBungee {
    pub fn new(other: &Rc<RefCell<Particle>>, spring_constant: Real, rest_length: Real) -> Self {
        Self {
            other: other.clone(),
            spring_constant,
            rest_length,
        }
    }
}

impl ParticleForceGenerator for ParticleBungee {
    fn update_force(&mut self, particle: &mut Particle, _duration: Real) {
        let mut force = particle.position - self.other.borrow().position;

        // Check if the bungee is compressed or slack. If so, no force.
        let mut magnitude = force.magnitude();
        if magnitude <= self.rest_length {
            return;
        }

        // Calculate the magnitude of the force.
        magnitude = self.spring_constant * (self.rest_length - magnitude);

        // Calculate the final force and apply it.
        force.normalize();
        force *= -magnitude;
        particle.add_force(&force);
    }
}

/// A force generator that applies a buoyancy force for a plane of liquid
/// parallel to XZ plane.
///
/// `unimplemented!()`
///
// TODO: Implement later
pub struct ParticleBuoyancy {}
