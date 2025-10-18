//! Particle system implementation

use crate::physics::{Force, Vec2};
use rand::Rng;

/// Represents a single particle in the system
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Particle {
    /// Position in 2D space
    pub position: [f32; 2],
    /// Velocity vector
    pub velocity: [f32; 2],
    /// Current lifetime (seconds remaining)
    pub life: f32,
    /// Visual size (radius in pixels)
    pub size: f32,
    /// RGBA color
    pub color: [f32; 4],
    /// Mass for physics calculations
    pub mass: f32,
    /// Padding for alignment
    _padding: [f32; 2],
}

impl Particle {
    /// Creates a new particle with the given parameters
    pub fn new(position: Vec2, velocity: Vec2, life: f32, size: f32, color: [f32; 4]) -> Self {
        Self {
            position: position.into(),
            velocity: velocity.into(),
            life,
            size,
            color,
            mass: 1.0,
            _padding: [0.0; 2],
        }
    }

    /// Updates the particle state for one time step
    pub fn update(&mut self, dt: f32, forces: &[Force]) {
        let pos = Vec2::from(self.position);
        let mut vel = Vec2::from(self.velocity);
        
        // Calculate acceleration from all forces
        let mut acceleration = Vec2::ZERO;
        for force in forces {
            acceleration += force.calculate_at(pos) / self.mass;
        }
        
        // Euler integration
        vel += acceleration * dt;
        let new_pos = pos + vel * dt;
        
        // Apply drag
        vel *= 0.99;
        
        // Update particle
        self.position = new_pos.into();
        self.velocity = vel.into();
        self.life -= dt;
    }

    /// Returns true if the particle is still alive
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }

    /// Gets the particle position as Vec2
    pub fn pos(&self) -> Vec2 {
        Vec2::from(self.position)
    }

    /// Gets the particle velocity as Vec2
    pub fn vel(&self) -> Vec2 {
        Vec2::from(self.velocity)
    }
}

/// Configuration for particle system behavior
#[derive(Clone, Debug)]
pub struct ParticleConfig {
    pub max_particles: usize,
    pub spawn_rate: f32,
    pub particle_lifetime: f32,
    pub particle_size: f32,
    pub gravity: Vec2,
    pub drag_coefficient: f32,
}

impl Default for ParticleConfig {
    fn default() -> Self {
        Self {
            max_particles: 10000,
            spawn_rate: 500.0,
            particle_lifetime: 5.0,
            particle_size: 3.0,
            gravity: Vec2::new(0.0, 100.0),
            drag_coefficient: 0.99,
        }
    }
}

/// Particle emitter that spawns new particles
#[derive(Clone, Debug)]
pub struct Emitter {
    pub position: Vec2,
    pub rate: f32,
    pub spread: f32,
    pub initial_velocity: f32,
    pub particle_lifetime: f32,
    pub particle_size: f32,
    pub enabled: bool,
    accumulator: f32,
}

impl Emitter {
    /// Creates a new emitter at the given position
    pub fn new(position: Vec2) -> Self {
        Self {
            position,
            rate: 100.0,
            spread: std::f32::consts::PI / 4.0,
            initial_velocity: 50.0,
            particle_lifetime: 5.0,
            particle_size: 3.0,
            enabled: true,
            accumulator: 0.0,
        }
    }

    /// Spawns particles for this frame, returns the particles to add
    pub fn emit(&mut self, dt: f32) -> Vec<Particle> {
        if !self.enabled {
            return Vec::new();
        }

        self.accumulator += dt * self.rate;
        let count = self.accumulator.floor() as usize;
        self.accumulator -= count as f32;

        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(count);

        for _ in 0..count {
            let angle = rng.gen_range(-self.spread..self.spread);
            let velocity = Vec2::new(
                angle.cos() * self.initial_velocity,
                angle.sin() * self.initial_velocity,
            );

            let color = [
                rng.gen_range(0.5..1.0),
                rng.gen_range(0.5..1.0),
                rng.gen_range(0.8..1.0),
                1.0,
            ];

            particles.push(Particle::new(
                self.position,
                velocity,
                self.particle_lifetime,
                self.particle_size,
                color,
            ));
        }

        particles
    }
}

/// Main particle system managing all particles
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub emitters: Vec<Emitter>,
    pub config: ParticleConfig,
}

impl ParticleSystem {
    /// Creates a new particle system with default configuration
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            emitters: Vec::new(),
            config: ParticleConfig::default(),
        }
    }

    /// Updates all particles for one frame
    pub fn update(&mut self, dt: f32, forces: &[Force]) {
        // Update existing particles
        for particle in &mut self.particles {
            particle.update(dt, forces);
        }

        // Remove dead particles
        self.particles.retain(|p| p.is_alive());

        // Emit new particles
        for emitter in &mut self.emitters {
            let new_particles = emitter.emit(dt);
            for particle in new_particles {
                if self.particles.len() < self.config.max_particles {
                    self.particles.push(particle);
                }
            }
        }
    }

    /// Adds an emitter to the system
    pub fn add_emitter(&mut self, emitter: Emitter) {
        self.emitters.push(emitter);
    }

    /// Returns the current particle count
    pub fn particle_count(&self) -> usize {
        self.particles.len()
    }
}

impl Default for ParticleSystem {
    fn default() -> Self {
        Self::new()
    }
}

