// Vertex shader for particle rendering with instancing

struct VertexInput {
    @location(0) position: vec2<f32>,
}

struct InstanceInput {
    @location(1) particle_pos: vec2<f32>,
    @location(2) velocity: vec2<f32>,
    @location(3) life: f32,
    @location(4) size: f32,
    @location(5) color: vec4<f32>,
    @location(6) mass: f32,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Scale the quad by particle size
    let scaled_pos = vertex.position * instance.size;
    
    // Transform to screen space (assuming normalized coordinates)
    // For now, simple pass-through. TODO: Add proper projection matrix
    let world_pos = scaled_pos + instance.particle_pos;
    
    // Convert to clip space (assuming canvas is 0-1280 x 0-720 or similar)
    // This is a simplified version - should use proper uniforms
    let clip_x = (world_pos.x / 640.0) - 1.0;
    let clip_y = 1.0 - (world_pos.y / 360.0);
    
    out.clip_position = vec4<f32>(clip_x, clip_y, 0.0, 1.0);
    out.color = instance.color;
    out.uv = vertex.position; // -1 to 1 range
    
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Create circular particles using distance from center
    let dist = length(in.uv);
    
    // Soft circle with smooth falloff
    let alpha = 1.0 - smoothstep(0.5, 1.0, dist);
    
    // Add glow effect
    let glow = pow(1.0 - dist, 2.0) * 0.3;
    
    var color = in.color;
    color.a *= alpha;
    color = vec4<f32>(color.rgb + vec3<f32>(glow), color.a);
    
    // Discard fully transparent pixels
    if (color.a < 0.01) {
        discard;
    }
    
    return color;
}

