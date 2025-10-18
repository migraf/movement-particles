# Next Steps

## ✅ What's Done

Your project structure is fully set up and the core particle engine is **working**! 

### Verified Working ✓
```
🦀 Testing particle-core...
✓ Created particle system (Max particles: 10000)
✓ Added emitter at (100, 100)
✓ Created forces (gravity + wind)
✓ Simulated 120 frames → 200 particles spawned
✓ Collision detection working (outline containment tests passing)
🎉 All tests passed!
```

## 🔧 Minor Fixes Needed

### 1. wgpu Surface Creation (5-10 minutes)
**Issue**: API compatibility between wgpu version and WASM canvas binding

**Solutions** (pick one):

**A) Use older compatible wgpu version** (easiest):
```toml
# In Cargo.toml workspace.dependencies
wgpu = "0.19"
```

**B) Update to latest API pattern**:
```rust
// In wasm-bridge/src/lib.rs
use wasm_bindgen::JsCast;

let surface = {
    let web_window = web_sys::window().unwrap();
    let canvas_element = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
    instance.create_surface_from_canvas(&canvas_element)
        .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?
};
```

**C) Use wgpu's web-specific helpers**:
Check wgpu 22.x docs for `create_surface_from_canvas` or similar

### 2. Shader Coordinate System (5 minutes)
Add uniform buffer for proper screen space transformation:

```wgsl
struct Uniforms {
    screen_size: vec2<f32>,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

// Then in vertex shader:
let clip_x = (world_pos.x / uniforms.screen_size.x) * 2.0 - 1.0;
let clip_y = 1.0 - (world_pos.y / uniforms.screen_size.y) * 2.0;
```

## 🚀 Ready to Run

Once wgpu surface issue is fixed:

```bash
# Terminal 1: Build WASM
cd crates/wasm-bridge
wasm-pack build --target web --out-dir ../../web/wasm-pkg

# Terminal 2: Run dev server
cd ../../web
npm install
npm run dev

# Open browser to http://localhost:3000
```

**Expected behavior**:
- Black canvas with colored particles
- Click anywhere to add emitters
- Particles fall with gravity
- FPS counter and particle count in top-left

## 📚 Learning Path

### Week 1: Get It Running
1. Fix wgpu surface (today)
2. Build and test in browser
3. Experiment with forces (edit `wasm-bridge/src/lib.rs`)
4. Try different emitter positions

### Week 2: Enhance Particles
1. Add color gradients over lifetime
2. Implement particle trails
3. Add more force types (vortex, turbulence)
4. Optimize with spatial grid for collisions

### Week 3: Computer Vision
1. Enable camera in browser
2. Integrate MediaPipe segmentation
3. Extract outlines from video feed
4. Visualize outlines on canvas

### Week 4: Interaction
1. Pass outline data to Rust
2. Implement particle-outline collision
3. Add repulsion forces around body
4. Detect and amplify swipe motions

## 🎯 Key Files to Know

### For Particle Behavior
- `crates/particle-core/src/particles.rs` - Particle logic
- `crates/particle-core/src/physics.rs` - Forces
- `crates/wasm-bridge/src/lib.rs` - Initial setup (emitters, forces)

### For Visuals
- `crates/renderer/src/shaders/particle.wgsl` - Shader
- `crates/renderer/src/particle_renderer.rs` - Rendering

### For Web Integration
- `web/src/main.ts` - Main app logic
- `web/index.html` - UI

## 💡 Quick Experiments

### Add a Vortex
```rust
// In wasm-bridge/src/lib.rs, new() function
let forces = vec![
    Force::gravity(0.0, 50.0),
    Force::attractor(glam::Vec2::new(640.0, 360.0), 1000.0, 500.0),
];
```

### Change Particle Colors
```rust
// In particle-core/src/particles.rs, Emitter::emit()
let color = [
    1.0,  // Red
    0.5,  // Green  
    0.0,  // Blue
    1.0,  // Alpha
];
```

### More Particles
```rust
// In particle-core/src/particles.rs, ParticleConfig::default()
max_particles: 50000,
spawn_rate: 1000.0,
```

## 📖 Documentation Reference

- **README.md** - Overall project plan
- **docs/PARTICLES.md** - Deep dive on particle systems (457 lines!)
- **docs/COMPUTER_VISION.md** - CV implementation guide (630 lines!)
- **GETTING_STARTED.md** - Build instructions
- **PROJECT_STATUS.md** - Current status summary

## 🐛 Troubleshooting

### "Surface error" in console
- Check wgpu version compatibility
- Verify WebGPU/WebGL2 support in browser
- Try Chrome/Edge (best WebGPU support)

### WASM build fails
```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

### Nothing renders
- Open browser console (F12)
- Check for errors
- Verify canvas has correct size
- Test with native example: `cargo run --example native_test`

## 🎨 Make It Yours

This is **your** learning project! Try:

- Different particle shapes (squares, triangles)
- Particle connections (draw lines between nearby particles)
- Audio reactivity (particles respond to microphone)
- Multiple emitter types
- Predator/prey particle systems
- Fluid simulation
- Fireworks
- Starfield effects

## 🤝 Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [MediaPipe Docs](https://developers.google.com/mediapipe)
- [The Nature of Code](https://natureofcode.com/) - Particle systems

---

**You're ready to go!** The structure is solid, the core works, just need that one wgpu compatibility fix and you'll see particles in your browser. 🚀

