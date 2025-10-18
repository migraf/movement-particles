# Getting Started

Quick start guide for building and running the Movement Particles project.

## Prerequisites

1. **Rust** (latest stable)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **wasm-pack** (for building WASM)
   ```bash
   cargo install wasm-pack
   ```

3. **Node.js** (v18 or later) and npm
   ```bash
   # Using nvm (recommended)
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.0/install.sh | bash
   nvm install 18
   ```

4. **WASM target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

## Building the Project

### Option 1: Using the build script (recommended)

```bash
chmod +x build.sh
./build.sh
```

### Option 2: Manual build

1. Build the WASM module:
   ```bash
   cd crates/wasm-bridge
   wasm-pack build --target web --out-dir ../../web/wasm-pkg
   cd ../..
   ```

2. Install web dependencies:
   ```bash
   cd web
   npm install
   ```

## Running the Development Server

```bash
cd web
npm run dev
```

Open your browser to `http://localhost:3000`

## Project Structure

```
movement-particles/
├── crates/
│   ├── particle-core/    # Core particle engine (Rust)
│   ├── renderer/         # Rendering engine (wgpu)
│   └── wasm-bridge/      # WASM bindings
├── web/
│   ├── src/             # TypeScript frontend
│   └── index.html       # Main HTML file
└── docs/                # Documentation
```

## Next Steps

1. **Test the basic particle system**: Click anywhere on the canvas to add particle emitters
2. **Enable the camera**: Click "Enable Camera" button (CV integration not yet implemented)
3. **Explore the code**: Start with `crates/particle-core/src/particles.rs`

## Troubleshooting

### WASM build fails
- Ensure you have the wasm32-unknown-unknown target: `rustup target add wasm32-unknown-unknown`
- Try cleaning: `cargo clean && ./build.sh`

### Camera not working
- Grant camera permissions in your browser
- Use HTTPS or localhost (required for getUserMedia API)

### Canvas is black
- Check browser console for errors
- Ensure WebGPU/WebGL2 is supported (try Chrome/Edge/Firefox)

## Development Tips

- Use `cargo check` to quickly validate Rust code without building
- Use `cargo clippy` for Rust linting
- Enable Rust-Analyzer in your IDE for better autocompletion
- Web changes auto-reload with Vite's hot module replacement

## Building for Production

```bash
# Build optimized WASM
cd crates/wasm-bridge
wasm-pack build --target web --release --out-dir ../../web/wasm-pkg

# Build optimized web bundle
cd ../../web
npm run build

# Output will be in web/dist/
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [Project Documentation](./docs/)

