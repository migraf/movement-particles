//! Core particle engine for movement-particles
//! 
//! This crate provides the fundamental particle system implementation including:
//! - Particle lifecycle management
//! - Physics simulation (forces, velocity, acceleration)
//! - Collision detection with outlines
//! - Spatial partitioning for optimization

pub mod particles;
pub mod physics;
pub mod collision;

// Re-export commonly used types
pub use particles::{Particle, ParticleSystem, Emitter};
pub use physics::{Force, Vec2};
pub use collision::{Outline, SpatialGrid};
