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
uniform sampler2D u_fontTexture;

out vec4 fragColor;

#define PI 3.14159265359
#define TWO_PI 6.28318530718

// =========================================================================
// 1. UTILITY FUNCTIONS & HIGH-PERFORMANCE HASHES
// =========================================================================
float hash11(float p) {
    p = fract(p * 0.1031);
    p *= p + 33.33;
    p *= p + p;
    return fract(p);
}

float hash21(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

// Multi-spectral psychedelic neon palette (Inigo Quilez cosine generator)
vec3 palette(float t) {
    vec3 a = vec3(0.5, 0.5, 0.5);
    vec3 b = vec3(0.5, 0.5, 0.5);
    vec3 c = vec3(1.0, 1.0, 1.0);
    vec3 d = vec3(0.0, 0.33, 0.67);
    return clamp(a + b * cos(TWO_PI * (c * t + d)), 0.0, 1.0);
}

vec3 palette_gold(float t) {
    vec3 a = vec3(0.68, 0.45, 0.22);
    vec3 b = vec3(0.55, 0.48, 0.35);
    vec3 c = vec3(1.1, 1.0, 0.8);
    vec3 d = vec3(0.02, 0.28, 0.58);
    return clamp(a + b * cos(TWO_PI * (c * t + d)), 0.0, 1.0);
}

// Sample glyph from 16x16 font atlas
// cell_idx in 0..255, local_uv in [0..1]
vec2 get_glyph_sample(int cell_idx, vec2 local_uv, float lod) {
    if (local_uv.x < 0.03 || local_uv.x > 0.97 || local_uv.y < 0.03 || local_uv.y > 0.97) {
        return vec2(0.0);
    }
    float c = float(cell_idx % 16);
    float r = float(cell_idx / 16);
    vec2 atlas_uv = vec2((c + clamp(local_uv.x, 0.0, 1.0)) / 16.0, (r + clamp(local_uv.y, 0.0, 1.0)) / 16.0);
    
    // Stroke in red channel, hardware-blurred bloom in higher mipmap level
    float stroke = textureLod(u_fontTexture, atlas_uv, lod).r;
    float glow = textureLod(u_fontTexture, atlas_uv, lod + 2.5).r;
    return vec2(stroke, glow);
}

// Procedural Lightning Arcs
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

// Compute comprehensive Solar Flare / Sun Flare Effects
vec3 compute_sun_flare(vec2 uv, vec2 sun_pos, float time) {
    vec2 sun_d = uv - sun_pos;
    float sun_r = length(sun_d);
    float sun_phi = atan(sun_d.y, sun_d.x);

    // 1. Incandescent Solar Core & Corona
    float disc = exp(-sun_r * 32.0) * 2.8;
    float inner_corona = exp(-sun_r * 8.5) * 1.1;
    float outer_corona = exp(-sun_r * 2.8) * 0.35;
    vec3 solar_core = vec3(1.3, 1.2, 1.0) * disc 
                    + vec3(1.15, 0.85, 0.45) * inner_corona
                    + vec3(0.95, 0.55, 0.25) * outer_corona;

    // 2. Multi-Harmonic Diffraction Starburst Rays (Sun Flare Spikes)
    float rays_8 = max(0.0, cos(8.0 * sun_phi + time * 0.28));
    float rays_12 = max(0.0, cos(12.0 * sun_phi - time * 0.19 + 0.6));
    float starburst = pow(rays_8, 6.0) * 1.4 + pow(rays_12, 8.0) * 0.9;
    float shimmer = 0.75 + 0.25 * hash11(floor(sun_phi * 36.0) + floor(time * 14.0));
    float ray_intensity = starburst * exp(-sun_r * 2.5) * shimmer;
    vec3 ray_col = vec3(1.25, 1.05, 0.7) * ray_intensity;

    // 3. Cinematic Anamorphic Horizontal Flare Beam
    vec2 anamorphic_d = vec2(sun_d.x * 0.98 + sun_d.y * 0.17, -sun_d.x * 0.17 + sun_d.y * 0.98);
    float streak = exp(-abs(anamorphic_d.y) * 45.0) * exp(-abs(anamorphic_d.x) * 1.3) * 1.4;
    vec3 streak_col = vec3(streak * 1.35, streak * 0.95, streak * 1.55);

    // 4. Volumetric Crepuscular Godrays through cyber rain
    float godray_wave = max(0.0, sin(sun_phi * 13.0 + time * 1.1) * cos(sun_phi * 5.0 - time * 0.5));
    float godrays = pow(godray_wave, 5.0) * exp(-sun_r * 1.8) * 1.1;
    vec3 godray_col = vec3(1.0, 0.8, 0.5) * godrays;

    // 5. Multi-Element Optical Lens Ghost Flares along optical axis
    vec2 axis = -sun_pos;
    float halo_d = abs(sun_r - 0.42);
    float halo = exp(-halo_d * halo_d * 160.0) * 0.65;
    vec3 halo_col = palette(sun_phi * 0.4 + time * 0.25) * halo;

    vec2 ghost1_d = uv - axis * 0.45;
    float ghost1 = exp(-length(ghost1_d) * 14.0) * 0.45;
    vec2 ghost2_d = uv - axis * 0.95;
    float ghost2 = exp(-length(ghost2_d) * 8.5) * 0.35;
    vec2 ghost3_d = uv - axis * (-0.4);
    float ghost3 = exp(-length(ghost3_d) * 18.0) * 0.4;

    vec3 ghost_col = vec3(0.2, 0.9, 0.8) * ghost1 
                   + vec3(0.95, 0.5, 0.8) * ghost2
                   + vec3(1.0, 0.85, 0.3) * ghost3;

    return (solar_core + ray_col + streak_col + godray_col + halo_col + ghost_col)
           * (0.85 + 0.15 * sin(time * 2.4));
}

// ACES Tone Mapping
vec3 aces_tonemap(vec3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

// =========================================================================
// 2. SCENE 1: THE GATEWAY TO CYBERSPACE (3D GROUND DATA OCEAN & SKY RAIN)
// =========================================================================
vec3 render_scene1_gateway(vec2 uv, float t) {
    vec3 col = vec3(0.0);
    
    // Perspective horizon tilt
    float y_cam = uv.y + 0.12;
    
    // Ground Data Grid (y_cam < 0)
    if (y_cam < -0.01) {
        float depth = 0.45 / (-y_cam);
        float world_x = uv.x * depth * 0.85;
        float world_z = depth + t * 2.8;
        
        float gx = abs(fract(world_x) - 0.5);
        float gz = abs(fract(world_z) - 0.5);
        
        float grid_lines = exp(-gx * 22.0) + exp(-gz * 22.0);
        float fog = smoothstep(18.0, 1.2, depth);
        
        vec3 grid_col = mix(vec3(0.05, 0.85, 0.65), vec3(0.95, 0.25, 0.75), fract(world_x * 0.1 + t * 0.1));
        col += grid_col * grid_lines * 1.5 * fog;
        
        // Ground data packets flashing along lines
        float packet = step(0.92, hash21(floor(vec2(world_x, world_z)) + floor(t * 4.0)));
        col += vec3(1.2, 1.4, 1.6) * packet * exp(-gx * 10.0) * exp(-gz * 10.0) * fog * 2.0;
    }
    
    // Sky Matrix Rain (y_cam >= -0.05)
    float col_w = 28.0;
    float sky_col_idx = floor((uv.x + 1.0) * 0.5 * col_w);
    float sky_seed = hash11(sky_col_idx * 13.7 + 5.2);
    
    float rain_speed = 1.8 + 2.2 * sky_seed;
    float rain_y = (uv.y + 1.0) * 8.0 + t * rain_speed + sky_seed * 32.0;
    float rain_row = floor(rain_y);
    float rain_pos = mod(rain_row, 18.0);
    
    float drop_head = exp(-abs(rain_pos - 1.0) * 2.2);
    float drop_tail = max(0.0, 1.0 - rain_pos / 15.0);
    
    vec2 char_uv = vec2(fract((uv.x + 1.0) * 0.5 * col_w * 0.9), fract(rain_y));
    int glyph = int(mod(sky_seed * 73.0 + rain_row * 3.0 + floor(t * 3.0), 90.0));
    vec2 g_samp = get_glyph_sample(glyph, char_uv, 0.5);
    
    vec3 rain_col = mix(vec3(0.1, 0.9, 0.4), vec3(0.2, 0.85, 1.0), sky_seed);
    if (rain_pos < 1.3) rain_col = vec3(1.6, 1.8, 2.0); // laser head
    
    float rain_light = (drop_head * 1.8 + drop_tail * 0.6) * (g_samp.x * 1.5 + g_samp.y * 0.8);
    col += rain_col * rain_light;
    
    // Glowing neon horizon line
    float horiz_dist = abs(y_cam);
    float horiz_glow = exp(-horiz_dist * 25.0) * 1.2 + exp(-horiz_dist * 5.0) * 0.35;
    vec3 horiz_col = vec3(0.3, 0.9, 1.2) * (0.8 + 0.2 * sin(t * 1.5));
    col += horiz_col * horiz_glow;
    
    return col;
}

// =========================================================================
// 3. SCENE 2: THE HYPER-SPEED HELICAL VORTEX TUNNEL
// =========================================================================
vec3 render_scene2_vortex(vec2 uv, float t) {
    vec3 col = vec3(0.0);
    float r_len = length(uv);
    float phi = atan(uv.y, uv.x);
    
    float z = 1.0 / (r_len + 0.035);
    float twist = z * 0.18 + sin(t * 0.55) * 0.38;
    float tunnel_phi = phi + twist;
    
    const float NUM_COLS = 44.0;
    float norm_u = (tunnel_phi / TWO_PI + 0.5) * NUM_COLS;
    float col_idx = floor(norm_u);
    float fract_u = fract(norm_u);

    float tunnel_v = z * 1.35 + t * 3.4;
    float row_idx = floor(tunnel_v);
    float fract_v = fract(tunnel_v);

    float col_seed = hash11(col_idx * 13.37 + 7.19);
    float stream_speed = 0.6 + 0.8 * hash11(col_idx * 31.17);
    float stream_time = t * (4.2 * stream_speed) + col_seed * 128.0;
    float stream_pos = mod(row_idx + stream_time, 24.0);

    float drop_head_dist = abs(stream_pos - 1.0);
    float head_intensity = exp(-drop_head_dist * 1.8);
    float tail_intensity = max(0.0, 1.0 - stream_pos / 18.0);
    bool is_head = (stream_pos < 1.4);

    vec2 char_uv = vec2((fract_u - 0.12) / 0.76, (fract_v - 0.12) / 0.76);

    // Cryptographic deciphering
    int glyph = 0;
    bool is_deciphered = false;
    float decipher_cycle = fract(t * 0.18 + col_seed);
    int row_in_block = int(mod(row_idx, 16.0));
    if (decipher_cycle > 0.35 && row_in_block >= 0 && row_in_block < 8) {
        glyph = int(mod(col_seed * 23.0 + float(row_in_block) * 7.0, 70.0));
        is_deciphered = true;
    } else {
        glyph = int(mod(col_seed * 113.0 + row_idx * 7.0 + floor(t * (4.0 + col_seed * 6.0)), 110.0));
    }

    float lod = clamp((z - 2.0) * 0.15, 0.0, 3.0);
    vec2 glyph_sample = get_glyph_sample(glyph, char_uv, lod);
    float stroke = glyph_sample.x;
    float glow = glyph_sample.y;

    float color_phase = z * 0.08 + col_idx * 0.06 + t * 0.35;
    vec3 neon_color = palette(color_phase);

    if (is_deciphered) {
        vec3 decipher_shimmer = mix(neon_color, vec3(0.5, 1.0, 0.95), 0.35 + 0.15 * sin(t * 2.5 + float(glyph)));
        neon_color = decipher_shimmer * 1.25;
    }
    if (is_head) {
        neon_color = mix(neon_color, vec3(1.5, 1.7, 2.0), 0.85);
    }

    float stream_light = 0.22 + 0.78 * tail_intensity;
    if (is_head) stream_light = 1.6;
    float glyph_vis = stroke * 1.6 + glow * 1.0;
    vec3 tunnel_color = neon_color * glyph_vis * stream_light;

    float fog = smoothstep(32.0, 1.8, z);
    col += tunnel_color * fog;

    // Floating 3D holographic word banners (200..212)
    float ring_r = r_len * (1.6 + 0.4 * sin(t * 0.8));
    float ring_phi = phi - t * 0.5;
    const float RING_BANNERS = 8.0;
    float ring_u = (ring_phi / TWO_PI + 0.5) * RING_BANNERS;
    float ring_idx = floor(ring_u);
    float ring_fract_u = fract(ring_u);

    float ring_mask = smoothstep(0.12, 0.0, abs(ring_r - 0.72));
    if (ring_mask > 0.01) {
        int banner_glyph = 200 + int(mod(ring_idx + floor(t * 0.2), 13.0));
        vec2 banner_uv = vec2((ring_fract_u - 0.05) / 0.9, (fract(r_len * 4.0 - t * 0.2) - 0.1) / 0.8);
        vec2 b_sample = get_glyph_sample(banner_glyph, banner_uv, 0.0);
        
        vec3 holo_col = palette(ring_idx * 0.25 - t * 0.3) * 1.8;
        holo_col += vec3(0.4, 0.8, 1.0) * b_sample.y * 1.2;
        col += holo_col * (b_sample.x * 2.0 + b_sample.y * 0.6) * ring_mask;
    }
    
    return col;
}

// =========================================================================
// 4. SCENE 3: THE AI SINGULARITY CORE & SACRED CYBER MANDALA
// =========================================================================
vec3 render_scene3_mandala(vec2 uv, float t) {
    vec3 col = vec3(0.0);
    float r = length(uv);
    float phi = atan(uv.y, uv.x);
    
    // 4 Concentric Counter-Rotating Sacred Glyph Rings
    const int NUM_RINGS = 4;
    for (int k = 0; k < NUM_RINGS; k++) {
        float fk = float(k);
        float R_k = 0.25 + fk * 0.18;
        float half_w = 0.065;
        
        float d_ring = abs(r - R_k);
        float ring_envelope = smoothstep(half_w, 0.0, d_ring);
        
        if (ring_envelope > 0.01) {
            float dir = (mod(fk, 2.0) == 0.0) ? 1.0 : -1.0;
            float speed = (0.50 - fk * 0.08) * dir;
            float rot_phi = phi - t * speed;
            
            float num_cells = 12.0 + fk * 8.0;
            float cell_u = (rot_phi / TWO_PI + 0.5) * num_cells;
            float cell_idx = floor(cell_u);
            float fract_cu = fract(cell_u);
            
            // Map cell coordinates
            float fract_cr = (r - (R_k - half_w)) / (2.0 * half_w);
            vec2 cell_uv = vec2((fract_cu - 0.08) / 0.84, (fract_cr - 0.08) / 0.84);
            
            // Deciphered sacred glyphs (Kanji 46..69 & Runes 70..109)
            int glyph = 46 + int(mod(cell_idx * 7.0 + fk * 13.0, 64.0));
            vec2 samp = get_glyph_sample(glyph, cell_uv, 0.0);
            
            vec3 ring_neon = palette_gold(fk * 0.25 + t * 0.1 + cell_idx * 0.05);
            vec3 ring_light = ring_neon * (samp.x * 2.2 + samp.y * 1.2) + vec3(1.2, 1.1, 0.8) * samp.x;
            
            col += ring_light * ring_envelope;
        }
        
        // Crystalline golden ring boundaries
        float border = exp(-abs(r - (R_k - half_w)) * 50.0) + exp(-abs(r - (R_k + half_w)) * 50.0);
        col += vec3(1.0, 0.85, 0.4) * border * 0.45;
    }
    
    // Core Singularity Flare
    vec2 sun_origin = vec2(0.0);
    vec3 solar_burst = compute_sun_flare(uv, sun_origin, t);
    col += solar_burst;
    
    // Crackling Lightning Arcs bridging the rings
    float bolts = fxLightning(uv, t, 37.1);
    col += vec3(1.0, 0.9, 0.6) * bolts * 0.75;
    
    return col;
}

// =========================================================================
// 5. MAIN COMPOSITING PIPELINE WITH GENTLE INTERLEAVING
// =========================================================================
void main() {
    vec2 raw_uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Barrel distortion / Curved cyber lens
    float r2 = dot(raw_uv, raw_uv);
    vec2 screen_uv = raw_uv * (1.0 + 0.06 * r2);

    // =========================================================================
    // MACRO STORY CYCLE (72.0s PERIOD)
    // Chapter 1: The Gateway to Cyberspace (0.0s - 20.0s)
    // Crossing 1: Defragmentation Pulse Wave (20.0s - 24.0s)
    // Chapter 2: Helical Vortex & Data Torrent (24.0s - 46.0s)
    // Crossing 2: Gravitational Singularity Ripple (46.0s - 50.0s)
    // Chapter 3: AI Singularity Core & Sacred Mandala (50.0s - 68.0s)
    // Crossing 3: Quantum Bloom Rebirth Wash (68.0s - 72.0s)
    // =========================================================================
    const float CYCLE_DURATION = 72.0;
    float macro_t = mod(u_time * 0.92, CYCLE_DURATION);

    // Dynamic camera rolls evolving per chapter
    float roll1 = 0.06 * sin(u_time * 0.35);
    float roll2 = 0.28 * sin(u_time * 0.40);
    float roll3 = 0.12 * sin(u_time * 0.25);

    // Scene weights with smoothstep easing (partition of unity)
    float sc1_w = smoothstep(24.0, 20.0, macro_t) + smoothstep(68.0, 72.0, macro_t);
    sc1_w = clamp(sc1_w, 0.0, 1.0);
    float sc2_w = smoothstep(20.0, 24.0, macro_t) * (1.0 - smoothstep(46.0, 50.0, macro_t));
    float sc3_w = smoothstep(46.0, 50.0, macro_t) * (1.0 - smoothstep(68.0, 72.0, macro_t));

    float sum_w = sc1_w + sc2_w + sc3_w + 1e-4;
    sc1_w /= sum_w;
    sc2_w /= sum_w;
    sc3_w /= sum_w;

    float cam_roll = roll1 * sc1_w + roll2 * sc2_w + roll3 * sc3_w;
    float ca = cos(cam_roll), sa = sin(cam_roll);
    vec2 p = mat2(ca, -sa, sa, ca) * screen_uv;

    // Cyber glitch horizontal slicing (occurs intermittently during chapter transitions)
    float glitch_trigger = step(0.97, sin(u_time * 3.1) * sin(u_time * 7.3));
    if (glitch_trigger > 0.5) {
        float slice = step(0.5, fract(gl_FragCoord.y * 0.02 + u_time * 8.0));
        if (slice > 0.5) {
            p.x += 0.03 * sin(gl_FragCoord.y * 0.2 + u_time * 15.0);
        }
    }

    // =========================================================================
    // SCENE CHANGING CROSSING WARPS & CAUSTIC BEAMS
    // =========================================================================
    vec2 p_warped = p;
    vec3 crossing_fx = vec3(0.0);

    // Crossing 1 (20s - 24s): Horizontal Defragmentation Pulse Wave
    if (macro_t >= 20.0 && macro_t < 24.0) {
        float ct = (macro_t - 20.0) / 4.0;
        float sweep = p.y - (ct * 2.4 - 1.2);
        p_warped.x += sin(sweep * 25.0) * exp(-abs(sweep) * 8.0) * 0.035;
        
        float beam = exp(-abs(sweep) * 12.0) * 2.2;
        crossing_fx += vec3(0.3, 0.95, 1.2) * beam;
    }
    // Crossing 2 (46s - 50s): Gravitational Singularity Ripple Wave
    else if (macro_t >= 46.0 && macro_t < 50.0) {
        float ct = (macro_t - 46.0) / 4.0;
        float r = length(p);
        float wave = abs(r - ct * 1.35);
        p_warped += normalize(p + 1e-4) * sin((r - ct * 1.35) * 32.0) * exp(-wave * 11.0) * 0.045 * (1.0 - ct);
        
        float pulse_beam = exp(-wave * 8.5) * (1.0 - ct * 0.4) * 2.0;
        crossing_fx += vec3(1.2, 0.85, 0.4) * pulse_beam;
    }
    // Crossing 3 (68s - 72s): Quantum Expansion Wash
    else if (macro_t >= 68.0) {
        float ct = (macro_t - 68.0) / 4.0;
        float wash_pulse = pow(ct, 2.0) * 1.8;
        crossing_fx += vec3(0.8, 1.1, 1.3) * wash_pulse * exp(-length(p) * 2.0);
    }

    // Evaluate each scene
    vec3 color_scene1 = (sc1_w > 0.001) ? render_scene1_gateway(p_warped, u_time) : vec3(0.0);
    vec3 color_scene2 = (sc2_w > 0.001) ? render_scene2_vortex(p_warped, u_time) : vec3(0.0);
    vec3 color_scene3 = (sc3_w > 0.001) ? render_scene3_mandala(p_warped, u_time) : vec3(0.0);

    // Gently interleave the scenes
    vec3 final_color = color_scene1 * sc1_w 
                     + color_scene2 * sc2_w 
                     + color_scene3 * sc3_w;

    // Apply crossing optical beams
    final_color += crossing_fx;

    // Subtle sun flare present in Scene 2 background as well
    if (sc2_w > 0.01) {
        vec2 sun_origin = vec2(
            0.12 * sin(u_time * 0.35),
            0.08 * cos(u_time * 0.42)
        );
        final_color += compute_sun_flare(p_warped, sun_origin, u_time) * (0.35 * sc2_w);
    }

    // =========================================================================
    // VFX POST-PROCESSING PIPELINE
    // =========================================================================
    // 1. Radial Chromatic Aberration
    float r_lens = length(p);
    float ca_shift = r_lens * 0.015;
    final_color.r *= (1.0 + ca_shift * 2.5);
    final_color.b *= (1.0 - ca_shift * 1.6);

    // 2. Phosphor CRT Scanlines
    float scanline = 0.90 + 0.10 * sin(gl_FragCoord.y * 2.6);
    final_color *= scanline;

    // 3. Cyber Matrix Glitch Color Inversion
    if (glitch_trigger > 0.5) {
        float slice = step(0.48, fract(gl_FragCoord.y * 0.015 + u_time * 5.0));
        if (slice > 0.5) {
            final_color = final_color.gbr * 1.25;
        }
    }

    // 4. Film Shimmer / Cyber Dust Grain
    float grain = (hash11(gl_FragCoord.x * 12.9898 + gl_FragCoord.y * 78.233 + fract(u_time) * 100.0) - 0.5) * 0.025;
    final_color += grain;

    // 5. Radial Vignette
    float vignette = clamp(1.0 - 0.38 * pow(length(raw_uv), 2.2), 0.0, 1.0);
    final_color *= vignette;

    // 6. ACES Filmic Tone Mapping for deep pitch-black shadows & electric highlights
    vec3 mapped = aces_tonemap(final_color * 1.30);

    fragColor = vec4(mapped, 1.0);
}
"#;
