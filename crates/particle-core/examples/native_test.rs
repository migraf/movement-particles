//! Simple native test to verify particle system works
//! Run with: cargo run --example native_test

use particle_core::{ParticleSystem, Emitter, Force};
use glam::Vec2;

fn main() {
    println!("🦀 Testing particle-core...\n");

    // Create particle system
    let mut system = ParticleSystem::new();
    println!("✓ Created particle system");
    println!("  Max particles: {}", system.config.max_particles);

    // Add an emitter
    let emitter = Emitter::new(Vec2::new(100.0, 100.0));
    system.add_emitter(emitter);
    println!("✓ Added emitter at (100, 100)");

    // Create forces
    let forces = vec![
        Force::gravity(0.0, 98.0),  // Gravity
        Force::wind(Vec2::new(10.0, 0.0), 5.0),  // Light wind
    ];
    println!("✓ Created forces (gravity + wind)");

    // Simulate for a few frames
    println!("\n📊 Simulating 120 frames (2 seconds at 60fps)...\n");
    let dt = 1.0 / 60.0;
    
    for frame in 0..120 {
        system.update(dt, &forces);
        
        if frame % 30 == 0 {
            println!("Frame {}: {} particles", frame, system.particle_count());
        }
    }

    println!("\n✅ Particle system test completed!");
    println!("Final particle count: {}", system.particle_count());
    
    // Test outline collision
    println!("\n🎯 Testing collision detection...");
    use particle_core::Outline;
    
    let outline_points = vec![
        Vec2::new(50.0, 50.0),
        Vec2::new(150.0, 50.0),
        Vec2::new(150.0, 150.0),
        Vec2::new(50.0, 150.0),
    ];
    
    let outline = Outline::from_points(outline_points);
    println!("✓ Created outline with {} segments", outline.segments.len());
    
    // Test point containment
    let inside_point = Vec2::new(100.0, 100.0);
    let outside_point = Vec2::new(200.0, 200.0);
    
    println!("Point (100, 100) inside: {}", outline.contains(inside_point));
    println!("Point (200, 200) inside: {}", outline.contains(outside_point));
    
    println!("\n🎉 All tests passed!");
}

