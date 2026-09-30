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

// High-performance hash functions
float hash11(float p) {
    p = fract(p * 0.1031);
    p *= p + 33.33;
    p *= p + p;
    return fract(p);
}

// Multi-spectral psychedelic neon palette (Inigo Quilez cosine generator)
vec3 palette(float t) {
    vec3 a = vec3(0.5, 0.5, 0.5);
    vec3 b = vec3(0.5, 0.5, 0.5);
    vec3 c = vec3(1.0, 1.0, 1.0);
    vec3 d = vec3(0.0, 0.33, 0.67);
    return a + b * cos(6.2831853 * (c * t + d));
}

// Sample glyph from 16x16 font atlas
// cell_idx in 0..255, local_uv in [0..1]
vec2 get_glyph_sample(int cell_idx, vec2 local_uv, float lod) {
    if (local_uv.x < 0.04 || local_uv.x > 0.96 || local_uv.y < 0.04 || local_uv.y > 0.96) {
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



// Compute comprehensive Solar Flare / Sun Flare Effects
vec3 compute_sun_flare(vec2 uv, vec2 sun_pos, float time) {
    vec2 sun_d = uv - sun_pos;
    float sun_r = length(sun_d);
    float sun_phi = atan(sun_d.y, sun_d.x);

    // 1. Incandescent Solar Core & Corona (tuned for brilliance without washing out the entire screen)
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
    vec3 streak_col = vec3(
        streak * 1.35,
        streak * 0.95,
        streak * 1.55
    );

    // 4. Volumetric Crepuscular Godrays / Sunbeams through matrix rain
    float godray_wave = max(0.0, sin(sun_phi * 13.0 + time * 1.1) * cos(sun_phi * 5.0 - time * 0.5));
    float godrays = pow(godray_wave, 5.0) * exp(-sun_r * 1.8) * 1.1;
    vec3 godray_col = vec3(1.0, 0.8, 0.5) * godrays;

    // 5. Multi-Element Optical Lens Ghost Flares along optical axis
    vec2 axis = -sun_pos;
    // Chromatic rainbow halo ring
    float halo_d = abs(sun_r - 0.42);
    float halo = exp(-halo_d * halo_d * 160.0) * 0.65;
    vec3 halo_col = palette(sun_phi * 0.4 + time * 0.25) * halo;

    // Secondary lens aperture ghost circles
    vec2 ghost1_d = uv - axis * 0.45;
    float ghost1 = exp(-length(ghost1_d) * 14.0) * 0.45;
    vec2 ghost2_d = uv - axis * 0.95;
    float ghost2 = exp(-length(ghost2_d) * 8.5) * 0.35;
    vec2 ghost3_d = uv - axis * (-0.4);
    float ghost3 = exp(-length(ghost3_d) * 18.0) * 0.4;

    vec3 ghost_col = vec3(0.2, 0.9, 0.8) * ghost1 
                   + vec3(0.95, 0.5, 0.8) * ghost2
                   + vec3(1.0, 0.85, 0.3) * ghost3;

    vec3 flare_total = (solar_core + ray_col + streak_col + godray_col + halo_col + ghost_col)
                     * (0.85 + 0.15 * sin(time * 2.4));
    return flare_total;
}

void main() {
    vec2 raw_uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // ==========================================
    // SHADER VFX: BARREL DISTORTION / CURVED LENS
    // ==========================================
    float r2 = dot(raw_uv, raw_uv);
    vec2 screen_uv = raw_uv * (1.0 + 0.08 * r2);

    // Dynamic camera roll and sway
    float roll = 0.26 * sin(u_time * 0.40);
    float ca = cos(roll), sa = sin(roll);
    vec2 uv = mat2(ca, -sa, sa, ca) * screen_uv;

    // Cyber glitch horizontal slicing
    float glitch_trigger = step(0.965, sin(u_time * 2.9) * sin(u_time * 6.7));
    if (glitch_trigger > 0.5) {
        float slice = step(0.5, fract(gl_FragCoord.y * 0.02 + u_time * 8.0));
        if (slice > 0.5) {
            uv.x += 0.035 * sin(gl_FragCoord.y * 0.2 + u_time * 15.0);
        }
    }

    float r_len = length(uv);
    float phi = atan(uv.y, uv.x);

    vec3 final_color = vec3(0.0);

    // ==========================================
    // LAYER 1: 3D CYLINDRICAL MATRIX VORTEX TUNNEL
    // ==========================================
    float z = 1.0 / (r_len + 0.035);

    // Helical vortex coordinate
    float twist = z * 0.18 + sin(u_time * 0.55) * 0.38;
    float tunnel_phi = phi + twist;
    
    // Discretize into angular columns and depth rows
    const float NUM_COLS = 44.0;
    float norm_u = (tunnel_phi / 6.2831853 + 0.5) * NUM_COLS;
    float col_idx = floor(norm_u);
    float fract_u = fract(norm_u);

    // Forward flight down the vortex tunnel
    float tunnel_v = z * 1.35 + u_time * 3.2;
    float row_idx = floor(tunnel_v);
    float fract_v = fract(tunnel_v);

    // Column pseudorandom seeds
    float col_seed = hash11(col_idx * 13.37 + 7.19);
    float stream_speed = 0.6 + 0.8 * hash11(col_idx * 31.17);
    
    // Rain stream pulse down the column
    float stream_time = u_time * (4.0 * stream_speed) + col_seed * 128.0;
    float stream_pos = mod(row_idx + stream_time, 24.0);

    // Raindrop intensity curve: laser head + exponential tail
    float drop_head_dist = abs(stream_pos - 1.0);
    float head_intensity = exp(-drop_head_dist * 1.8);
    float tail_intensity = max(0.0, 1.0 - stream_pos / 18.0);
    bool is_head = (stream_pos < 1.4);

    // Cell character UV with margin padding
    vec2 char_uv = vec2((fract_u - 0.12) / 0.76, (fract_v - 0.12) / 0.76);

    // Determine glyph: Continuous deciphering cyber streams
    int glyph = 0;
    bool is_deciphered = false;

    // Stream cryptographic cycle:
    // Periodically, rain streams decipher into luminous Kanji and sacred runes
    float decipher_cycle = fract(u_time * 0.18 + col_seed);
    int row_in_block = int(mod(row_idx, 16.0));
    if (decipher_cycle > 0.35 && row_in_block >= 0 && row_in_block < 8) {
        // Deciphered sacred glyph sequence (Kanji 46..69 & Runes 70..109)
        glyph = int(mod(col_seed * 23.0 + float(row_in_block) * 7.0, 70.0));
        is_deciphered = true;
    } else {
        // Fast-scrambling matrix rain code
        glyph = int(mod(col_seed * 113.0 + row_idx * 7.0 + floor(u_time * (4.0 + col_seed * 6.0)), 110.0));
    }

    // Sample glyph strokes and bloom
    float lod = clamp((z - 2.0) * 0.15, 0.0, 3.0);
    vec2 glyph_sample = get_glyph_sample(glyph, char_uv, lod);
    float stroke = glyph_sample.x;
    float glow = glyph_sample.y;

    // Multi-spectral neon psychedelic color for this tunnel segment
    float color_phase = z * 0.08 + col_idx * 0.06 + u_time * 0.35;
    vec3 neon_color = palette(color_phase);

    if (is_deciphered) {
        // Deciphered glyphs pulse with an energetic crystalline luminescence
        vec3 decipher_shimmer = mix(neon_color, vec3(0.5, 1.0, 0.95), 0.35 + 0.15 * sin(u_time * 2.5 + float(glyph)));
        neon_color = decipher_shimmer * 1.25;
    }

    // Head raindrop is blinding laser white-hot core
    if (is_head) {
        neon_color = mix(neon_color, vec3(1.5, 1.7, 2.0), 0.85);
    }

    // Compute layer 1 glyph lighting
    float stream_light = 0.22 + 0.78 * tail_intensity;
    if (is_head) stream_light = 1.6;
    float glyph_vis = stroke * 1.6 + glow * 1.0;
    vec3 tunnel_color = neon_color * glyph_vis * stream_light;

    // Depth attenuation / atmospheric cyber fog
    float fog = smoothstep(32.0, 1.8, z);
    tunnel_color *= fog;

    final_color += tunnel_color;

    // ==========================================
    // LAYER 2: 3D FLOATING HOLOGRAPHIC WORD BANNERS
    // ==========================================
    float ring_r = r_len * (1.6 + 0.4 * sin(u_time * 0.8));
    float ring_phi = phi - u_time * 0.5;
    const float RING_BANNERS = 8.0;
    float ring_u = (ring_phi / 6.2831853 + 0.5) * RING_BANNERS;
    float ring_idx = floor(ring_u);
    float ring_fract_u = fract(ring_u);

    float ring_mask = smoothstep(0.12, 0.0, abs(ring_r - 0.72));
    if (ring_mask > 0.01) {
        // Floating holographic cyber banners (200..212)
        int banner_glyph = 200 + int(mod(ring_idx + floor(u_time * 0.2), 13.0));
        vec2 banner_uv = vec2((ring_fract_u - 0.05) / 0.9, (fract(r_len * 4.0 - u_time * 0.2) - 0.1) / 0.8);
        vec2 b_sample = get_glyph_sample(banner_glyph, banner_uv, 0.0);
        
        vec3 holo_col = palette(ring_idx * 0.25 - u_time * 0.3) * 1.8;
        holo_col += vec3(0.4, 0.8, 1.0) * b_sample.y * 1.2;
        final_color += holo_col * (b_sample.x * 2.0 + b_sample.y * 0.6) * ring_mask;
    }

    // ==========================================
    // LAYER 3: PSYCHEDELIC VORTEX CORE SINGULARITY
    // ==========================================
    float core_r = r_len;
    if (core_r < 0.22) {
        float core_spiral = phi * 3.0 + 1.0 / (core_r + 0.01) * 0.5 - u_time * 4.0;
        float core_col_idx = floor(mod(core_spiral * 3.0, 16.0));
        int core_glyph = 46 + int(mod(core_col_idx + floor(u_time * 6.0), 24.0));
        vec2 core_uv = vec2(fract(core_spiral), fract(core_r * 20.0));
        vec2 c_sample = get_glyph_sample(core_glyph, core_uv, 1.0);
        
        vec3 core_col = palette(u_time * 0.6 + core_r * 10.0);
        float core_glow = exp(-core_r * 14.0);
        final_color += (c_sample.x * core_col * 2.5 + core_col * core_glow * 1.5) * (1.0 - core_r / 0.22);
    }

    // ==========================================
    // LAYER 4: SUN FLARE EFFECTS (SOLAR BURST)
    // ==========================================
    vec2 sun_origin = vec2(
        0.13 * sin(u_time * 0.36) + 0.05 * cos(u_time * 0.74),
        0.08 * cos(u_time * 0.43) + 0.04 * sin(u_time * 0.92)
    );
    vec3 sun_flare = compute_sun_flare(screen_uv, sun_origin, u_time);
    final_color += sun_flare;

    // ==========================================
    // VFX PIPELINE
    // ==========================================
    // 1. Radial Chromatic Aberration & Spectral Dispersion
    float ca_shift = r_len * 0.018;
    final_color.r *= (1.0 + ca_shift * 2.8);
    final_color.b *= (1.0 - ca_shift * 1.8);

    // 2. Phosphor CRT Scanlines & Cathode Glow
    float scanline = 0.89 + 0.11 * sin(gl_FragCoord.y * 2.6);
    final_color *= scanline;

    // 3. Cyber Matrix Glitch Color Inversion
    if (glitch_trigger > 0.5) {
        float slice = step(0.48, fract(gl_FragCoord.y * 0.015 + u_time * 5.0));
        if (slice > 0.5) {
            final_color = final_color.gbr * 1.35;
        }
    }

    // 4. Subtle Film Shimmer / Cyber Dust Grain
    float grain = (hash11(gl_FragCoord.x * 12.9898 + gl_FragCoord.y * 78.233 + fract(u_time) * 100.0) - 0.5) * 0.03;
    final_color += grain;

    // 5. Radial Vignette (Lens Darkening at Edges)
    float vignette = clamp(1.0 - 0.42 * pow(length(raw_uv), 2.2), 0.0, 1.0);
    final_color *= vignette;

    // 6. Contrast S-Curve with deep obsidian shadows and brilliant solar highlights
    vec3 c_pow = pow(max(final_color, vec3(0.0)), vec3(1.68));
    final_color = (c_pow / (c_pow + vec3(0.20))) * 1.42;

    fragColor = vec4(clamp(final_color, 0.0, 1.0), 1.0);
}
"#;
