//! WebAssembly bridge for browser integration
//! 
//! This crate provides the JavaScript API for the particle system

use particle_core::{ParticleSystem, Emitter, Force, Outline};
use renderer::{Renderer, ParticleRenderer};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

/// Main application state
#[wasm_bindgen]
pub struct App {
    particle_system: ParticleSystem,
    renderer: Option<Renderer>,
    particle_renderer: Option<ParticleRenderer>,
    forces: Vec<Force>,
    outline: Option<Outline>,
    last_time: f64,
}

#[wasm_bindgen]
impl App {
    /// Creates a new application instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();
        console_log!("Initializing Movement Particles...");

        let mut particle_system = ParticleSystem::new();
        
        // Add a default emitter
        let emitter = Emitter::new(glam::Vec2::new(640.0, 360.0));
        particle_system.add_emitter(emitter);

        // Add default forces
        let forces = vec![
            Force::gravity(0.0, 50.0),
        ];

        Self {
            particle_system,
            renderer: None,
            particle_renderer: None,
            forces,
            outline: None,
            last_time: 0.0,
        }
    }

    /// Initializes the renderer with a canvas element
    pub async fn init_renderer(&mut self, canvas: web_sys::HtmlCanvasElement, width: u32, height: u32) -> Result<(), JsValue> {
        console_log!("Initializing renderer {}x{}", width, height);

        // Create wgpu instance - prefer WebGPU, fallback to WebGL
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::BROWSER_WEBGPU | wgpu::Backends::GL,
            ..Default::default()
        });

        console_log!("Created wgpu instance");

        // Create surface from canvas - WASM-specific approach
        let surface = instance
            .create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()))
            .map_err(|e| {
                let err_msg = format!("Failed to create surface: {:?}\n\nPlease ensure:\n1. Your browser supports WebGPU\n2. Canvas is not already in use\n3. Try Chrome/Edge with WebGPU enabled", e);
                console_log!("{}", err_msg);
                JsValue::from_str(&err_msg)
            })?;

        console_log!("Created surface");

        let renderer = Renderer::new(surface, width, height).await;
        let particle_renderer = ParticleRenderer::new(
            &renderer.device,
            renderer.config.format,
            self.particle_system.config.max_particles,
        );

        self.renderer = Some(renderer);
        self.particle_renderer = Some(particle_renderer);

        console_log!("Renderer initialized successfully");
        Ok(())
    }

    /// Updates the particle system for one frame
    #[wasm_bindgen]
    pub fn update(&mut self, timestamp: f64) {
        let dt = if self.last_time == 0.0 {
            0.016 // First frame, assume 60fps
        } else {
            ((timestamp - self.last_time) / 1000.0).min(0.1) // Cap at 100ms
        };
        self.last_time = timestamp;

        // Update particle system
        self.particle_system.update(dt as f32, &self.forces);
    }

    /// Renders the current frame
    #[wasm_bindgen]
    pub fn render(&mut self) -> Result<(), JsValue> {
        let renderer = self.renderer.as_ref().ok_or("Renderer not initialized")?;
        let particle_renderer = self.particle_renderer.as_ref().ok_or("Particle renderer not initialized")?;

        let (output, view) = match renderer.begin_frame() {
            Ok(frame) => frame,
            Err(wgpu::SurfaceError::Lost) => {
                console_log!("Surface lost, reconfiguring...");
                return Ok(());
            }
            Err(wgpu::SurfaceError::OutOfMemory) => {
                return Err("Out of memory".into());
            }
            Err(e) => {
                console_log!("Surface error: {:?}", e);
                return Ok(());
            }
        };

        let mut encoder = renderer.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        particle_renderer.render(
            &mut encoder,
            &view,
            &renderer.queue,
            &self.particle_system.particles,
        );

        renderer.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    /// Updates the outline from computer vision data
    #[wasm_bindgen]
    pub fn update_outline(&mut self, points: &[f32]) {
        if points.len() < 4 {
            self.outline = None;
            return;
        }

        let outline_points: Vec<glam::Vec2> = points
            .chunks_exact(2)
            .map(|chunk| glam::Vec2::new(chunk[0], chunk[1]))
            .collect();

        self.outline = Some(Outline::from_points(outline_points));
        
        // TODO: Add outline-based forces to self.forces
    }

    /// Resizes the renderer
    #[wasm_bindgen]
    pub fn resize(&mut self, width: u32, height: u32) {
        if let Some(renderer) = &mut self.renderer {
            renderer.resize(width, height);
        }
    }

    /// Returns the current particle count
    #[wasm_bindgen]
    pub fn particle_count(&self) -> usize {
        self.particle_system.particle_count()
    }

    /// Adds an emitter at the given position
    #[wasm_bindgen]
    pub fn add_emitter(&mut self, x: f32, y: f32) {
        let emitter = Emitter::new(glam::Vec2::new(x, y));
        self.particle_system.add_emitter(emitter);
    }
}
