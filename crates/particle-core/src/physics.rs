//! Physics simulation types and force calculations

pub use glam::Vec2;

/// Represents a force that can affect particles
#[derive(Clone, Debug)]
pub enum Force {
    /// Constant gravitational force
    Gravity(Vec2),
    
    /// Wind with direction and turbulence
    Wind {
        direction: Vec2,
        strength: f32,
        turbulence: f32,
    },
    
    /// Point attractor
    Attractor {
        position: Vec2,
        strength: f32,
        radius: f32,
    },
    
    /// Point repulsor
    Repulsor {
        position: Vec2,
        strength: f32,
        radius: f32,
    },
}

impl Force {
    /// Calculates the force vector at a given position
    pub fn calculate_at(&self, position: Vec2) -> Vec2 {
        match self {
            Force::Gravity(g) => *g,
            
            Force::Wind { direction, strength, turbulence } => {
                let turbulence_offset = Vec2::new(
                    (position.x * 0.1).sin() * turbulence,
                    (position.y * 0.1).cos() * turbulence,
                );
                direction.normalize_or_zero() * *strength + turbulence_offset
            }
            
            Force::Attractor { position: pos, strength, radius } => {
                let diff = *pos - position;
                let dist_sq = diff.length_squared();
                let radius_sq = radius * radius;
                
                if dist_sq < radius_sq && dist_sq > 0.0 {
                    let dist = dist_sq.sqrt();
                    // Inverse square law with smoothing
                    let force_magnitude = strength / (dist_sq + 1.0);
                    diff.normalize_or_zero() * force_magnitude
                } else {
                    Vec2::ZERO
                }
            }
            
            Force::Repulsor { position: pos, strength, radius } => {
                let diff = position - *pos;
                let dist_sq = diff.length_squared();
                let radius_sq = radius * radius;
                
                if dist_sq < radius_sq && dist_sq > 0.0 {
                    let dist = dist_sq.sqrt();
                    // Inverse square law with smoothing
                    let force_magnitude = strength / (dist_sq + 1.0);
                    diff.normalize_or_zero() * force_magnitude
                } else {
                    Vec2::ZERO
                }
            }
        }
    }

    /// Creates a gravity force
    pub fn gravity(x: f32, y: f32) -> Self {
        Force::Gravity(Vec2::new(x, y))
    }

    /// Creates a wind force
    pub fn wind(direction: Vec2, strength: f32) -> Self {
        Force::Wind {
            direction,
            strength,
            turbulence: 0.0,
        }
    }

    /// Creates an attractor force
    pub fn attractor(position: Vec2, strength: f32, radius: f32) -> Self {
        Force::Attractor {
            position,
            strength,
            radius,
        }
    }

    /// Creates a repulsor force
    pub fn repulsor(position: Vec2, strength: f32, radius: f32) -> Self {
        Force::Repulsor {
            position,
            strength,
            radius,
        }
    }
}

/// Physics utility functions
pub mod utils {
    use super::Vec2;

    /// Reflects a velocity vector off a surface with the given normal
    pub fn reflect(velocity: Vec2, normal: Vec2) -> Vec2 {
        velocity - 2.0 * velocity.dot(normal) * normal
    }

    /// Clamps a vector to a maximum length
    pub fn clamp_length(v: Vec2, max_length: f32) -> Vec2 {
        let len_sq = v.length_squared();
        if len_sq > max_length * max_length {
            v.normalize() * max_length
        } else {
            v
        }
    }

    /// Linear interpolation between two vectors
    pub fn lerp(a: Vec2, b: Vec2, t: f32) -> Vec2 {
        a + (b - a) * t
    }
}

