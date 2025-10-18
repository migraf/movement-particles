# Computer Vision Implementation

Technical considerations for real-time person detection and outline extraction in a browser environment.

## Requirements

The computer vision system must:
1. **Detect** one or more persons in webcam feed
2. **Extract** clean outlines/contours of detected persons
3. **Run in real-time** at minimum 30fps (preferably 60fps)
4. **Work in browser** with reasonable hardware requirements
5. **Integrate smoothly** with Rust/WASM particle system
6. **Handle occlusion** and partial visibility gracefully

## Solution Approaches

### Option 1: MediaPipe (Recommended for Phase 1)

**Google's MediaPipe** provides highly optimized ML solutions for web.

**Key Solutions:**
- **Image Segmentation** - Separates person from background
- **Pose Detection** - 33 landmark points for body tracking
- **Object Detection** - General person detection

**Pros:**
- Excellent performance (optimized with SIMD, WebGL acceleration)
- Runs entirely in browser (no server needed)
- Well-documented, actively maintained
- Multiple quality/speed trade-offs
- Free and open source

**Cons:**
- JavaScript/TypeScript only (no direct Rust integration)
- Need to bridge data to WASM
- Less control over model internals

**Implementation:**
```typescript
import { ImageSegmenter, FilesetResolver } from '@mediapipe/tasks-vision';

// Initialize
const vision = await FilesetResolver.forVisionTasks(
  "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision/wasm"
);

const imageSegmenter = await ImageSegmenter.createFromOptions(vision, {
  baseOptions: {
    modelAssetPath: "https://storage.googleapis.com/mediapipe-models/image_segmenter/selfie_segmenter/float16/latest/selfie_segmenter.tflite",
    delegate: "GPU"
  },
  outputCategoryMask: true,
  outputConfidenceMasks: false
});

// Per frame
function processFrame(videoElement: HTMLVideoElement) {
  imageSegmenter.segmentForVideo(videoElement, performance.now(), (result) => {
    const mask = result.categoryMask;
    const outline = extractOutline(mask);
    // Send to WASM
    wasmModule.update_outline(outline);
  });
}
```

### Option 2: TensorFlow.js

**Models Available:**
- **BodyPix** - Person segmentation (older, less efficient)
- **MoveNet** - Pose estimation (17 keypoints)
- **BlazePose** - Similar to MediaPipe

**Pros:**
- Flexible, large ecosystem
- Can use custom models
- Good documentation

**Cons:**
- Generally slower than MediaPipe
- BodyPix is outdated
- Larger bundle sizes

**Implementation:**
```typescript
import * as bodySegmentation from '@tensorflow-models/body-segmentation';

const model = bodySegmentation.SupportedModels.MediaPipeSelfieSegmentation;
const segmenterConfig = {
  runtime: 'tfjs',
  modelType: 'general'
};

const segmenter = await bodySegmentation.createSegmenter(model, segmenterConfig);

async function processFrame(video: HTMLVideoElement) {
  const segmentation = await segmenter.segmentPeople(video, {
    flipHorizontal: false,
    multiSegmentation: false,
    segmentBodyParts: false
  });
  
  const mask = await bodySegmentation.toBinaryMask(
    segmentation,
    {r: 0, g: 0, b: 0, a: 0},
    {r: 255, g: 255, b: 255, a: 255}
  );
  
  const outline = extractOutlineFromMask(mask);
}
```

### Option 3: ONNX Runtime Web + Rust

Run ONNX models in browser, potentially with Rust bindings.

**Pros:**
- Can use any ONNX-converted model
- Potential for Rust integration via wasm-bindgen
- Good performance

**Cons:**
- More setup required
- Need to find/convert appropriate models
- Less out-of-the-box than MediaPipe

**Models to Consider:**
- YOLOv8 pose estimation
- U-Net segmentation variants
- Mobile-optimized person detectors

### Option 4: Pure Rust CV (Not Recommended for Phase 1)

Use Rust libraries like `rust-cv`, `imageproc`, `opencv-rust`.

**Pros:**
- Full Rust implementation
- Complete control

**Cons:**
- No pre-trained person detection models readily available
- Would need to compile existing models to WASM
- Significantly more work
- Likely worse performance than optimized JS solutions

**Verdict:** Use for learning, but not for production person detection.

## Outline Extraction

Once we have a segmentation mask, extract a clean outline.

### Algorithm 1: Contour Finding (OpenCV-style)

```typescript
interface Point {
  x: number;
  y: number;
}

function extractOutline(mask: ImageData): Point[] {
  const { width, height, data } = mask;
  const visited = new Set<number>();
  const outline: Point[] = [];
  
  // Find first foreground pixel on edge
  let startX = -1, startY = -1;
  outer: for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const idx = (y * width + x) * 4;
      if (data[idx] > 128) { // Foreground
        // Check if it's an edge pixel
        if (isEdgePixel(x, y, data, width, height)) {
          startX = x;
          startY = y;
          break outer;
        }
      }
    }
  }
  
  if (startX === -1) return []; // No person found
  
  // Trace contour using Moore-Neighbor algorithm
  let x = startX, y = startY;
  const directions = [
    [-1, -1], [0, -1], [1, -1],
    [1, 0], [1, 1], [0, 1],
    [-1, 1], [-1, 0]
  ];
  
  do {
    outline.push({ x, y });
    visited.add(y * width + x);
    
    // Find next edge pixel
    let found = false;
    for (const [dx, dy] of directions) {
      const nx = x + dx;
      const ny = y + dy;
      if (isEdgePixel(nx, ny, data, width, height) && 
          !visited.has(ny * width + nx)) {
        x = nx;
        y = ny;
        found = true;
        break;
      }
    }
    if (!found) break;
  } while (x !== startX || y !== startY);
  
  return outline;
}

function isEdgePixel(x: number, y: number, data: Uint8ClampedArray, 
                     width: number, height: number): boolean {
  if (x < 0 || x >= width || y < 0 || y >= height) return false;
  
  const idx = (y * width + x) * 4;
  if (data[idx] < 128) return false; // Background
  
  // Check if any neighbor is background
  for (let dy = -1; dy <= 1; dy++) {
    for (let dx = -1; dx <= 1; dx++) {
      const nx = x + dx, ny = y + dy;
      if (nx >= 0 && nx < width && ny >= 0 && ny < height) {
        const nidx = (ny * width + nx) * 4;
        if (data[nidx] < 128) return true; // Adjacent to background
      }
    }
  }
  return false;
}
```

### Algorithm 2: Marching Squares

Convert binary mask to vector outline using marching squares algorithm.

**Advantages:**
- Cleaner results
- Natural vector representation
- Easy to simplify

**Libraries:**
- `d3-contour` - JavaScript implementation
- Implement custom version

### Simplification

Raw outline has thousands of points. Simplify for performance.

**Douglas-Peucker Algorithm:**
```typescript
function simplifyOutline(points: Point[], epsilon: number): Point[] {
  if (points.length < 3) return points;
  
  // Find point with maximum distance from line
  let maxDist = 0;
  let maxIndex = 0;
  const end = points.length - 1;
  
  for (let i = 1; i < end; i++) {
    const dist = perpendicularDistance(
      points[i],
      points[0],
      points[end]
    );
    if (dist > maxDist) {
      maxDist = dist;
      maxIndex = i;
    }
  }
  
  // If max distance is greater than epsilon, recursively simplify
  if (maxDist > epsilon) {
    const left = simplifyOutline(points.slice(0, maxIndex + 1), epsilon);
    const right = simplifyOutline(points.slice(maxIndex), epsilon);
    return [...left.slice(0, -1), ...right];
  }
  
  return [points[0], points[end]];
}
```

**Target:** Reduce from 1000s of points to 50-200 points for collision detection.

## Data Transfer: JavaScript → Rust/WASM

Efficient communication is critical for performance.

### Approach 1: Typed Arrays (Recommended)

```typescript
// JavaScript side
const outlineData = new Float32Array(outline.length * 2);
for (let i = 0; i < outline.length; i++) {
  outlineData[i * 2] = outline[i].x;
  outlineData[i * 2 + 1] = outline[i].y;
}

// Pass pointer to WASM
wasmModule.update_outline(outlineData);
```

```rust
// Rust side
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn update_outline(data: &[f32]) {
    let points: Vec<Vec2> = data
        .chunks_exact(2)
        .map(|chunk| Vec2::new(chunk[0], chunk[1]))
        .collect();
    
    // Convert to collision-friendly format
    let outline = Outline::from_points(points);
    
    // Update particle system
    PARTICLE_SYSTEM.lock().unwrap().set_outline(outline);
}
```

### Approach 2: Shared Memory (Advanced)

Use `SharedArrayBuffer` for zero-copy data sharing.

**Pros:** Fastest possible transfer
**Cons:** Requires cross-origin isolation, browser support

### Approach 3: JSON (Not Recommended)

Simple but slow - avoid for real-time data.

## Temporal Consistency

Avoid jittery outlines frame-to-frame.

### Smoothing Techniques

1. **Exponential Moving Average**
```typescript
class OutlineSmoothing {
  private history: Point[][] = [];
  private alpha = 0.3; // Smoothing factor
  
  smooth(outline: Point[]): Point[] {
    if (this.history.length === 0) {
      this.history.push(outline);
      return outline;
    }
    
    const prev = this.history[this.history.length - 1];
    const smoothed: Point[] = [];
    
    for (let i = 0; i < Math.min(outline.length, prev.length); i++) {
      smoothed.push({
        x: this.alpha * outline[i].x + (1 - this.alpha) * prev[i].x,
        y: this.alpha * outline[i].y + (1 - this.alpha) * prev[i].y
      });
    }
    
    this.history.push(smoothed);
    if (this.history.length > 5) this.history.shift();
    
    return smoothed;
  }
}
```

2. **Kalman Filter**
   - More sophisticated
   - Better for predictive tracking
   - Overkill for most cases

3. **Point Correspondence**
   - Match points between frames
   - Smooth individual point trajectories
   - More complex but better results

## Velocity Estimation

Track outline movement for "swipe" detection.

```typescript
class VelocityTracker {
  private previousOutline: Point[] | null = null;
  private previousTime: number = 0;
  
  estimateVelocity(outline: Point[], timestamp: number): { x: number, y: number } {
    if (!this.previousOutline) {
      this.previousOutline = outline;
      this.previousTime = timestamp;
      return { x: 0, y: 0 };
    }
    
    const dt = (timestamp - this.previousTime) / 1000; // Convert to seconds
    if (dt === 0) return { x: 0, y: 0 };
    
    // Calculate center of mass movement
    const prevCenter = this.getCentroid(this.previousOutline);
    const currCenter = this.getCentroid(outline);
    
    const velocity = {
      x: (currCenter.x - prevCenter.x) / dt,
      y: (currCenter.y - prevCenter.y) / dt
    };
    
    this.previousOutline = outline;
    this.previousTime = timestamp;
    
    return velocity;
  }
  
  private getCentroid(points: Point[]): Point {
    const sum = points.reduce(
      (acc, p) => ({ x: acc.x + p.x, y: acc.y + p.y }),
      { x: 0, y: 0 }
    );
    return {
      x: sum.x / points.length,
      y: sum.y / points.length
    };
  }
}
```

## Multi-Person Support

Handle multiple people in frame.

### Detection
Most segmentation models can output separate masks per person.

```typescript
const segmentation = await segmenter.segmentPeople(video, {
  multiSegmentation: true
});

const outlines = segmentation.map(seg => extractOutline(seg.mask));

// Send all outlines to WASM
wasmModule.update_outlines(outlines);
```

### Collision Handling
- Maintain separate outline structures per person
- Check particles against all outlines
- May need to increase repulsion distance to prevent clipping

## Performance Optimization

### Model Selection
- **Speed tier** (30+ fps on mid-range hardware)
  - MediaPipe selfie_segmenter (landscape model)
  - Lower resolution input (320x240)
  
- **Quality tier** (for powerful hardware)
  - MediaPipe general model
  - Higher resolution (640x480)

### Frame Skipping
Don't process every video frame for CV.

```typescript
let frameCount = 0;
const CV_UPDATE_INTERVAL = 2; // Process every 2nd frame

function onVideoFrame() {
  frameCount++;
  
  if (frameCount % CV_UPDATE_INTERVAL === 0) {
    processComputerVision(video);
  }
  
  // Always update particles at full frame rate
  updateParticles();
  render();
  
  requestAnimationFrame(onVideoFrame);
}
```

### Resolution Scaling
Process lower resolution for CV, display at full resolution.

```typescript
const cvCanvas = document.createElement('canvas');
cvCanvas.width = 320;  // Lower resolution
cvCanvas.height = 240;

const displayCanvas = document.querySelector('canvas');
displayCanvas.width = 1280;  // Display resolution
displayCanvas.height = 720;

// Scale outline coordinates to display resolution
const scaleX = displayCanvas.width / cvCanvas.width;
const scaleY = displayCanvas.height / cvCanvas.height;
```

### Web Workers
Run CV processing in separate thread.

```typescript
// main.ts
const cvWorker = new Worker('cv-worker.ts');

cvWorker.postMessage({ video: videoFrame });

cvWorker.onmessage = (e) => {
  const outline = e.data.outline;
  wasmModule.update_outline(outline);
};

// cv-worker.ts
onmessage = async (e) => {
  const outline = await processFrame(e.data.video);
  postMessage({ outline });
};
```

## Coordinate System Considerations

Handle different coordinate spaces:

1. **Video coordinates** - Native video resolution
2. **Canvas coordinates** - Display canvas size
3. **Particle system coordinates** - May be normalized or pixel-based
4. **Outline coordinates** - Extracted from video, need scaling

```typescript
function transformOutlineToParticleSpace(
  outline: Point[],
  videoWidth: number,
  videoHeight: number,
  canvasWidth: number,
  canvasHeight: number
): Point[] {
  return outline.map(p => ({
    x: (p.x / videoWidth) * canvasWidth,
    y: (p.y / videoHeight) * canvasHeight
  }));
}
```

## Testing & Debugging

### Visualization Tools
```typescript
function debugDrawOutline(ctx: CanvasRenderingContext2D, outline: Point[]) {
  ctx.strokeStyle = 'lime';
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.moveTo(outline[0].x, outline[0].y);
  for (let i = 1; i < outline.length; i++) {
    ctx.lineTo(outline[i].x, outline[i].y);
  }
  ctx.closePath();
  ctx.stroke();
  
  // Draw normals for debugging collision
  ctx.strokeStyle = 'red';
  for (let i = 0; i < outline.length; i++) {
    const p1 = outline[i];
    const p2 = outline[(i + 1) % outline.length];
    const normal = calculateNormal(p1, p2);
    const center = { x: (p1.x + p2.x) / 2, y: (p1.y + p2.y) / 2 };
    ctx.beginPath();
    ctx.moveTo(center.x, center.y);
    ctx.lineTo(center.x + normal.x * 20, center.y + normal.y * 20);
    ctx.stroke();
  }
}
```

### Performance Monitoring
```typescript
const stats = {
  cvTime: 0,
  outlineExtractionTime: 0,
  pointCount: 0,
  fps: 0
};

function measurePerformance() {
  const start = performance.now();
  
  // CV processing
  const cvStart = performance.now();
  const mask = runSegmentation();
  stats.cvTime = performance.now() - cvStart;
  
  // Outline extraction
  const outlineStart = performance.now();
  const outline = extractOutline(mask);
  stats.outlineExtractionTime = performance.now() - outlineStart;
  
  stats.pointCount = outline.length;
  
  console.log('CV Stats:', stats);
}
```

## Recommended Implementation Path

**Phase 1:** 
- Use MediaPipe Image Segmentation
- Basic outline extraction (contour tracing)
- Simple Douglas-Peucker simplification
- Direct TypedArray transfer to WASM

**Phase 2:**
- Add smoothing (EMA)
- Implement velocity tracking
- Optimize outline processing

**Phase 3:**
- Multi-person support
- Advanced smoothing (Kalman)
- Web Worker threading

## References

- [MediaPipe Solutions](https://developers.google.com/mediapipe/solutions/vision/image_segmenter)
- [TensorFlow.js Body Segmentation](https://github.com/tensorflow/tfjs-models/tree/master/body-segmentation)
- [Marching Squares Tutorial](https://en.wikipedia.org/wiki/Marching_squares)
- [Douglas-Peucker Algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm)
- [Real-time Computer Vision in the Browser](https://www.youtube.com/watch?v=kQtj9FKlCKw)

