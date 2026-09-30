# 💖 VALINFINITE — Infinite Real-Time GPU Fractals

Continuous scale-invariant GPU fractals in WebAssembly and WebGL2 with vivid psychedelic palettes, zero precision degradation, and audio synchronization.

🌐 **Live Portal**: [https://tekk.github.io/valinfinite/](https://tekk.github.io/valinfinite/)
- **Celestial Heart**: [https://tekk.github.io/valinfinite/celestial-heart/](https://tekk.github.io/valinfinite/celestial-heart/)
- **Cyber Matrix Vortex**: [https://tekk.github.io/valinfinite/matrix-vortex/](https://tekk.github.io/valinfinite/matrix-vortex/)
- **Cosmic Mandelbrot**: [https://tekk.github.io/valinfinite/cosmic-mandelbrot/](https://tekk.github.io/valinfinite/cosmic-mandelbrot/)
- **Vortex Void**: [https://tekk.github.io/valinfinite/vortex-void/](https://tekk.github.io/valinfinite/vortex-void/)

---

## 🔬 Mathematical Formulations & Scene Algorithms

### 1. Celestial Heart (Infinite Shepard Zoom)

<div align="center">
  <img src="screenshots/celestial_heart_mobile.gif" alt="Celestial Heart Mobile Preview" width="220">
</div>

#### Logarithmic Octave Synthesis (Shepard Scale)
Infinite scale-invariant zoom is achieved without floating-point precision collapse by synthesizing $N = 4$ overlapping logarithmic octaves. Each octave $k \in \{0, \dots, N-1\}$ scales exponentially by base $S = 3.2$:

$$\phi_k(t) = \operatorname{fract}\left(t \cdot v + \frac{k}{N}\right), \quad s_k(t) = \exp\left(\phi_k(t) \cdot \ln S\right)$$

A quadratic Hanning window eliminates boundary discontinuities:

$$w_k(t) = \left[\frac{1 - \cos(2\pi \phi_k(t))}{2}\right]^2, \quad \mathbf{C}_{\text{total}} = \frac{\sum_{k=0}^{N-1} \mathbf{C}_k \cdot w_k(t)}{\sum_{k=0}^{N-1} w_k(t)}$$

#### Cardioid Cleft Fold & Inversive Geometry
Within each octave, the complex coordinate $z \in \mathbb{C}$ undergoes bilateral symmetry, smooth cardioid cleft folding, and spherical inversion over $M = 12$ iterations:

$$z_x \leftarrow |z_x|, \quad z_y \leftarrow z_y - \alpha \left(\sqrt{|z_x| + \epsilon} - \beta\right)$$

$$z \leftarrow \begin{cases} 
\frac{z}{r_{\min}^2} & |z|^2 < r_{\min}^2 \\ 
\frac{z}{|z|^2} & r_{\min}^2 \le |z|^2 < r_{\max}^2 
\end{cases}$$

---

### 2. Cyber Matrix Vortex (3D Cylindrical Wormhole)

<div align="center">
  <img src="screenshots/matrix_vortex_mobile.gif" alt="Cyber Matrix Vortex Mobile Preview" width="220">
</div>

#### 3D Cylindrical Coordinate Projection
Screen-space coordinates $\mathbf{u} \in [-1, 1]^2$ are projected onto an infinite 3D cylindrical tunnel with depth $z$:

$$r = \|\mathbf{u}\|_2, \quad \phi = \operatorname{atan2}(u_y, u_x), \quad z = \frac{1}{r + \epsilon}$$

The cylinder surface is mapped into discretized helical matrix rain columns:

$$u_{\text{cyl}} = \left(\frac{\phi + z \cdot \omega_{\text{twist}}}{2\pi} + \frac{1}{2}\right) \cdot N_{\text{cols}}, \quad v_{\text{cyl}} = z \cdot v_z + t \cdot v_t$$

$$\text{col} = \lfloor u_{\text{cyl}} \rfloor, \quad \text{row} = \lfloor v_{\text{cyl}} \rfloor$$

#### Deciphering Streams & Hardware Mipmap Bloom
Stream intensity decomposes into a white-hot laser head and exponential phosphor decay:

$$I(p) = \exp(-\gamma \cdot |p - 1.0|) + \alpha_{\text{ambient}}, \quad p = \operatorname{mod}(\text{row} + t \cdot v_{\text{stream}}, L)$$

Target columns decrypt pseudo-random runes into the locked cipher sequence (`VALIKA`, `PETO`, `VALI`, `TEKK`, `TEKKO`). Halo bloom is extracted directly via hardware texture LOD mipmaps:

$$I_{\text{glow}}(\mathbf{uv}) = \operatorname{tex}_{\text{LOD}}(\mathbf{uv}, \lambda + 2.5)$$

---

### 3. Cosmic Mandelbrot (Deep Fractal Zoom)

<div align="center">
  <img src="screenshots/cosmic_mandelbrot_mobile.gif" alt="Cosmic Mandelbrot Mobile Preview" width="220">
</div>

#### Dual-Phase Zoom Dynamics & Ease-Out Return
The trajectory alternates between an immersive $24.0\text{s}$ plunge ($1\times \to 16{,}000\times$) toward the Seahorse Valley cusp $c_0 = -0.743643887 + 0.131825904i$ and a swift $6.0\text{s}$ ease-out return with accelerated rotational spin:

$$\text{Zoom-In } (p \in [0, 1]): \quad s(p) = \exp(p \cdot \ln s_{\max}), \quad \theta_{\text{in}}(p) = p \cdot \theta_0 + \sigma \sin(\omega t)$$

$$\text{Zoom-Out } (q \in [0, 1]): \quad s(q) = \exp\left((1 - q)^{2.2} \cdot \ln s_{\max}\right), \quad \theta_{\text{out}}(q) = \theta_0 + (1 - (1-q)^{2.2}) \cdot (2\pi - \theta_0) + \sigma \sin(\omega t)$$

$$c(\mathbf{u}, t) = c_0 + \mathbf{R}(\theta_t) \cdot \mathbf{u} \cdot \left(\frac{s_0}{s(t)}\right), \quad \mathbf{R}(\theta_t) = \begin{pmatrix} \cos \theta_t & -\sin \theta_t \\ \sin \theta_t & \cos \theta_t \end{pmatrix}$$

Upon completing the $2\pi$ rotation at $q = 1$, the camera smoothly rolls into the subsequent zoom-in without boundary cuts or resets.

#### Renormalized Continuous Potential (Escape-Time)
For $z_{n+1} = z_n^2 + c$, escape dynamics with threshold $R_{\text{esc}} = 256.0$ are smoothed using continuous fractional iteration count $\nu$:

$$\nu = n + 1 - \frac{\ln\left(\ln |z_n|\right)}{\ln 2}$$

Trigonometric cosine spectrum mapping:

$$\mathbf{C}(\nu) = \mathbf{a} + \mathbf{b} \cos\left(2\pi (\mathbf{c} \cdot \nu + \mathbf{d})\right)$$

---

### 4. Vortex Void (High-Contrast Cardioid Plunge)

<div align="center">
  <img src="screenshots/vortex_void_mobile.gif" alt="Vortex Void Mobile Preview" width="220">
</div>

#### Swirling Cardiac Rotations & Cardioid Orbit Traps
Building upon Shepard synthesis with accelerated plunge ($v = 0.60$), coordinates undergo dynamic harmonic camera sway:

$$\mathbf{u}_{\text{rot}} = \begin{pmatrix} \cos(\omega t) & -\sin(\omega t) \\ \sin(\omega t) & \cos(\omega t) \end{pmatrix} \mathbf{u}, \quad \omega t = 0.22 \sin(0.85 t)$$

Orbit trap distance to the true cardioid manifold:

$$d_{\text{trap}}(z) = \sqrt{\left(|z_x|\right)^2 + \left(z_y - \kappa \left(\sqrt{|z_x| + \epsilon} - \beta\right)\right)^2}$$

#### Negative-Space Cosmic Chasms & Non-Linear Sigmoid Response
Deep cosmic voids between cardioid shells are carved using a sinusoidal threshold mask:

$$M_{\text{dark}} = \left[\max\left(0, \frac{\sin(\mu \cdot \text{accum} - \nu \cdot d_{\text{trap}} + \omega t) - \tau_{\text{bias}}}{1 - \tau_{\text{bias}}}\right)\right]^\gamma$$

Non-linear tone mapping achieves pitch-black cosmic shadows with blazing neon highlights:

$$\mathbf{C}_{\text{final}} = \left(\frac{\mathbf{C}^\gamma}{\mathbf{C}^\gamma + \mathbf{k}}\right) \cdot \alpha_{\text{boost}}$$

---

## 🎵 Soundtracks

- **Celestial Heart**: *GoldFish & Cat Dealers — Colours & Lights (Clément Leroux Remix)*
- **Cyber Matrix Vortex**: *cYsmix — Escapism (Original Mix)*
- **Cosmic Mandelbrot**: *Brookes Brothers — Beautiful feat. Robert Owens (Original Mix)*
- **Vortex Void**: *Oliver Heldens feat. Shungudzo — Fire In My Soul*

---

## 🏛️ Project Architecture

```
├── index.html                 # Responsive portal entry with 3D liquid fluid canvas
├── style.css                  # Modern glassmorphism & responsive grid
├── celestial-heart/           # Celestial Heart (Shepard cardioid zoom)
│   ├── index.html
│   ├── pkg/
│   └── audio/
├── matrix-vortex/             # Cyber Matrix Vortex (3D glyph tunnel)
│   ├── index.html
│   ├── pkg/
│   └── audio/
├── cosmic-mandelbrot/         # Cosmic Mandelbrot (Deep zoom + ease-out return)
│   ├── index.html
│   ├── pkg/
│   └── audio/
├── vortex-void/               # Vortex Void (High-contrast cardiac plunge)
│   ├── index.html
│   ├── pkg/
│   └── audio/
└── screenshots/               # Previews and animated mobile GIFs
```

---

## 🛠️ Local Development

```bash
# Compile WASM
wasm-pack build --target web --release

# Serve locally
python3 -m http.server 8088
```
Navigate to `http://localhost:8088/`.
