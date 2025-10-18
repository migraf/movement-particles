# Movement Particles

An interactive browser-based application that combines real-time person detection with particle physics, allowing users to physically interact with particles using their body movements captured through a webcam.

## Project Overview

This project creates an immersive visual experience where:
- A webcam feed captures the user in real-time
- Computer vision algorithms detect and extract person outlines
- A particle system renders thousands of interactive particles
- Detected body outlines act as physical barriers/forces for particles
- Users can "push" particles with their movements, creating dynamic visual effects

## Goals

- **Primary**: Improve Rust programming skills by implementing core systems in Rust/WebAssembly
- **Secondary**: Create a visually stunning, performant interactive experience
- **Learning**: Explore the intersection of computer vision, physics simulation, and real-time graphics in a web environment

## Technology Stack

### Core Technologies
- **Rust** - Primary implementation language
- **WebAssembly (WASM)** - Compile Rust to run in the browser
- **WebGL/WebGPU** - GPU-accelerated rendering via `wgpu` (Rust graphics library)
- **JavaScript/TypeScript** - Browser API integration and UI

### Graphics & Rendering
- **wgpu** - Rust graphics library targeting WebGPU/WebGL
- **winit** (or web-sys for direct web integration) - Window/canvas management
- Custom particle rendering pipeline

### Computer Vision
Options to evaluate (see COMPUTER_VISION.md for details):
- **MediaPipe** - Google's efficient person segmentation (JavaScript/WASM)
- **TensorFlow.js** - BodyPix or MoveNet models
- **ONNX Runtime Web** - Run pre-trained models with Rust compiled to WASM
- **rust-cv** ecosystem - Native Rust CV libraries (if feasible)

### Build Tools
- **wasm-pack** - Build Rust for the web
- **trunk** or **wasm-bindgen** - WASM tooling and JS interop
- **npm/pnpm** - Frontend dependency management

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    Browser Frontend                      │
├─────────────────────────────────────────────────────────┤
│                                                           │
│  ┌──────────────┐    ┌──────────────┐   ┌────────────┐ │
│  │  Video Input │───▶│   CV Module  │──▶│  Outline   │ │
│  │  (getUserMedia)│   │  (Person Det)│   │  Extractor │ │
│  └──────────────┘    └──────────────┘   └─────┬──────┘ │
│                                                 │        │
│                                                 ▼        │
│  ┌──────────────────────────────────────────────────┐  │
│  │         Rust/WASM Core Engine                     │  │
│  ├──────────────────────────────────────────────────┤  │
│  │  • Particle System (physics, lifecycle)          │  │
│  │  • Collision Detection (outline ↔ particles)     │  │
│  │  • Force Application (repulsion, flow)           │  │
│  │  • Spatial Partitioning (optimization)           │  │
│  └────────────────────┬─────────────────────────────┘  │
│                       │                                 │
│                       ▼                                 │
│  ┌──────────────────────────────────────────────────┐  │
│  │         Rendering Pipeline (wgpu/WebGPU)         │  │
│  ├──────────────────────────────────────────────────┤  │
│  │  • Particle Renderer (instanced rendering)       │  │
│  │  • Outline Renderer (stroke/fill)                │  │
│  │  • Post-processing (blur, glow effects)          │  │
│  └──────────────────────────────────────────────────┘  │
│                                                           │
└─────────────────────────────────────────────────────────┘
```

## Project Structure

```
movement-particles/
├── crates/
│   ├── core/              # Core particle engine (Rust)
│   │   ├── particles/     # Particle system implementation
│   │   ├── physics/       # Physics simulation
│   │   └── collision/     # Collision detection
│   ├── renderer/          # Rendering engine (wgpu)
│   └── wasm-bridge/       # WASM bindings and JS interop
├── web/
│   ├── src/
│   │   ├── cv/           # Computer vision integration
│   │   ├── ui/           # User interface
│   │   └── main.ts       # Application entry point
│   ├── public/
│   └── package.json
├── docs/
│   ├── PARTICLES.md      # Particle system design
│   └── COMPUTER_VISION.md # CV implementation notes
├── examples/             # Standalone examples/tests
└── README.md
```

## Development Phases

### Phase 1: Foundation (Weeks 1-2)
- [ ] Set up Rust workspace with multiple crates
- [ ] Configure WASM build pipeline (wasm-pack, trunk)
- [ ] Create basic web frontend with video capture
- [ ] Implement minimal particle system in Rust
- [ ] Set up wgpu rendering pipeline for particles
- [ ] Verify WASM compilation and browser execution

### Phase 2: Particle System (Weeks 2-3)
- [ ] Implement comprehensive particle physics (velocity, acceleration, forces)
- [ ] Add particle lifecycle management (spawn, update, death)
- [ ] Create spatial partitioning (quadtree/grid) for collision optimization
- [ ] Implement various particle behaviors (gravity, drag, flow fields)
- [ ] Performance optimization (SIMD, parallel processing where possible)
- [ ] Visual effects (trails, glow, size variation)

### Phase 3: Computer Vision Integration (Weeks 3-4)
- [ ] Evaluate and select CV solution (MediaPipe vs TensorFlow.js vs ONNX)
- [ ] Integrate person detection into web frontend
- [ ] Extract clean outline/contour data from segmentation
- [ ] Convert outline to collision geometry (polygon simplification)
- [ ] Pass outline data from JS to Rust/WASM efficiently
- [ ] Visualize outlines alongside particles

### Phase 4: Interaction System (Weeks 4-5)
- [ ] Implement particle-outline collision detection
- [ ] Add force fields around outline edges
- [ ] Create repulsion/attraction mechanics
- [ ] Implement "swiping" detection (velocity-based forces)
- [ ] Fine-tune physics parameters for satisfying interaction
- [ ] Add particle response to different body parts/movements

### Phase 5: Polish & Effects (Week 6)
- [ ] Add visual polish (particle color schemes, gradients)
- [ ] Implement post-processing effects (glow, motion blur)
- [ ] Create UI controls (particle count, physics params, CV settings)
- [ ] Add performance monitoring and FPS counter
- [ ] Optimize rendering pipeline (instancing, batching)
- [ ] Test on various devices and browsers

### Phase 6: Advanced Features (Optional)
- [ ] Multiple person support
- [ ] Particle types with different behaviors
- [ ] Audio reactivity (particles respond to sound)
- [ ] Recording/screenshot functionality
- [ ] Presets and save/load settings
- [ ] Deploy as static web app

## Key Challenges

1. **Performance**: Real-time CV + physics simulation + rendering at 60fps
2. **JS ↔ Rust Boundary**: Efficient data transfer between JS (CV) and WASM (physics)
3. **Collision Complexity**: Fast particle-to-outline collision detection
4. **Browser Compatibility**: WebGPU fallback to WebGL, camera permissions
5. **WASM Optimization**: Keeping binary size reasonable, maximizing performance

## Performance Targets

- **60 FPS** with 10,000+ particles on modern hardware
- **<100ms** latency from movement to particle response
- **<5MB** initial WASM bundle size
- Support for **1-2 concurrent person detections**

## Getting Started

(To be filled in during Phase 1)

```bash
# Install Rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack trunk

# Install Node dependencies
cd web
npm install

# Build and run development server
trunk serve
```

## Resources

### Rust + WASM
- [The wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)

### Computer Vision
- [MediaPipe Solutions](https://developers.google.com/mediapipe/solutions/vision/pose_landmarker)
- [TensorFlow.js Models](https://www.tensorflow.org/js/models)
- [ONNX Runtime Web](https://onnxruntime.ai/docs/tutorials/web/)

### Particle Systems
- [GPU Gems 3: Chapter 23](https://developer.nvidia.com/gpugems/gpugems3/part-iv-image-effects/chapter-23-high-speed-screen-particles)
- [The Art of Fluid Animation](https://www.joshondesign.com/2014/04/29/webgl_particles)

## License

MIT

## Author

Built as a Rust learning project exploring real-time graphics and computer vision.

