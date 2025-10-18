# Particle System Implementation

Comprehensive design considerations for building a high-performance particle system in Rust/WebAssembly.

## Overview

The particle system is the visual centerpiece of this project. It must:
- Handle 10,000+ particles at 60fps
- Respond physically to body outline collisions
- Look visually appealing with various effects
- Run efficiently in WebAssembly
- Leverage GPU for rendering

## Architecture Options

### 1. CPU-based Physics, GPU Rendering (Recommended for Phase 1)

**Structure:**
```rust
struct Particle {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    life: f32,
    max_life: f32,
    color: Color,
    size: f32,
}

struct ParticleSystem {
    particles: Vec<Particle>,
    emitters: Vec<Emitter>,
    forces: Vec<Force>,
    spatial_grid: SpatialGrid,
}
```

**Pros:**
- Simple to implement and debug
- Full control over physics simulation
- Easy to integrate with collision detection
- Works reliably in WASM

**Cons:**
- CPU-bound, limited particle count
- WASM has some performance overhead vs native

### 2. GPU Compute-based (WebGPU)

Use compute shaders to update particle state on GPU.

**Pros:**
- Massive parallelization (100k+ particles possible)
- Minimal CPU-GPU data transfer
- Modern, future-proof approach

**Cons:**
- More complex implementation
- WebGPU not universally supported yet (need fallback)
- Collision detection more complex on GPU
- Debugging is harder

### 3. Hybrid Approach

CPU handles collision/interaction logic, GPU handles basic forces and rendering.

**Best of both worlds** - Recommended for final implementation.

## Core Components

### Particle Structure

```rust
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub life: f32,
    pub size: f32,
    pub color: [f32; 4], // RGBA
    pub mass: f32,       // For physics calculations
}

impl Particle {
    pub fn update(&mut self, dt: f32, forces: &[Force]) {
        // Apply forces
        let mut acceleration = Vec2::ZERO;
        for force in forces {
            acceleration += force.calculate(self.position);
        }
        
        // Euler integration (or use Verlet/RK4 for better stability)
        self.velocity += acceleration * dt;
        self.position += self.velocity * dt;
        
        // Apply drag
        self.velocity *= 0.99;
        
        // Update life
        self.life -= dt;
    }
    
    pub fn is_alive(&self) -> bool {
        self.life > 0.0
    }
}
```

### Force System

Forces affect particle movement. Types include:
- **Global forces**: gravity, wind, drag
- **Field forces**: attraction/repulsion points, vortices, flow fields
- **Outline forces**: repulsion from body outline (key interaction mechanism)

```rust
pub enum Force {
    Gravity(Vec2),
    Wind { 
        direction: Vec2, 
        strength: f32,
        turbulence: f32 
    },
    Attractor {
        position: Vec2,
        strength: f32,
        radius: f32,
    },
    Repulsor {
        position: Vec2,
        strength: f32,
        radius: f32,
    },
    FlowField {
        field: Vec<Vec2>, // Grid of force vectors
        cell_size: f32,
    },
    OutlineForce {
        segments: Vec<LineSegment>,
        repulsion_distance: f32,
        strength: f32,
    },
}

impl Force {
    pub fn calculate(&self, particle_pos: Vec2) -> Vec2 {
        match self {
            Force::Gravity(g) => *g,
            Force::Attractor { position, strength, radius } => {
                let diff = *position - particle_pos;
                let dist = diff.length();
                if dist < *radius && dist > 0.0 {
                    diff.normalize() * strength / (dist * dist)
                } else {
                    Vec2::ZERO
                }
            }
            // ... other force calculations
        }
    }
}
```

### Emitter System

Controls particle spawning patterns.

```rust
pub struct Emitter {
    pub position: Vec2,
    pub rate: f32,          // Particles per second
    pub spread: f32,        // Angular spread in radians
    pub initial_velocity: f32,
    pub particle_lifetime: f32,
    pub enabled: bool,
}

pub enum EmitterShape {
    Point,
    Circle { radius: f32 },
    Rectangle { width: f32, height: f32 },
    Line { start: Vec2, end: Vec2 },
}
```

## Collision Detection

Most performance-critical part of the system. Particles need to collide with the detected body outline.

### Outline Representation

The CV system provides a person outline. Convert it to a collision-friendly format:

```rust
pub struct Outline {
    pub segments: Vec<LineSegment>,
    pub simplified: Vec<Vec2>,  // Simplified polygon
    pub bounds: AABB,           // Bounding box for quick rejection
    pub velocity: Vec2,         // For swipe detection
}

pub struct LineSegment {
    pub start: Vec2,
    pub end: Vec2,
    pub normal: Vec2,  // Outward facing normal
}
```

### Spatial Partitioning

Essential for performance with thousands of particles.

**Option 1: Uniform Grid**
```rust
pub struct SpatialGrid {
    cell_size: f32,
    cells: HashMap<(i32, i32), Vec<usize>>, // Cell coords -> particle indices
    width: usize,
    height: usize,
}

impl SpatialGrid {
    pub fn insert(&mut self, particle_idx: usize, position: Vec2) {
        let cell = self.get_cell(position);
        self.cells.entry(cell).or_default().push(particle_idx);
    }
    
    pub fn query_nearby(&self, position: Vec2, radius: f32) -> Vec<usize> {
        // Return particle indices in nearby cells
    }
}
```

**Option 2: Quadtree** (better for non-uniform distributions)
- More complex but can be more efficient
- Better when particles cluster in certain areas

### Collision Response

When particle collides with outline:

```rust
fn handle_outline_collision(
    particle: &mut Particle, 
    outline: &Outline
) -> bool {
    for segment in &outline.segments {
        if let Some(collision) = check_particle_segment_collision(
            particle.position, 
            particle.size / 2.0, 
            segment
        ) {
            // Reflect velocity
            let normal = segment.normal;
            particle.velocity = reflect(particle.velocity, normal) * 0.7; // Energy loss
            
            // Push particle outside
            particle.position = collision.contact_point + normal * particle.size;
            
            // Add outline velocity (for "swiping" effect)
            particle.velocity += outline.velocity * 0.5;
            
            return true;
        }
    }
    false
}

fn reflect(velocity: Vec2, normal: Vec2) -> Vec2 {
    velocity - 2.0 * velocity.dot(normal) * normal
}
```

### Swipe Detection

Enhance interaction by detecting fast movements:

```rust
pub struct SwipeForce {
    pub outline_velocity: Vec2,
    pub strength_multiplier: f32,
}

impl SwipeForce {
    pub fn from_outline(current: &Outline, previous: &Outline, dt: f32) -> Self {
        let velocity = (current.center() - previous.center()) / dt;
        let speed = velocity.length();
        
        // Amplify fast movements
        let multiplier = if speed > SWIPE_THRESHOLD {
            1.0 + (speed - SWIPE_THRESHOLD) * 2.0
        } else {
            1.0
        };
        
        Self {
            outline_velocity: velocity,
            strength_multiplier: multiplier,
        }
    }
}
```

## Rendering Strategies

### Instanced Rendering (Recommended)

Render all particles in a single draw call using instancing.

```rust
// Vertex shader receives instance data
struct InstanceData {
    position: vec2<f32>,
    color: vec4<f32>,
    size: f32,
}

// Upload all particle data as instance buffer
let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    label: Some("Particle Instance Buffer"),
    contents: bytemuck::cast_slice(&particle_data),
    usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
});

// Single draw call for all particles
render_pass.draw(0..6, 0..particle_count); // 6 vertices for quad
```

### Particle Appearance Options

1. **Simple Circles/Quads**
   - Fastest, clean look
   - Use signed distance field in fragment shader for smooth circles

2. **Textured Sprites**
   - Use texture atlas for variety
   - Glow/blur textures for soft look

3. **Point Sprites**
   - Hardware accelerated
   - Limited customization

4. **Trails**
   - Keep history of last N positions
   - Render as line strip or fading particles

### Visual Effects

```rust
// Glow effect in fragment shader
fn glow(uv: vec2<f32>, center: vec2<f32>, size: f32) -> f32 {
    let dist = distance(uv, center);
    let falloff = 1.0 - smoothstep(0.0, size, dist);
    return pow(falloff, 2.0); // Quadratic falloff
}

// Color variation over lifetime
fn lifetime_color(t: f32) -> vec4<f32> {
    // t goes from 1.0 (birth) to 0.0 (death)
    let birth_color = vec4<f32>(1.0, 1.0, 0.5, 1.0); // Yellow
    let death_color = vec4<f32>(0.2, 0.4, 1.0, 0.0); // Transparent blue
    return mix(death_color, birth_color, t);
}
```

## Performance Optimization

### WASM-Specific Optimizations

1. **Use SIMD when possible**
   ```rust
   #[cfg(target_arch = "wasm32")]
   use std::arch::wasm32::*;
   ```

2. **Minimize allocations**
   - Reuse particle pool instead of allocating/deallocating
   - Pre-allocate vectors with capacity

3. **Batch JS ↔ Rust calls**
   - Don't call across boundary per-particle
   - Pass all outline data in single call

4. **Compile with optimizations**
   ```toml
   [profile.release]
   opt-level = 3
   lto = true
   codegen-units = 1
   ```

### Algorithm Optimization

1. **Parallel Processing**
   ```rust
   use rayon::prelude::*;
   
   particles.par_iter_mut().for_each(|p| {
       p.update(dt, &forces);
   });
   ```
   Note: Check if rayon works well with WASM, may need wasm-bindgen-rayon

2. **Fixed Time Step**
   ```rust
   const FIXED_DT: f32 = 1.0 / 60.0;
   let mut accumulator = 0.0;
   
   accumulator += frame_time;
   while accumulator >= FIXED_DT {
       update_physics(FIXED_DT);
       accumulator -= FIXED_DT;
   }
   ```

3. **Limit Collision Checks**
   - Only check particles near outline (spatial partitioning)
   - Use bounding box tests before detailed checks
   - Simplify outline (fewer segments)

## Parameter Tuning

Key parameters to expose for experimentation:

```rust
pub struct ParticleConfig {
    pub max_particles: usize,           // 10000
    pub spawn_rate: f32,                // 500/sec
    pub particle_lifetime: f32,         // 5.0 sec
    pub particle_size: f32,             // 2.0 - 5.0
    pub gravity: Vec2,                  // (0, 100)
    pub drag_coefficient: f32,          // 0.99
    pub outline_repulsion_strength: f32, // 500.0
    pub outline_repulsion_distance: f32, // 50.0
    pub swipe_force_multiplier: f32,    // 2.0
    pub collision_damping: f32,         // 0.7
}
```

## Future Enhancements

- **Particle Types**: Different behaviors (light, heavy, electric)
- **Connections**: Link nearby particles with lines (constellation effect)
- **Morphing**: Particles form shapes/patterns
- **Physics Interactions**: Particles affect each other (flocking, attraction)
- **Audio Reactivity**: Size/color respond to audio input
- **Recording**: Capture particle animations

## References

- [Nature of Code - Particle Systems](https://natureofcode.com/book/chapter-4-particle-systems/)
- [Coding Train - Particle Systems](https://thecodingtrain.com/tracks/the-nature-of-code-2/noc/4-particles)
- [GPU Gems - Particle Systems](https://developer.nvidia.com/gpugems/gpugems3/part-iv-image-effects/chapter-23-high-speed-screen-particles)
- [WebGPU Samples - Particles](https://webgpu.github.io/webgpu-samples/samples/particles)

