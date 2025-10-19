//! Rendering engine using wgpu for WebGPU/WebGL
//! 
//! This crate handles all GPU-accelerated rendering including:
//! - Particle rendering with instancing
//! - Outline visualization
//! - Post-processing effects

pub mod particle_renderer;

pub use particle_renderer::ParticleRenderer;

/// Main renderer state
pub struct Renderer {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: (u32, u32),
}

impl Renderer {
    /// Creates a new renderer for the given surface
    pub async fn new(surface: wgpu::Surface<'static>, width: u32, height: u32) -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .ok_or_else(|| {
                "Failed to find an appropriate GPU adapter. \
                Your browser or system may not support WebGPU/WebGL2.".to_string()
            })?;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    memory_hints: Default::default(),
                },
                None,
            )
            .await
            .map_err(|e| format!("Failed to create GPU device: {}", e))?;

        let surface_caps = surface.get_capabilities(&adapter);
        
        if surface_caps.formats.is_empty() {
            return Err("No surface formats available".to_string());
        }

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        if surface_caps.alpha_modes.is_empty() {
            return Err("No alpha modes available".to_string());
        }

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: (width, height),
        })
    }

    /// Resizes the render surface
    pub fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.size = (width, height);
            self.config.width = width;
            self.config.height = height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    /// Begins a new render pass
    pub fn begin_frame(&self) -> Result<(wgpu::SurfaceTexture, wgpu::TextureView), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        Ok((output, view))
    }
}
