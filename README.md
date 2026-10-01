# Infinite Real-Time GPU animations

[![hovnokod](https://raw.githubusercontent.com/tekk/hovnokod-badge/main/assets/badges/hovnokod-flat.svg)](https://github.com/tekk/hovnokod-badge)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=white)](https://webassembly.org/)
[![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![GitHub Pages](https://img.shields.io/badge/GitHub%20Pages-222222?logo=githubpages&logoColor=white)](https://tekk.github.io/valinfinite/)
[![License: GPL-2.0-only](https://img.shields.io/badge/License-GPL--2.0--only-blue.svg)](LICENSE)

Continuous scale-invariant GPU fractals in WebAssembly and WebGL2 with vivid psychedelic palettes, zero precision degradation, multi-act demoscene cinematics, and non-repeating soundtrack synchronization.

**Live Demo Portal**: [https://tekk.github.io/valinfinite/](https://tekk.github.io/valinfinite/)

---

## Gallery of 7 Real-Time GPU Animations

### Series I: Classic Procedural Zooms
Continuous logarithmic zooms and pure procedural coordinate topologies.

| [01 · Celestial Heart](https://tekk.github.io/valinfinite/celestial-heart/) | [02 · Cyber Matrix Vortex](https://tekk.github.io/valinfinite/matrix-vortex/) | [03 · Vortex Void](https://tekk.github.io/valinfinite/vortex-void/) |
| :---: | :---: | :---: |
| <a href="https://tekk.github.io/valinfinite/celestial-heart/"><img src="screenshots/celestial_heart_mobile.gif?v=1.2.3" alt="Celestial Heart Mobile Preview" width="220"></a> | <a href="https://tekk.github.io/valinfinite/matrix-vortex/"><img src="screenshots/matrix_vortex_mobile.gif?v=1.2.3" alt="Cyber Matrix Vortex Mobile Preview" width="220"></a> | <a href="https://tekk.github.io/valinfinite/vortex-void/"><img src="screenshots/vortex_void_mobile.gif?v=1.2.3" alt="Vortex Void Mobile Preview" width="220"></a> |
| Shepard Scale Zoom | Cylindrical Matrix Tunnel | 5-Scene Singularity Odyssey |

### Series II: Evolving Narrative & Metamorphic Odysseys
Multi-act narrative journeys, organic Bezier SDF morphing, optical wavefront crossings, and exponential relativistic dynamics.

| [04 · Celestial Odyssey](https://tekk.github.io/valinfinite/celestial-odyssey/) | [05 · Matrix Saga](https://tekk.github.io/valinfinite/matrix-saga/) | [06 · Cosmic Infinity](https://tekk.github.io/valinfinite/cosmic-infinity/) | [07 · Vortex Metamorphosis](https://tekk.github.io/valinfinite/vortex-metamorphosis/) |
| :---: | :---: | :---: | :---: |
| <a href="https://tekk.github.io/valinfinite/celestial-odyssey/"><img src="screenshots/celestial_odyssey_mobile.gif?v=1.2.3" alt="Celestial Odyssey Mobile Preview" width="180"></a> | <a href="https://tekk.github.io/valinfinite/matrix-saga/"><img src="screenshots/matrix_saga_mobile.gif?v=1.2.3" alt="Matrix Saga Mobile Preview" width="180"></a> | <a href="https://tekk.github.io/valinfinite/cosmic-infinity/"><img src="screenshots/cosmic_infinity_mobile.gif?v=1.2.3" alt="Cosmic Infinity Mobile Preview" width="180"></a> | <a href="https://tekk.github.io/valinfinite/vortex-metamorphosis/"><img src="screenshots/vortex_metamorphosis_mobile.gif?v=1.2.3" alt="Vortex Metamorphosis Mobile Preview" width="180"></a> |
| 3-Act Wavefront Journey | Cyberpunk Infiltration | 64,000x Mandelbrot Plunge | 8-Phase Bezier Morphing |

---

## Mathematical Formulations & Scene Algorithms

### 1. Celestial Heart (Classic Shepard Scale Zoom)

<div align="center">
  <img src="screenshots/celestial_heart_mobile.gif?v=1.2.3" alt="Celestial Heart Mobile Preview" width="220">
</div>

#### Logarithmic Octave Synthesis
Scale invariance is achieved using $N = 4$ overlapping octaves with scale multiplier $S = 3.2$. Each octave $k \in \{0, \dots, N-1\}$ has a sawtooth phase $\phi_k(t) \in [0, 1)$:

$$\phi_k(t) = \mathrm{fract}\left(t \cdot v + \frac{k}{N}\right), \quad s_k(t) = \exp\left(\phi_k(t) \cdot \ln S\right)$$

#### Perceptual Octave Windowing
Smooth birth and death of layers are governed by a raised-cosine Hann window:

$$w_k(t) = \frac{1}{2} - \frac{1}{2} \cos\left(2\pi \cdot \phi_k(t)\right)$$

#### Bilateral Symmetry & Cardioid Cleft Fold
Within each octave, the complex coordinate $z$ undergoes bilateral folding and cardioid indentation:

$$z_x \leftarrow |z_x|, \quad z_y \leftarrow z_y - \alpha \left(\sqrt{|z_x| + \epsilon} - \beta\right)$$

---

### 2. Cyber Matrix Vortex (Classic Cylindrical Wormhole)

<div align="center">
  <img src="screenshots/matrix_vortex_mobile.gif?v=1.2.3" alt="Cyber Matrix Vortex Mobile Preview" width="220">
</div>

#### Cylindrical Raymarched Tunnel Projection
2D screen coordinates $\mathbf{u} = (u_x, u_y)$ map onto an infinite 3D cylinder interior:

$$r = \|\mathbf{u}\|_2, \quad \phi = \mathrm{atan2}(u_y, u_x), \quad z = \frac{1}{r + \epsilon}$$

#### Logarithmic Streaming Cipher Columns
Glyph streams cascade down along cylindrical coordinate boundaries:

$$I(p) = \exp(-\gamma \cdot |p - 1.0|) + \alpha_{\text{ambient}}, \quad p = \mathrm{mod}(\text{row} + t \cdot v_{\text{stream}}, L)$$

#### Mipmapped Stroke & Atmospheric Fog
Glyph stroke intensities and glowing halos are sampled with continuous level-of-detail:

$$I_{\text{glow}}(\mathbf{uv}) = \mathrm{tex}_{\mathrm{LOD}}(\mathbf{uv}, \lambda + 2.5), \quad \mathbf{C}_{\text{pixel}} = \mathbf{C}_{\text{glyph}} \cdot \mathrm{smoothstep}(z_{\max}, z_{\min}, z)$$

---

### 3. Vortex Void (5-Scene Futuristic Demoscene Odyssey)

<div align="center">
  <img src="screenshots/vortex_void_mobile.gif?v=1.2.3" alt="Vortex Void Mobile Preview" width="220">
</div>

#### Multi-Scene Quantum Macro Cycle & Partition of Unity
The animation unfolds across an $80.0\text{s}$ macro cycle featuring 5 brand-new, distinct futuristic demoscene realms connected via continuous $C^1$-smooth transition windows ($\Delta t = 3.6\text{s}$):

$$\sum_{k=1}^5 w_k(t) = 1.0, \quad \mathbf{C}_{\text{total}} = \sum_{k=1}^5 w_k(t) \mathbf{C}_k(\mathbf{u}_{\text{warp}}, t) + \mathbf{C}_{\text{flare}}(t)$$

- **Scene 1: The Event Horizon ($0\text{s} - 16\text{s}$)**: Relativistic Kerr black hole with spacetime gravitational lensing, Lense-Thirring frame-dragging, Doppler-beamed accretion disk, razor-sharp photon ring ($1.55 r_s$), and magnetic polar plasma jets.
- **Scene 2: The Cyber Tesseract ($16\text{s} - 32\text{s}$)**: 4D quantum hypercube double-rotating simultaneously in the $XW$ and $YZ$ planes, stereographically projected into 3D and 2D perspective, with 32 laser edges, glowing quantum nodes, and traveling photon data pulses.
- **Scene 3: The Cyber-Hex Conduit ($32\text{s} - 48\text{s}$)**: 6-sided faceted metallic obsidian conduit with 6 longitudinal corner emerald ribs, segmental panel seams, expanding hexagonal iris quantum gates, and relativistic particle streaks.
- **Scene 4: The Quantum Gyroscope ($48\text{s} - 64\text{s}$)**: 3-axis concentric toroidal gimbal rings with chamfered bevels, laser-etched precision telemetry graduations, a pulsating central magnetic plasma core with dipole flux loops, and orbiting drone satellites.
- **Scene 5: The Hyperbolic Kaleidoscope Void & Implosion ($64\text{s} - 80\text{s}$)**: Non-Euclidean domain folding crystal cathedral in deep obsidian space, culminating in a violent gravitational collapse ($\mathbf{u} \to \infty$) and a blinding relativistic warp shockwave burst that smoothly loops back into Scene 1.

#### Mathematical Formulations: Relativistic Lensing & Spatial Ripple Wave
- **Einstein Gravitational Deflection & Doppler Boosting**:
  $$\mathbf{p}_{\text{lens}} = \mathbf{p} \left(1.0 - \frac{r_s}{\|\mathbf{p}\|^{1.5} + \epsilon}\right), \quad I_{\text{doppler}} = \mathrm{clamp}\left(1.0 - 0.75 \frac{p_{\text{disk},x}}{r_{\text{disk}} + 0.05}, 0.25, 2.8\right)$$
- **Optical Spatial Ripple Wave**:
  $$\mathbf{u}_{\text{warp}} = \mathbf{u} + \frac{\mathbf{u}}{\|\mathbf{u}\|} \cdot A \sin(\omega \|\mathbf{u}\| - \tau \nu) e^{-k \|\mathbf{u}\|} \cdot 4 w_a(t) w_b(t)$$

---

### 4. Celestial Odyssey (3-Act Narrative Journey & Wavefront Crossing)

<div align="center">
  <img src="screenshots/celestial_odyssey_mobile.gif?v=1.2.3" alt="Celestial Odyssey Mobile Preview" width="220">
</div>

#### Macro Story Architecture & Dynamic Act Weights
The animation unfolds across a $72.0\text{s}$ macro cycle smoothly partitioned into three distinct acts:

$$w_1(t) + w_2(t) + w_3(t) = 1, \quad \theta_{\text{cam}}(t) = \theta_1(t) w_1(t) + \theta_2(t) w_2(t) + \theta_3(t) w_3(t)$$

- **Act 1: Celestial Genesis ($0\text{s} - 20\text{s}$)**: Serene infinite Shepard scale zoom with gentle breathing sway, sapphire/rose neon auroras, and rhythmic heartbeat pulses.
- **Act 2: Vortex Mandala Bloom ($24\text{s} - 44\text{s}$)**: Hypnotic swirling vortex rotation with dihedral symmetry folds, blooming kaleidoscopic heart rosettes, and emerald-cyan auroras.
- **Act 3: Supernova Astral Storm ($48\text{s} - 68\text{s}$)**: Relativistic accelerating surges, molten plasma gold, incandescent solar core eruptions, and atmospheric radiance.

#### Scene Changing Crossings (Optical Caustics & Singularity Ripples)
Acts are bridged by optical wavefront sweeps refracting spacetime:

$$\mathbf{u}' = \mathbf{u} + \mathbf{d}_{\text{wave}} \cdot \sin(\psi(\mathbf{u}) \cdot \omega_c) e^{-k_c |\psi(\mathbf{u}) - c_0(t)|}, \quad \mathbf{C}_{\text{crossing}} = \mathbf{C}_{\text{beam}} e^{-k_b |\psi(\mathbf{u}) - c_0(t)|}$$

#### Integrated Strictly Monotonic Zoom Phase
Zoom phase is guaranteed monotonic $\frac{d\Phi}{dt} > 0$ across all time variations:

$$\Phi(t) = v_0 \cdot t - \frac{A_1}{\omega_1} \cos(\omega_1 t) - \frac{A_2}{\omega_2} \cos(\omega_2 t)$$

---

### 5. Matrix Saga (3-Stage Cyberpunk Infiltration & Dynamic Plexus)

<div align="center">
  <img src="screenshots/matrix_saga_mobile.gif?v=1.2.3" alt="Matrix Saga Mobile Preview" width="220">
</div>

#### Tri-Realm Convex Interpolation
The narrative progresses through three distinct cyberpunk environments across a $72.0\text{s}$ macro cycle:

$$\mathbf{C}_{\text{total}} = \sum_{k=1}^3 w_k(t) \mathbf{C}_k + \mathbf{C}_{\text{crossing}} + \mathbf{C}_{\text{sparkles}}, \quad \sum_{k=1}^3 w_k(t) = 1$$

- **Chapter 1: Gateway to Cyberspace ($0\text{s} - 20\text{s}$)**: Planar 3D perspective cyber grid receding into an infinite neon horizon with centered falling glyph streams.
- **Chapter 2: Helical Vortex Wormhole ($21\text{s} - 43\text{s}$)**: Accelerated 3D cylindrical vortex plunge with dynamic helical twisting and cipher streams, gracefully fading out into deep void before Chapter 3.
- **Chapter 3: Dynamic Quantum Plexus & Parallax Sparkles ($42\text{s} - 64\text{s}$)**: Moving constellation vertices interconnected with local mesh lines whose connection intensities dynamically adapt based on distance, animated traveling photon pulses, and a multi-tier parallax layer of tiny sparkles that starts flowing upward from the bottom, curves horizontally, and continuously evolves its trajectory.
- **The Wormhole Returns ($63\text{s} - 72\text{s}$)**: As Chapter 3 finishes, the camera plunges back into the helical wormhole at hyper-velocity before completing the macro loop.

#### Planar Perspective Grid & Neural Plexus Equations
- **Perspective Data Grid**:
  $$z_{\text{grid}} = \frac{h}{|u_y + \delta|}, \quad X = u_x \cdot z_{\text{grid}}, \quad Z = z_{\text{grid}} + v_z t, \quad I_{\text{grid}} = e^{-\kappa |X - \lfloor X \rceil|} + e^{-\kappa |Z - \lfloor Z \rceil|}$$
- **Distance-Based Dynamic Plexus Lines**:
  $$d_{ij} = \|\mathbf{P}_i - \mathbf{P}_j\|, \quad w_{ij} = \left(\text{smoothstep}(D_{\max}, D_{\min}, d_{ij})\right)^2$$
  $$I_{\text{laser}} = \left(1 - \text{smoothstep}(\epsilon_0, \epsilon_1, d_{\text{line}})\right) \cdot w_{ij}, \quad I_{\text{pulse}} = e^{-\beta |h_{\text{proj}} - p_{\text{pos}}|} \cdot e^{-\gamma d_{\text{line}}} \cdot w_{ij}$$

---

### 6. Cosmic Infinity (64,000x Multi-Target Deep Zoom & Continuous Filament Flow)

<div align="center">
  <img src="screenshots/cosmic_infinity_mobile.gif?v=1.2.3" alt="Cosmic Infinity Mobile Preview" width="220">
</div>

#### Dual-Phase Zoom Dynamics: Multi-Target Wandering, Multi-Stage Rotation & Relativistic Return
The trajectory operates across a 32.0s macro cycle at 1x speed (24.0s deep plunge and 8.0s relativistic return). Each cycle dynamically locks onto a different dense boundary feature from a curated catalog (Seahorse Valley, Quad Spiral Dendrites, Triple Spiral Valleys, Satellite Mini-Brots, Elephant Valley boundary) with smooth target panning and continuous filament tracking, ensuring fluid and continuous camera orientation across all cycle boundaries without hops or jumps:

$$\text{Zoom-In } (p \in [0, 1]): \quad s_{\text{in}}(p) = \begin{cases} v_0 \cdot p & p \le p_0 \\ 1 - a_{\text{dec}} (1 - p)^2 & p > p_0 \end{cases}, \quad \text{zoom}(p) = \exp(s_{\text{in}}(p) \cdot \ln s_{\max})$$

At peak zoom ($s_{\max} = 64{,}000\times$, calibrated to prevent floating-point precision block quantization rectangles), inward velocity $\left.\frac{ds_{\text{in}}}{dp}\right|_{p=1} = 0$, bringing the plunge to a complete stop. Rotation speeds dynamically surge and ease through multi-stage harmonic modulation. The camera then smoothly accelerates outward with a logistic sigmoid return and 360° turnaround spin:

$$\text{Zoom-Out } (q \in [0, 1]): \quad g(q) = \frac{\sigma(k(2q - 1)) - \sigma(-k)}{\sigma(k) - \sigma(-k)}, \quad \text{zoom}(q) = \exp((1 - g(q)) \cdot \ln s_{\max})$$

$$\sigma(x) = \frac{1}{1 + \exp(-x)}, \quad \theta_{\text{out}}(q) = \theta_0 + g(q) \cdot (2\pi - \theta_0)$$

Rotation and viewport centering are $C^1$-continuous without hops at either turnaround point ($p=1$ or $q=1$).

---

### 7. Vortex Metamorphosis (8-Phase Bezier Morphing & Procedural VFX)

<div align="center">
  <img src="screenshots/vortex_metamorphosis_mobile.gif?v=1.2.3" alt="Vortex Metamorphosis Mobile Preview" width="220">
</div>

Concentric multi-layer Bezier SDF rendering with harmonic scaling and out-of-phase rotational offsets:

$$\mathbf{p}_l = \frac{1}{s_l} \mathbf{R}(\theta_l) \mathbf{p}, \quad s_l = 1 + \alpha l + \delta \sin(\omega_s t + l), \quad d_l = f_{\text{morph}}(\mathbf{p}_l, \max(0, t_{\text{scene}} - l \Delta \tau)) \cdot s_l$$

Continuous topological metamorphosis interpolates between signed distance fields across the sequence:

$$d_{\text{morph}}(t) = (1 - S(\tau)) d_{\text{prev}} + S(\tau) d_{\text{next}}, \quad S(\tau) = \tau^2(3 - 2\tau), \quad \tau = \mathrm{clamp}\left(\frac{t - t_0}{\Delta t}, 0, 1\right)$$

$$\mathbf{C}_{\text{accum}} = \mathbf{C}_{\text{bg}} + \sum_{l=0}^{N-1} w_l \left[ \mathbf{C}_l \left(e^{-k_{\text{core}} |d_l|} + e^{-k_{\text{aura}} |d_l|} + I_{\text{fill}}(d_l)\right) + \mathbf{C}_{\text{hi}} e^{-3 k_{\text{core}} |d_l|} \right]$$

Refractive shockwave solar lensing and ACES filmic tonemapping:

$$\mathbf{p}' = \mathbf{p} + \frac{\mathbf{p}}{\|\mathbf{p}\|} \sin((\|\mathbf{p}\| - r_w) \omega_w) e^{-k_w |\|\mathbf{p}\| - r_w|} \cdot A_w, \quad \mathbf{C}_{\text{final}} = \frac{\mathbf{C} (a \mathbf{C} + b)}{\mathbf{C} (c \mathbf{C} + d) + e}$$

---

## Soundtrack

Audio engine features a persistent non-repeating shuffle pool stored in `localStorage`. Tracks are selected randomly, with playback control OSD.

1. **Fire In My Soul** — Oliver Heldens feat. Shungudzo *(02:55)*
2. **No Limits (Vocal Mix)** — Danism, Train & DJ Rae *(06:15)*
3. **Say My Name (Sub Focus Remix)** — Morgan Seatree, Sub Focus *(03:13)*
4. **Bambou (Original Mix)** — Sebastien Leger *(07:16)*
5. **TRONCE** — Sili *(04:51)*
6. **Beautiful** — Brookes Brothers feat. Robert Owens *(04:55)*
7. **Escapism (Original Mix)** — cYsmix *(05:00)*
8. **Unity** — TheFatRat *(04:09)*
9. **Underground** — Tantrum Desire *(04:34)*
10. **Rhyme Dust (Dimension Remix)** — MK, Dom Dolla *(03:24)*
11. **Horizon** — 1991, Poppy Baskcomb *(03:00)*
12. **Colours & Lights (Clément Leroux Remix)** — GoldFish & Cat Dealers *(03:22)*
13. **Mend Your Ways** — PSYQUI *(04:27)*
14. **Nights Introlude** — Nightmares On Wax *(04:40)*
15. **Genesis** — Subsonic *(03:43)*
16. **Get To Me** — Culture Shock *(04:05)*
17. **Focused** — Soulfreq *(07:36)*
18. **King Of The Swingers (Gettin' Mad Mix)** — Krushed & Sorted *(06:04)*
19. **Drugs I Like** — nate band *(03:18)*
20. **Remember Me** — High Contrast *(03:55)*
21. **TANGARA** — Etherwood, Hugh Hardie *(03:43)*
22. **Beat Keep Rockin'** — Starjunk 95 *(03:03)*
23. **Spectra Ocean Dream Circuit** — Starjunk 95 *(03:14)*
24. **Groove District** — Starjunk 95 *(03:06)*
25. **Tell You What I Did** — Pola & Bryson, Zitah *(03:29)*
26. **TAKE ME** — D A N N Y *(02:09)*
27. **Mirage** — MPH, Skrillex *(04:52)*
28. **Liberate (Lane 8 Remix)** — Eric Prydz *(05:14)*
29. **Szikra** — Kornél Kovács *(06:41)*
30. **I Run** — YUSSI *(02:04)*
31. **On & On** — Chris Lake, Yael Watchman *(03:15)*
32. **Out For Blood** — QZB *(04:08)*
33. **Don't Stop** — MUZZ *(02:56)*
34. **Tu Cafe (Mash Up)** — Prodigy *(04:02)*
35. **The People (Mehlor Remix)** — Harrie Summers, Joey Rich *(06:27)*
36. **Spacefunk** — Stussko, Kolter *(07:36)*
37. **Bunker** — Culture Shock *(04:41)*
38. **On & On (Kanine Remix)** — Sub Focus, bbyclose, Kanine *(02:55)*

---

## Project Architecture

```
├── index.html                   # Responsive landing portal with 3D fluid metaball shader
├── style.css                    # Glassmorphism styling, crossfade cards & animated loops
├── audio/
│   ├── player.js                # Smart non-repeating shuffle audio player (38 tracks)
│   ├── playlist.json            # Track catalog metadata
│   └── tracks/                  # 38 high-fidelity demoscene tracks
│
├── celestial-heart/             # [01] Classic Shepard scale infinite zoom
├── matrix-vortex/               # [02] Classic cylindrical raymarched matrix tunnel
├── vortex-void/                 # [03] 5-Scene futuristic demoscene singularity odyssey
│
├── celestial-odyssey/           # [04] 3-Act narrative journey with optical wavefronts
├── matrix-saga/                 # [05] 3-Stage cyberpunk infiltration & sacred AI mandala
├── cosmic-infinity/             # [06] 64,000x deep zoom with relativistic S-curve return
├── vortex-metamorphosis/        # [07] 8-Phase Bezier SDF morphing & procedural VFX
│
├── screenshots/                 # Landscape PNGs + Vertical Mobile Animated GIFs
└── src/                         # Rust & WebGL2 shader source pipeline
```

---

## Local Development

```bash
# Compile WASM
wasm-pack build --target web --release

# Serve locally, i.e. with python3
python3 -m http.server 8088
```
Navigate to `http://localhost:8088/`.

---

## License

This project is licensed under the **GNU General Public License v2.0 only** (`GPL-2.0-only`). See the [LICENSE](LICENSE) file for the full license text.

---

## Audio Credits & Disclaimer

The musical tracks featured in this project are incorporated strictly for non-commercial, interactive art and demoscene demonstration purposes. All music rights, copyrights, composition, and performance ownership belong unconditionally to their respective original artists, producers, and affiliated record labels.

No copyright infringement is intended. If you are a copyright owner and wish to request removal or modification of any track, please submit an issue on this repository.

Please support the featured artists by purchasing and streaming their music on their official channels:

| # | Track Title | Artist(s) | Duration |
|---|-------------|-----------|----------|
| 01 | **Fire In My Soul** | Oliver Heldens feat. Shungudzo | 02:55 |
| 02 | **No Limits (Vocal Mix)** | Danism, Train & DJ Rae | 06:15 |
| 03 | **Say My Name (Sub Focus Remix)** | Morgan Seatree, Sub Focus | 03:13 |
| 04 | **Bambou (Original Mix)** | Sebastien Leger | 07:16 |
| 05 | **TRONCE** | Sili | 04:51 |
| 06 | **Beautiful** | Brookes Brothers feat. Robert Owens | 04:55 |
| 07 | **Escapism (Original Mix)** | cYsmix | 05:00 |
| 08 | **Unity** | TheFatRat | 04:09 |
| 09 | **Underground** | Tantrum Desire | 04:34 |
| 10 | **Rhyme Dust (Dimension Remix)** | MK, Dom Dolla | 03:24 |
| 11 | **Horizon** | 1991, Poppy Baskcomb | 03:00 |
| 12 | **Colours & Lights (Clément Leroux Remix)** | GoldFish & Cat Dealers | 03:22 |
| 13 | **Mend Your Ways** | PSYQUI | 04:27 |
| 14 | **Nights Introlude** | Nightmares On Wax | 04:40 |
| 15 | **Genesis** | Subsonic | 03:43 |
| 16 | **Get To Me** | Culture Shock | 04:05 |
| 17 | **Focused** | Soulfreq | 07:36 |
| 18 | **King Of The Swingers (Gettin' Mad Mix)** | Krushed & Sorted | 06:04 |
| 19 | **Drugs I Like** | nate band | 03:18 |
| 20 | **Remember Me** | High Contrast | 03:55 |
| 21 | **TANGARA** | Etherwood, Hugh Hardie | 03:43 |
| 22 | **Beat Keep Rockin'** | Starjunk 95 | 03:03 |
| 23 | **Spectra Ocean Dream Circuit** | Starjunk 95 | 03:14 |
| 24 | **Groove District** | Starjunk 95 | 03:06 |
| 25 | **Tell You What I Did** | Pola & Bryson, Zitah | 03:29 |
| 26 | **TAKE ME** | D A N N Y | 02:09 |
| 27 | **Mirage** | MPH, Skrillex | 04:52 |
| 28 | **Liberate (Lane 8 Remix)** | Eric Prydz | 05:14 |
| 29 | **Szikra** | Kornél Kovács | 06:41 |
| 30 | **I Run** | YUSSI | 02:04 |
| 31 | **On & On** | Chris Lake, Yael Watchman | 03:15 |
| 32 | **Out For Blood** | QZB | 04:08 |
| 33 | **Don't Stop** | MUZZ | 02:56 |
| 34 | **Tu Cafe (Mash Up)** | Prodigy | 04:02 |
| 35 | **The People (Mehlor Remix)** | Harrie Summers, Joey Rich | 06:27 |
| 36 | **Spacefunk** | Stussko, Kolter | 07:36 |
| 37 | **Bunker** | Culture Shock | 04:41 |
| 38 | **On & On (Kanine Remix)** | Sub Focus, bbyclose, Kanine | 02:55 |
