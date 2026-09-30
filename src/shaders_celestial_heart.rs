pub const VERTEX_SHADER_SOURCE: &str = r#"#version 300 es
in vec2 a_position;

void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
}
"#;

pub const FRAGMENT_SHADER_SOURCE: &str = r#"#version 300 es
precision highp float;

uniform vec2 u_resolution;
uniform float u_time;

out vec4 fragColor;

#define PI 3.14159265359
#define TWO_PI 6.28318530718

// =========================================================================
// 1. DYNAMIC COLOR PALETTES ACROSS STORY CHAPTERS
// =========================================================================
vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
    return clamp(a + b * cos(TWO_PI * (c * t + d)), 0.0, 1.0);
}

// Act 1: Celestial Sapphire, Rose Quartz, Radiant Violet, Diamond Core
vec3 palette_act1(float t) {
    return palette(t, vec3(0.52, 0.48, 0.56), vec3(0.48, 0.45, 0.52), vec3(1.0, 1.0, 1.0), vec3(0.00, 0.33, 0.67));
}

// Act 2: Emerald Aurora, Golden Cyan, Electric Orchid, Sacred Mandala
vec3 palette_act2(float t) {
    return palette(t, vec3(0.45, 0.60, 0.52), vec3(0.50, 0.48, 0.40), vec3(1.0, 0.9, 1.1), vec3(0.12, 0.45, 0.78));
}

// Act 3: Solar Plasma Gold, Molten Amber, Crimson Blaze, Obsidian Core
vec3 palette_act3(float t) {
    return palette(t, vec3(0.68, 0.45, 0.22), vec3(0.55, 0.48, 0.35), vec3(1.1, 1.0, 0.8), vec3(0.02, 0.28, 0.58));
}

// =========================================================================
// 2. PROCEDURAL ACTION VFX (LIGHTNING & SHOCKWAVES)
// =========================================================================
float fxLightning(vec2 p, float t, float seed) {
    float bolt = 0.0;
    for (int i = 0; i < 3; i++) {
        float fi = float(i);
        float flash = step(0.68, fract(sin(floor(t * 5.0 + fi * 19.3 + seed)) * 43758.5453));
        if (flash > 0.0) {
            float y = p.y * 3.2;
            float path = 0.28 * sin(y * 2.2 + t * 16.0 + fi * 3.14)
                       + 0.12 * sin(y * 7.5 - t * 32.0);
            float d = abs(p.x - path - (fi - 1.0) * 0.42);
            bolt += flash * (exp(-d * 75.0) * 1.6 + exp(-d * 14.0) * 0.4);
        }
    }
    return bolt;
}

// ACES Filmic Tone Mapping for cinematic HDR range
vec3 aces_tonemap(vec3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

void main() {
    vec2 raw_uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // =========================================================================
    // 3. MACRO STORY TIMELINE (72.0s CYCLE)
    // =========================================================================
    // Act 1: The Celestial Genesis & Sacred Heart (0.0s - 20.0s)
    // Crossing 1: Solar Caustic Refraction Sweep (20.0s - 24.0s)
    // Act 2: Kaleidoscopic Vortex & Mandala Bloom (24.0s - 44.0s)
    // Crossing 2: Gravitational Singularity Ripple (44.0s - 48.0s)
    // Act 3: Supernova Storm & Relativistic Astral Fire (48.0s - 68.0s)
    // Crossing 3: Celestial Aurora Rebirth Curtain (68.0s - 72.0s)
    const float CYCLE_DURATION = 72.0;
    float macro_t = mod(u_time * 0.92, CYCLE_DURATION);

    // Continuous smoothly integrated zoom time (monotonic, derivative always > 0)
    // Act 1: calm smooth pace; Act 2: swirling flow; Act 3: rhythmic accelerating surges
    float base_zoom = 0.40 * u_time - 0.06 * cos(u_time * 0.85) - 0.025 * cos(u_time * 2.3);

    // Scene weights for continuous, butter-smooth blending
    float act1_w = smoothstep(24.0, 20.0, macro_t) + smoothstep(68.0, 72.0, macro_t);
    act1_w = clamp(act1_w, 0.0, 1.0);
    float act2_w = smoothstep(20.0, 24.0, macro_t) * (1.0 - smoothstep(44.0, 48.0, macro_t));
    float act3_w = smoothstep(44.0, 48.0, macro_t) * (1.0 - smoothstep(68.0, 72.0, macro_t));

    // Normalize weights to strictly partition unity
    float sum_w = act1_w + act2_w + act3_w + 1e-4;
    act1_w /= sum_w;
    act2_w /= sum_w;
    act3_w /= sum_w;

    // =========================================================================
    // 4. SCENE CHANGING CROSSINGS (OPTICAL CAUSTICS, WARPS & BEAM REFRACTIONS)
    // =========================================================================
    vec2 uv = raw_uv;
    vec3 crossing_fx = vec3(0.0);

    // Crossing 1 (20s - 24s): Diagonal Solar Caustic Refraction Wave
    if (macro_t >= 20.0 && macro_t < 24.0) {
        float ct = (macro_t - 20.0) / 4.0; // 0..1
        float sweep = (raw_uv.x + raw_uv.y) * 0.707 - (ct * 3.2 - 1.6);
        float refract_amp = exp(-abs(sweep) * 7.0) * (1.0 - ct * 0.3);
        uv += vec2(sin(sweep * 15.0), cos(sweep * 15.0)) * refract_amp * 0.038;
        
        float beam = exp(-abs(sweep) * 9.0) * 1.8;
        crossing_fx += vec3(1.2, 0.95, 0.55) * beam;
    }
    // Crossing 2 (44s - 48s): Gravitational Singularity Ripple Wave
    else if (macro_t >= 44.0 && macro_t < 48.0) {
        float ct = (macro_t - 44.0) / 4.0; // 0..1
        float r = length(raw_uv);
        float wave_front = ct * 1.35;
        float d_wave = abs(r - wave_front);
        float ripple = sin((r - wave_front) * 36.0) * exp(-d_wave * 12.0) * (1.0 - ct);
        uv += normalize(raw_uv + 1e-4) * ripple * 0.045;
        
        float pulse_beam = exp(-d_wave * 8.5) * (1.0 - ct * 0.5) * 2.0;
        crossing_fx += vec3(0.6, 0.95, 1.3) * pulse_beam;
    }
    // Crossing 3 (68s - 72s): Vertical Celestial Aurora Dissolve Curtain
    else if (macro_t >= 68.0) {
        float ct = (macro_t - 68.0) / 4.0; // 0..1
        float sweep = raw_uv.x - (ct * 2.8 - 1.4);
        float dissolve = exp(-abs(sweep) * 7.5);
        uv += vec2(0.0, sin(raw_uv.x * 20.0 + u_time * 8.0)) * dissolve * 0.035;
        
        float aurora_beam = exp(-abs(sweep) * 8.0) * 1.9;
        crossing_fx += vec3(1.15, 0.75, 1.25) * aurora_beam;
    }

    // =========================================================================
    // 5. CAMERA DYNAMICS: SWAY, VORTEX TWIST & RELATIVISTIC RECOIL
    // =========================================================================
    // Act 1: Subtle gentle breathing sway
    float rot1 = 0.08 * sin(u_time * 0.45);
    // Act 2: Hypnotic swirling vortex rotation
    float rot2 = u_time * 0.28 + 0.16 * sin(u_time * 0.75);
    // Act 3: Relativistic rapid oscillation & storm vibration
    float rot3 = 0.18 * sin(u_time * 1.4) + 0.05 * sin(u_time * 4.2);
    
    float cam_rot = rot1 * act1_w + rot2 * act2_w + rot3 * act3_w;
    float ca = cos(cam_rot), sa = sin(cam_rot);
    mat2 cam_mat = mat2(ca, -sa, sa, ca);
    vec2 p = cam_mat * uv;

    // Organic Heartbeat pulse
    float beat_osc = sin(u_time * TWO_PI * 1.25);
    float glow_pulse = beat_osc * beat_osc * (0.35 + 0.45 * act3_w);

    // =========================================================================
    // 6. CONTINUOUS SHEPARD 4-OCTAVE INFINITE ZOOM ENGINE
    // =========================================================================
    const float S = 3.2;
    const float lnS = 1.1631508; // ln(3.2)
    const int NUM_OCTAVES = 4;
    const int ITERATIONS = 12;

    vec3 total_color = vec3(0.0);
    float total_weight = 0.0;

    for (int o = 0; o < NUM_OCTAVES; o++) {
        float phase = fract(base_zoom + float(o) * 0.25);
        float scale = exp(phase * lnS);

        // Continuous quadratic Hanning crossfade envelope
        float w = 0.5 - 0.5 * cos(TWO_PI * phase);
        w = w * w;

        vec2 z = p * scale;
        float accum = 0.0;
        float min_trap = 1e8;
        float edge_trap = 1e8;

        for (int i = 0; i < ITERATIONS; i++) {
            // 1. Bilateral symmetry fold
            z.x = abs(z.x);

            // 2. Exact cardioid cleft fold
            z.y -= 0.58 * (sqrt(max(0.0, z.x) + 0.035) - 0.187);

            // 3. Spherical inversion (fractal chamber recursion)
            float r2 = dot(z, z) + 1e-4;
            if (r2 < 0.22) {
                z *= (1.0 / 0.22);
            } else if (r2 < 1.38) {
                z *= (1.0 / r2);
            }

            // 4. Harmonic variation per story act:
            // Act 1: Classic subtle dilation
            // Act 2: Blooming mandala dihedral rotation fold
            // Act 3: Relativistic storm fold dilation
            float fold_rot = 0.06 * sin(u_time * 0.4) * act1_w
                           + (0.32 * sin(u_time * 0.65 + float(i) * 0.4)) * act2_w
                           + (0.15 * sin(u_time * 1.8 + float(i) * 0.6)) * act3_w;
            float cfr = cos(fold_rot), sfr = sin(fold_rot);
            z = mat2(cfr, -sfr, sfr, cfr) * z;

            // Secondary box fold dynamically awakening in Act 2 & Act 3
            vec2 box_dim = vec2(0.04 * act2_w + 0.02 * act3_w, 0.08 * act2_w + 0.05 * act3_w);
            z = abs(z) - box_dim;

            // Scale expansion & offset
            float scale_exp = 1.48 + 0.04 * act2_w + 0.06 * act3_w;
            vec2 offset = vec2(0.18, 0.30) + vec2(0.02 * sin(u_time * 0.5), 0.02 * cos(u_time * 0.5)) * act2_w;
            z = z * scale_exp - offset;

            // 5. Heart orbit trap
            float hx = abs(z.x);
            float hy = z.y - 0.52 * (sqrt(hx + 0.035) - 0.187);
            float hd = length(vec2(hx, hy));

            min_trap = min(min_trap, hd);
            float et = abs(z.x * z.y);
            edge_trap = min(edge_trap, et);

            accum += exp(-3.5 * hd) + 0.5 * exp(-7.0 * et);
        }

        // Multi-frequency color mapping across acts
        float col_coord = accum * 0.18 + min_trap * 1.35 + u_time * 0.40 + float(o) * 0.25;
        
        vec3 col_layer = palette_act1(col_coord) * act1_w
                       + palette_act2(col_coord) * act2_w
                       + palette_act3(col_coord) * act3_w;

        // Radiant neon aura along heart contours
        float aura = exp(-1.2 * min_trap);
        vec3 neon = vec3(
            0.65 + 0.35 * sin(u_time * 1.6 + col_coord * 4.0),
            0.55 + 0.45 * cos(u_time * 1.9 + col_coord * 3.0),
            0.85 + 0.15 * sin(u_time * 2.2 + col_coord * 5.0)
        );
        col_layer = mix(col_layer, neon, aura * 0.65);

        // Core singularity beam with organic heartbeat pulse
        float core_dist = length(uv) * scale;
        float core_beam = exp(-7.0 * core_dist) * (1.0 + glow_pulse);
        vec3 core_tint = mix(vec3(1.2, 0.65, 0.95), vec3(1.4, 1.1, 0.6), act3_w);
        col_layer += core_tint * core_beam * 2.2;

        // Electric iridescent filament lines
        float edge_line = exp(-11.0 * edge_trap);
        vec3 edge_tint = mix(vec3(0.25, 0.95, 1.0), vec3(1.0, 0.85, 0.3), act3_w);
        col_layer += edge_tint * edge_line * 0.95;

        total_color += col_layer * w;
        total_weight += w;
    }

    vec3 scene_color = total_color / max(total_weight, 1e-5);

    // =========================================================================
    // 7. STORY ACTION PARTICLES & LIGHTNING DISCHARGES (ACT 3 & CROSSINGS)
    // =========================================================================
    if (act3_w > 0.05) {
        float bolts = fxLightning(p, u_time, 13.7) * act3_w;
        scene_color += vec3(1.2, 0.95, 0.65) * bolts * 0.85;
    }

    // Blend scene changing optical crossing beams
    scene_color += crossing_fx;

    // Peripheral chromatic dispersion
    float dist_sq = dot(raw_uv, raw_uv);
    scene_color.r += 0.06 * sin(dist_sq * 9.0 + u_time * 2.0) * sqrt(dist_sq);
    scene_color.b += 0.06 * cos(dist_sq * 8.0 - u_time * 2.5) * sqrt(dist_sq);

    // Soft peripheral vignette
    float vignette = clamp(1.0 - 0.24 * dist_sq, 0.0, 1.0);
    scene_color *= vignette;

    // ACES Filmic Tone Mapping for rich punchy vibrance without blowout
    vec3 final_color = aces_tonemap(scene_color * 1.28);

    fragColor = vec4(final_color, 1.0);
}
"#;
