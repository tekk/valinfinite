# 💖 VALINFINITE — Infinite Real-Time GPU Fractal Zoom

A WebAssembly + WebGL2 application featuring continuous, scale-invariant heart fractal zooming with vivid psychedelic colors, zero precision breakdown, and synchronized audio.

🌐 **Live GitHub Pages Portal**: [https://tekk.github.io/valinfinite/](https://tekk.github.io/valinfinite/)
- **V1 (Celestial Heart)**: [https://tekk.github.io/valinfinite/v1](https://tekk.github.io/valinfinite/v1)
- **V2 (Vortex Void)**: [https://tekk.github.io/valinfinite/v2](https://tekk.github.io/valinfinite/v2)

---

## 🌿 Branches

- **`main`**: The responsive entry portal with interactive cards, screenshots, audio player, and GitHub Pages deployments.
- **`v1`**: Standalone codebase for Version 1 (*Celestial Heart*) — serene infinite zoom, gentle light breathing, and celestial pastel spectrum.
- **`v2`**: Standalone codebase for Version 2 (*Vortex Void*) — 2× accelerated pace, continuous camera vortex rotation, deep cosmic dark zones, and high-contrast iridescent filaments.

---

## 🎵 Soundtrack

- **Track**: *Brookes Brothers - Beautiful feat. Robert Owens (Original Mix)*
- Converted via `ffmpeg` to web-friendly `.m4a` (AAC) and `.mp3` for universal streaming across all mobile and desktop browsers.
- Selecting any animation card triggers audio playback automatically.

---

## ⚡ Performance Profiling

- **Mobile Optimization**: Device Pixel Ratio is capped at $\le 1.5\times$ to avoid high-DPI GPU fillrate bottlenecks while preserving crispness on AMOLED screens.
- **ALU Budget**: Lean WebGL2 fragment shader (under 300 ALUs/fragment) runs at a smooth **60–120 FPS**.
- **WASM Size**: Only **~40 KB**, compiled via Rust, `wasm-bindgen`, and `wasm-opt`.

---

## 🛠️ Building & Developing Locally

```bash
# Build WASM for web
wasm-pack build --target web --release

# Serve locally
python3 -m http.server 8088
```
Open `http://localhost:8088` in any modern browser.
