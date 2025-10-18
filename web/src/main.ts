/**
 * Main entry point for the Movement Particles web application
 */

// @ts-ignore - WASM module will be available after build
import init, { App } from '../wasm-pkg/wasm_bridge';

interface Stats {
  particleCount: HTMLElement;
  fps: HTMLElement;
}

class Application {
  private app: App | null = null;
  private canvas: HTMLCanvasElement;
  private video: HTMLVideoElement;
  private stats: Stats;
  private cameraEnabled = false;
  private lastFrameTime = 0;
  private frameCount = 0;
  private fpsUpdateTime = 0;

  constructor() {
    this.canvas = document.getElementById('canvas') as HTMLCanvasElement;
    this.video = document.getElementById('video') as HTMLVideoElement;
    this.stats = {
      particleCount: document.getElementById('particle-count')!,
      fps: document.getElementById('fps')!,
    };

    this.setupEventListeners();
  }

  async initialize() {
    try {
      // Check WebGPU support
      if (!navigator.gpu) {
        const msg = 'WebGPU is not supported in your browser.\n\n' +
                    'Please use:\n' +
                    '- Chrome/Edge 113+ with WebGPU enabled\n' +
                    '- Chrome Canary with chrome://flags/#enable-unsafe-webgpu\n\n' +
                    'Or check: https://caniuse.com/webgpu';
        alert(msg);
        throw new Error('WebGPU not supported');
      }

      console.log('✓ WebGPU is supported');
      console.log('Initializing WASM module...');
      await init();

      console.log('Creating application...');
      this.app = new App();

      // Setup canvas
      this.resizeCanvas();
      window.addEventListener('resize', () => this.resizeCanvas());

      console.log('Initializing renderer...');
      await this.app.init_renderer(this.canvas, this.canvas.width, this.canvas.height);

      // Hide loading screen
      const loading = document.getElementById('loading');
      if (loading) loading.style.display = 'none';

      // Show UI
      const ui = document.getElementById('ui');
      if (ui) ui.style.display = 'block';

      // Start render loop
      this.startRenderLoop();

      console.log('Application initialized successfully!');
    } catch (error) {
      console.error('Failed to initialize application:', error);
      
      // Show error in UI
      const loading = document.getElementById('loading');
      if (loading) {
        loading.innerHTML = `
          <h2 style="color: #ff4444;">⚠️ Initialization Failed</h2>
          <p style="max-width: 500px; margin: 20px auto; text-align: left;">
            ${error instanceof Error ? error.message : String(error)}
          </p>
          <p style="color: #aaa; font-size: 14px;">Check the browser console for more details</p>
        `;
      }
    }
  }

  private setupEventListeners() {
    const cameraBtn = document.getElementById('camera-btn') as HTMLButtonElement;
    cameraBtn.addEventListener('click', () => this.toggleCamera());

    // Add emitter on click
    this.canvas.addEventListener('click', (e) => {
      if (this.app) {
        const rect = this.canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = e.clientY - rect.top;
        this.app.add_emitter(x, y);
      }
    });
  }

  private resizeCanvas() {
    const dpr = window.devicePixelRatio || 1;
    this.canvas.width = window.innerWidth * dpr;
    this.canvas.height = window.innerHeight * dpr;
    this.canvas.style.width = `${window.innerWidth}px`;
    this.canvas.style.height = `${window.innerHeight}px`;

    if (this.app) {
      this.app.resize(this.canvas.width, this.canvas.height);
    }
  }

  private async toggleCamera() {
    const btn = document.getElementById('camera-btn') as HTMLButtonElement;
    
    if (!this.cameraEnabled) {
      try {
        btn.disabled = true;
        btn.textContent = 'Enabling...';

        const stream = await navigator.mediaDevices.getUserMedia({
          video: {
            width: { ideal: 640 },
            height: { ideal: 480 },
            facingMode: 'user',
          },
        });

        this.video.srcObject = stream;
        await this.video.play();

        this.cameraEnabled = true;
        btn.textContent = 'Disable Camera';
        btn.disabled = false;

        console.log('Camera enabled');
        // TODO: Start computer vision processing
      } catch (error) {
        console.error('Failed to enable camera:', error);
        alert('Failed to access camera. Please grant permission and try again.');
        btn.textContent = 'Enable Camera';
        btn.disabled = false;
      }
    } else {
      // Disable camera
      const stream = this.video.srcObject as MediaStream;
      if (stream) {
        stream.getTracks().forEach(track => track.stop());
      }
      this.video.srcObject = null;
      this.cameraEnabled = false;
      btn.textContent = 'Enable Camera';
    }
  }

  private startRenderLoop() {
    const render = (timestamp: number) => {
      if (this.app) {
        // Update
        this.app.update(timestamp);

        // Render
        try {
          this.app.render();
        } catch (error) {
          console.error('Render error:', error);
        }

        // Update stats
        this.updateStats(timestamp);
      }

      requestAnimationFrame(render);
    };

    requestAnimationFrame(render);
  }

  private updateStats(timestamp: number) {
    if (!this.app) return;

    // Update particle count
    this.stats.particleCount.textContent = this.app.particle_count().toString();

    // Calculate FPS
    this.frameCount++;
    if (timestamp - this.fpsUpdateTime > 1000) {
      const fps = Math.round((this.frameCount * 1000) / (timestamp - this.fpsUpdateTime));
      this.stats.fps.textContent = fps.toString();
      this.frameCount = 0;
      this.fpsUpdateTime = timestamp;
    }
  }
}

// Initialize application when page loads
const app = new Application();
app.initialize();

