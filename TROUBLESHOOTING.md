# Troubleshooting Guide

## WebGPU/WebGL Initialization Errors

If you're seeing errors like "Failed to create surface" or "canvas.getContext() returned null", follow this guide.

### Quick Fixes

1. **Use a Compatible Browser**
   - ✅ Chrome/Edge 113+ (recommended)
   - ✅ Firefox 113+ 
   - ✅ Chrome Canary (latest features)

2. **Enable WebGPU Support**
   
   **Chrome/Edge:**
   - Open `chrome://flags/#enable-unsafe-webgpu`
   - Set to "Enabled"
   - Restart browser
   
   **Firefox:**
   - Open `about:config`
   - Set `dom.webgpu.enabled` to `true`
   - Restart browser

3. **Clear Cache**
   ```bash
   # Hard refresh in browser
   Ctrl+Shift+R (Windows/Linux)
   Cmd+Shift+R (Mac)
   ```

4. **Check GPU Acceleration**
   - Chrome: Visit `chrome://gpu`
   - Ensure "Graphics Feature Status" shows "Hardware accelerated" for WebGPU/WebGL2

### Common Issues

#### Issue: "WebGPU not supported"
**Solution:** Your browser doesn't support WebGPU yet. The application will automatically fall back to WebGL2 if available.

#### Issue: "Canvas already in use"
**Solution:** Another application or tab might be using the GPU. Close other GPU-intensive tabs and reload.

#### Issue: "Failed to find an appropriate GPU adapter"
**Solution:** 
- Update your graphics drivers
- Ensure hardware acceleration is enabled in browser settings
- Try a different browser

### Testing Your Setup

1. **Check Browser Compatibility**
   - Visit: https://caniuse.com/webgpu
   - Visit: https://webgpu.io (try the demos)

2. **Start the Development Server**
   ```bash
   cd /home/micha/projects/movement-particles
   ./build.sh
   ```

3. **Open in Browser**
   - Navigate to: http://localhost:3000
   - Check the browser console (F12) for detailed logs

### Expected Console Output (Success)

```
Initializing WASM module...
Creating application...
Initializing renderer...
Canvas dimensions: 2560 x 1440
✓ WebGPU is available (or using WebGL fallback)
Created wgpu instance with backends: GL | BROWSER_WEBGPU
Created surface successfully
Renderer created successfully
Particle renderer created successfully
✓ Renderer initialized successfully!
Application initialized successfully!
```

### Expected Console Output (Error)

If you see errors, the new error messages will provide specific troubleshooting steps directly in the UI.

### Still Having Issues?

1. **Check System Requirements**
   - Modern GPU (< 5 years old recommended)
   - Updated GPU drivers
   - Hardware acceleration enabled

2. **Try These Browsers (in order)**
   - Chrome Canary (best WebGPU support)
   - Chrome 113+
   - Edge 113+
   - Firefox 113+ (with dom.webgpu.enabled)

3. **Verify Installation**
   ```bash
   # Ensure WASM module is built
   ls -la web/wasm-pkg/
   # Should see: wasm_bridge_bg.wasm and other files
   
   # Ensure dependencies are installed
   cd web && npm install
   ```

## Performance Issues

If the app runs but is slow:

1. **Check FPS in UI** - Should be 60 FPS
2. **Reduce Particle Count** - Click fewer times to add fewer emitters
3. **Close Other Tabs** - Free up GPU resources
4. **Check GPU Usage**
   - Task Manager (Windows): Performance > GPU
   - Activity Monitor (Mac): Window > GPU History

## Camera Issues

If camera doesn't work:

1. **Grant Camera Permission** - Allow when prompted
2. **Check Camera Access** - Visit browser settings
3. **Close Other Apps** - Camera might be in use by another app

## Need More Help?

- Check browser console for detailed error messages
- Look for console.log statements showing initialization steps
- The new error UI will provide specific solutions based on the error type

