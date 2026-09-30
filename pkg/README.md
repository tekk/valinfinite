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

$$\phi_k(t) = \mathrm{fract}\left(t \cdot v + \frac{k}{N}\right), \quad s_k(t) = \exp\left(\phi_k(t) \cdot \ln S\right)$$

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

$$r = \|\mathbf{u}\|_2, \quad \phi = \mathrm{atan2}(u_y, u_x), \quad z = \frac{1}{r + \epsilon}$$

The cylinder surface is mapped into discretized helical matrix rain columns:

$$u_{\text{cyl}} = \left(\frac{\phi + z \cdot \omega_{\text{twist}}}{2\pi} + \frac{1}{2}\right) \cdot N_{\text{cols}}, \quad v_{\text{cyl}} = z \cdot v_z + t \cdot v_t$$

$$\text{col} = \lfloor u_{\text{cyl}} \rfloor, \quad \text{row} = \lfloor v_{\text{cyl}} \rfloor$$

#### Deciphering Streams & Hardware Mipmap Bloom
Stream intensity decomposes into a white-hot laser head and exponential phosphor decay:

$$I(p) = \exp(-\gamma \cdot |p - 1.0|) + \alpha_{\text{ambient}}, \quad p = \mathrm{mod}(\text{row} + t \cdot v_{\text{stream}}, L)$$

Target columns periodically decipher cascading code into luminous Kanji and sacred cybernetic runes. Halo bloom is extracted directly via hardware texture LOD mipmaps:

$$I_{\text{glow}}(\mathbf{uv}) = \mathrm{tex}_{\mathrm{LOD}}(\mathbf{uv}, \lambda + 2.5)$$

#### Solar Flare Burst & Demoscene VFX Pipeline
A radiant sun flare at the vortex singularity projects multi-harmonic diffraction starburst rays, anamorphic streaks, and volumetric crepuscular shafts:

$$I_{\text{solar}}(\mathbf{u}) = c_{\text{disc}} \exp(-\beta_1 \|\mathbf{u} - \mathbf{p}_{\odot}\|) + \sum_{k \in \{8, 12\}} \cos^{m_k}(k \phi_{\odot} \pm \omega_k t) \exp(-\beta_{\text{ray}} \|\mathbf{u} - \mathbf{p}_{\odot}\|)$$

$$I_{\text{streak}}(\mathbf{u}) = \exp(-\beta_y |u_y'|) \exp(-\beta_x |u_x'|), \quad \mathbf{u}' = \mathbf{R}_{\theta} (\mathbf{u} - \mathbf{p}_{\odot})$$

---

### 3. Cosmic Mandelbrot (Deep Fractal Zoom)

<div align="center">
  <img src="screenshots/cosmic_mandelbrot_mobile.gif" alt="Cosmic Mandelbrot Mobile Preview" width="220">
</div>

#### Dual-Phase Zoom Dynamics: Gradual Stop & Exponential Ease-Out Return
The trajectory alternates between an immersive $24.0\text{s}$ plunge deep into Seahorse Valley ($1\times \to 64{,}000\times$) at $c_0 = -0.743643887 + 0.131825904i$, smoothly decelerating to a total stop at the apex, followed by an $8.0\text{s}$ exponential acceleration zoom-out with an easing stop:

$$\text{Zoom-In } (p \in [0, 1]): \quad s_{\text{in}}(p) = \begin{cases} v_0 \cdot p & p \le p_0 \\ 1 - a_{\text{dec}} (1 - p)^2 & p > p_0 \end{cases}, \quad \text{zoom}(p) = \exp(s_{\text{in}}(p) \cdot \ln s_{\max})$$

At $p = 1.0$, the inward velocity $\frac{ds_{\text{in}}}{dp} = 0$, bringing the zoom to a complete standstill at $64{,}000\times$. Zoom-out immediately commences, accelerating exponentially:

$$\text{Zoom-Out } (q \in [0, 1]): \quad g(q) = \frac{\sigma(k(2q - 1)) - \sigma(-k)}{\sigma(k) - \sigma(-k)}, \quad \text{zoom}(q) = \exp((1 - g(q)) \cdot \ln s_{\max})$$

$$\sigma(x) = \frac{1}{1 + \exp(-x)}, \quad \theta_{\text{out}}(q) = \theta_0 + g(q) \cdot (2\pi - \theta_0)$$

The camera accelerates exponentially into full zoom-out speed at $q = 0.5$, then gracefully eases to a stop back at overview ($1\times$) as it rolls into the next cycle.

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
