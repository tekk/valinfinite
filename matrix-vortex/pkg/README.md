# 💖 Infinite Real-Time GPU Heart Fractal Zoom (WASM + WebGL2)

A WebAssembly application featuring an infinite real-time fractal zoom into an anatomical glowing heart structure, rendered on the GPU with psychedelic colors and continuous scale-invariant depth.

![Infinite Heart Fractal Zoom Preview](/home/tekk/.gemini/antigravity/brain/58450bae-f7f6-4bf0-89b4-e9b21203d130/preview_desktop.png)

## ✨ Highlights

- **Pure Infinite Zoom (Zero Precision Breakdown)**: Uses Shepard multi-scale logarithmic octave synthesis. It zooms inward forever with no floating-point pixelation, no stutter, and no resets.
- **Heart-Shaped Fractal Dynamics**: Features a central heart cardioid with recursive internal chambers, organic cleft indentation, kaleidoscopic spherical inversion, and orbit traps.
- **Psychedelic Multi-Frequency Color Waves**: Trigonometric cosine color palettes dynamically shifting between electric magenta, cyan, solar gold, emerald, and ultraviolet glow.
- **Living Heartbeat Rhythm**: An anatomical double-pulse ("lub-dub") rhythmic pulsation subtly breathes life into the geometry.
- **Zero Controls**: Unobtrusive, borderless visual art designed for pure immersion.
- **Optimized for High-Performance Mobile WebGL**:
  - Lean WebGL2 fragment shader with 12 iterations per octave (under 300 ALUs/fragment).
  - Dynamic DPR capping (`dpr <= 1.5`) preserves full 60–120 FPS on high-DPI AMOLED screens without thermal throttling.
  - High-performance context flags (`powerPreference: high-performance`, `preserveDrawingBuffer: false`, `antialias: false`).
  - Total WASM binary size is only **~40 KB**.

---

## 🛠️ Project Structure

- `src/lib.rs`: Rust WebAssembly core, WebGL2 context manager, and `requestAnimationFrame` loop.
- `src/shaders.rs`: High-precision WebGL2 vertex and fragment shaders.
- `index.html`: Responsive fullscreen mobile and desktop viewport.
- `style.css`: Minimalist styles, preventing touch interference.
- `pkg/`: Generated WebAssembly binary and JavaScript bindings.

---

## 🚀 Building & Running

### 1. Build WASM
```bash
wasm-pack build --target web --release
```

### 2. Start Local Server
```bash
python3 -m http.server 8088
```

### 3. Open Preview
Navigate to `http://localhost:8088` on desktop or your mobile browser.
