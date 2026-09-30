# 💖 Infinite Real-Time GPU Fractals

Continuous scale-invariant GPU fractals in WebAssembly and WebGL2 with vivid psychedelic palettes, zero precision degradation, and audio synchronization.

🌐 **Live Portal**: [https://tekk.github.io/valinfinite/](https://tekk.github.io/valinfinite/)
- **Celestial Heart**: [https://tekk.github.io/valinfinite/celestial-heart/](https://tekk.github.io/valinfinite/celestial-heart/)
- **Cyber Matrix Vortex**: [https://tekk.github.io/valinfinite/matrix-vortex/](https://tekk.github.io/valinfinite/matrix-vortex/)
- **Cosmic Mandelbrot**: [https://tekk.github.io/valinfinite/cosmic-mandelbrot/](https://tekk.github.io/valinfinite/cosmic-mandelbrot/)
- **Vortex Void**: [https://tekk.github.io/valinfinite/vortex-void/](https://tekk.github.io/valinfinite/vortex-void/)

---

## 🔬 Mathematical Formulations & Scene Algorithms

### 1. Celestial Heart (Multi-Act Story & Scene Changing Crossings)

<div align="center">
  <img src="screenshots/celestial_heart_mobile.gif" alt="Celestial Heart Mobile Preview" width="220">
</div>

#### Macro Story Architecture & Dynamic Act Weights
The animation unfolds as a continuous narrative across a $72.0\text{s}$ macro cycle, smoothly partitioned into three distinct acts with gradual transitions:

$$w_1(t) + w_2(t) + w_3(t) = 1, \quad \theta_{\text{cam}}(t) = \theta_1(t) w_1(t) + \theta_2(t) w_2(t) + \theta_3(t) w_3(t)$$

- **Act 1: Celestial Genesis ($0\text{s} - 20\text{s}$)**: Serene infinite Shepard scale zoom with gentle breathing sway, sapphire/rose neon auroras, and rhythmic heartbeat pulses.
- **Act 2: Vortex Mandala Bloom ($24\text{s} - 44\text{s}$)**: Hypnotic swirling vortex rotation with dihedral symmetry folds, blooming kaleidoscopic heart rosettes, and emerald-cyan auroras.
- **Act 3: Supernova Astral Storm ($48\text{s} - 68\text{s}$)**: Relativistic accelerating surges, molten plasma gold, incandescent solar core eruptions, and crackling procedural lightning arcs.

#### Scene Changing Crossings (Optical Caustics & Singularity Ripples)
Chapters are bridged by optical wavefront sweeps refracting spacetime:

$$\mathbf{u}' = \mathbf{u} + \mathbf{d}_{\text{wave}} \cdot \sin\left(\psi(\mathbf{u}) \cdot \omega_c\right) e^{-k_c |\psi(\mathbf{u}) - c_0(t)|}, \quad \mathbf{C}_{\text{crossing}} = \mathbf{C}_{\text{beam}} e^{-k_b |\psi(\mathbf{u}) - c_0(t)|}$$

#### Logarithmic Octave Synthesis & Dynamic Folding Geometry
Infinite scale invariance is preserved across all acts using $N = 4$ overlapping logarithmic octaves ($S = 3.2$), driven by a strictly monotonic integrated zoom phase:

$$\Phi(t) = v_0 \cdot t - \frac{A_1}{\omega_1} \cos(\omega_1 t) - \frac{A_2}{\omega_2} \cos(\omega_2 t), \quad \frac{d\Phi}{dt} > 0 \quad \forall t$$

Within each octave, the complex coordinate $z$ undergoes bilateral symmetry, cardioid cleft folding, spherical inversion, and act-dependent dihedral rotations:

$$z_x \leftarrow |z_x|, \quad z_y \leftarrow z_y - \alpha \left(\sqrt{|z_x| + \epsilon} - \beta\right), \quad z \leftarrow \mathbf{R}\left(\theta_{\text{fold}}(t)\right) z - \mathbf{b}_{\text{fold}}(t)$$

---

### 2. Cyber Matrix Vortex (Multi-Scene Story & Gentle Interleaving)

<div align="center">
  <img src="screenshots/matrix_vortex_mobile.gif" alt="Cyber Matrix Vortex Mobile Preview" width="220">
</div>

#### Macro Story Architecture & Gentle Interleaving
The narrative progresses through three distinct cyberpunk realms across a $72.0\text{s}$ macro cycle, smoothly blended using partition-of-unity cubic smoothstep easing:

$$\mathbf{C}_{\text{total}} = \sum_{k=1}^3 w_k(t) \mathbf{C}_k + \mathbf{C}_{\text{crossing}}, \quad \sum_{k=1}^3 w_k(t) = 1$$

- **Chapter 1: The Gateway to Cyberspace ($0\text{s} - 20\text{s}$)**: Planar 3D perspective cyber grid receding into an infinite neon horizon, accompanied by cascading vertical sky rain and data packets.
- **Chapter 2: The Helical Vortex Descent ($24\text{s} - 46\text{s}$)**: Accelerated 3D cylindrical vortex plunge with dynamic helical twisting, cipher streams deciphering into luminous Kanji and sacred runes, and rotating holographic word banners.
- **Chapter 3: AI Singularity Core & Sacred Mandala ($50\text{s} - 68\text{s}$)**: Four concentric counter-rotating sacred glyph rings orbiting around a blinding incandescent solar singularity with volumetric crepuscular rays and lightning arcs.

#### Interleaving Crossings (Defragmentation & Gravitational Ripples)
Transitions are rendered organically through optical wavefront sweeps:

$$\text{Crossing 1: } \mathbf{u}' = \mathbf{u} + \hat{\mathbf{x}} \sin(\psi_y \cdot \omega) e^{-k_1 |\psi_y|}, \quad \text{Crossing 2: } \mathbf{u}' = \mathbf{u} + \frac{\mathbf{u}}{\|\mathbf{u}\|} \sin\left((\|\mathbf{u}\| - r_w) \omega_r\right) e^{-k_2 |\|\mathbf{u}\| - r_w|}$$

#### 3D Perspective Grid, Cylindrical Wormhole & Orbital Mandala
- **Perspective Data Grid**:
  $$z_{\text{grid}} = \frac{h}{|u_y + \delta|}, \quad X = u_x \cdot z_{\text{grid}}, \quad Z = z_{\text{grid}} + v_z t, \quad I_{\text{grid}} = e^{-\kappa |X - \lfloor X \rceil|} + e^{-\kappa |Z - \lfloor Z \rceil|}$$
- **3D Helical Vortex**:
  $$z = \frac{1}{\|\mathbf{u}\| + \epsilon}, \quad \phi_{\text{tunnel}} = \mathrm{atan2}(u_y, u_x) + z \cdot \omega_{\text{twist}}, \quad u_{\text{cyl}} = \left(\frac{\phi_{\text{tunnel}}}{2\pi} + \frac{1}{2}\right) N_{\text{cols}}$$
- **Counter-Rotating Sacred Mandala**:
  $$\theta_k = \phi - \Omega_k t, \quad u_k = \left(\frac{\theta_k}{2\pi} + \frac{1}{2}\right) N_k, \quad I_{\text{ring}}(r) = \exp\left(-\frac{(r - R_k)^2}{2\sigma_k^2}\right)$$

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

### 4. Vortex Void (Layered Bezier Metamorphosis)

<div align="center">
  <img src="screenshots/vortex_void_mobile.gif" alt="Vortex Void Mobile Preview" width="220">
</div>

#### Multi-Layer Bezier Rendering & Out-of-Phase Compositing
The animation renders concentric layers out-of-phase with dynamic harmonic scaling and rotational offsets:

$$\mathbf{p}_l = \frac{1}{s_l} \mathbf{R}(\theta_l) \mathbf{p}, \quad s_l = 1 + \alpha l + \delta \sin(\omega_s t + l), \quad \theta_l = (l - 2.5) \beta \sin(\omega_\theta t) + \phi_l$$

$$d_l = f_{\text{morph}}(\mathbf{p}_l, \max(0, t_{\text{scene}} - l \Delta \tau)) \cdot s_l$$

$$\mathbf{C}_{\text{accum}} = \mathbf{C}_{\text{bg}} + \sum_{l=0}^{N-1} w_l \left[ \mathbf{C}_l \left(e^{-k_{\text{core}} |d_l|} + e^{-k_{\text{aura}} |d_l|} + I_{\text{fill}}(d_l)\right) + \mathbf{C}_{\text{hi}} e^{-3 k_{\text{core}} |d_l|} \right]$$

#### Cubic Smoothstep Morphing & Topological Sequence
Continuous metamorphosis interpolates between signed distance fields across the macro cycle:

$$d_{\text{morph}}(t) = (1 - S(\tau)) d_{\text{prev}} + S(\tau) d_{\text{next}}, \quad S(\tau) = \tau^2(3 - 2\tau), \quad \tau = \mathrm{clamp}\left(\frac{t - t_0}{\Delta t}, 0, 1\right)$$

$$\text{Circle (unfilled } \to \text{ solid)} \longrightarrow \text{Heart (sustained)} \longrightarrow \text{Square} \longrightarrow \text{Octagon} \longrightarrow \text{Hexagon} \longrightarrow \text{Star} \longrightarrow \text{Polygon} \longrightarrow \text{Dynamic Bezier}$$

#### Solar Lensing & ACES Filmic Tone Mapping
Refractive radial shockwaves distort the field prior to projection, and high dynamic range energy is mapped to prevent saturation clipping:

$$\mathbf{p}' = \mathbf{p} + \frac{\mathbf{p}}{\|\mathbf{p}\|} \sin\left((\|\mathbf{p}\| - r_w) \omega_w\right) e^{-k_w |\|\mathbf{p}\| - r_w|} \cdot A_w$$

$$\mathbf{C}_{\text{final}} = \frac{\mathbf{C} (a \mathbf{C} + b)}{\mathbf{C} (c \mathbf{C} + d) + e}$$

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
