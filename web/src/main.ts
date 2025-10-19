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
      console.log('Initializing WASM module...');
      await init();

      console.log('Creating application...');
      this.app = new App();

      // Setup canvas
      this.resizeCanvas();
      window.addEventListener('resize', () => this.resizeCanvas());

      console.log('Initializing renderer...');
      console.log('Canvas dimensions:', this.canvas.width, 'x', this.canvas.height);
      
      // Check if WebGPU is available
      if (navigator.gpu) {
        console.log('✓ WebGPU is available');
      } else {
        console.log('⚠ WebGPU not available, will try WebGL fallback');
      }

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
        const errorMsg = error instanceof Error ? error.message : String(error);
        const isWebGPUError = errorMsg.includes('webgpu') || errorMsg.includes('surface');
        
        loading.innerHTML = `
          <h2 style="color: #ff4444;">⚠️ Initialization Failed</h2>
          <div style="max-width: 600px; margin: 20px auto; text-align: left; background: rgba(255,255,255,0.1); padding: 20px; border-radius: 8px;">
            <p style="margin-bottom: 15px; color: #fff;">${errorMsg}</p>
            ${isWebGPUError ? `
              <div style="border-top: 1px solid rgba(255,255,255,0.2); padding-top: 15px; margin-top: 15px;">
                <p style="color: #ffa500; font-weight: bold; margin-bottom: 10px;">🔧 Possible Solutions:</p>
                <ul style="color: #ccc; line-height: 1.8; padding-left: 20px;">
                  <li>Use <strong>Chrome 113+</strong> or <strong>Edge 113+</strong></li>
                  <li>Enable WebGPU: <code style="background: rgba(0,0,0,0.3); padding: 2px 6px; border-radius: 3px;">chrome://flags/#enable-unsafe-webgpu</code></li>
                  <li>Try <strong>Chrome Canary</strong> for latest WebGPU support</li>
                  <li>Firefox: Enable <code style="background: rgba(0,0,0,0.3); padding: 2px 6px; border-radius: 3px;">dom.webgpu.enabled</code> in about:config</li>
                </ul>
                <p style="color: #888; font-size: 12px; margin-top: 15px;">Check compatibility: <a href="https://caniuse.com/webgpu" target="_blank" style="color: #4a9eff;">caniuse.com/webgpu</a></p>
              </div>
            ` : ''}
          </div>
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

