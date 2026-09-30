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

// Sample glyph from 16x16 font atlas
// cell_idx in 0..255, local_uv in [0..1]
vec2 get_glyph_sample(int cell_idx, vec2 local_uv, float lod) {
    if (local_uv.x < 0.04 || local_uv.x > 0.96 || local_uv.y < 0.04 || local_uv.y > 0.96) {
        return vec2(0.0);
    }
    float c = float(cell_idx % 16);
    float r = float(cell_idx / 16);
    vec2 atlas_uv = vec2((c + clamp(local_uv.x, 0.0, 1.0)) / 16.0, (r + clamp(local_uv.y, 0.0, 1.0)) / 16.0);
    
    float stroke = textureLod(u_fontTexture, atlas_uv, lod).r;
    float glow = textureLod(u_fontTexture, atlas_uv, lod + 2.5).r;
    return vec2(stroke, glow);
}

// =========================================================================
// 2. MODERATED SUN FLARE EFFECTS (Crisp, non-blinding)
// =========================================================================
vec3 compute_sun_flare(vec2 uv, vec2 sun_pos, float time) {
    vec2 sun_d = uv - sun_pos;
    float sun_r = length(sun_d);
    float sun_phi = atan(sun_d.y, sun_d.x);

    // 1. Incandescent Solar Core & Corona (tuned for brilliance without washing out the entire screen)
    float disc = exp(-sun_r * 38.0) * 1.4;
    float inner_corona = exp(-sun_r * 11.0) * 0.55;
    float outer_corona = exp(-sun_r * 3.8) * 0.18;
    vec3 solar_core = vec3(1.2, 1.15, 1.0) * disc 
                    + vec3(1.10, 0.80, 0.40) * inner_corona
                    + vec3(0.85, 0.50, 0.20) * outer_corona;

    // 2. Multi-Harmonic Diffraction Starburst Rays (Sun Flare Spikes)
    float rays_8 = max(0.0, cos(8.0 * sun_phi + time * 0.28));
    float rays_12 = max(0.0, cos(12.0 * sun_phi - time * 0.19 + 0.6));
    float starburst = pow(rays_8, 6.0) * 0.7 + pow(rays_12, 8.0) * 0.45;
    float shimmer = 0.75 + 0.25 * hash11(floor(sun_phi * 36.0) + floor(time * 14.0));
    float ray_intensity = starburst * exp(-sun_r * 3.4) * shimmer;
    vec3 ray_col = vec3(1.15, 0.95, 0.60) * ray_intensity;

    // 3. Cinematic Anamorphic Horizontal Flare Beam
    vec2 anamorphic_d = vec2(sun_d.x * 0.98 + sun_d.y * 0.17, -sun_d.x * 0.17 + sun_d.y * 0.98);
    float streak = exp(-abs(anamorphic_d.y) * 55.0) * exp(-abs(anamorphic_d.x) * 1.6) * 0.65;
    vec3 streak_col = vec3(
        streak * 1.10,
        streak * 0.80,
        streak * 1.30
    );

    // 4. Secondary Ghost Flares & Rainbow Rings
    vec3 ghosts = vec3(0.0);
    vec2 flare_axis = -sun_pos;
    for (float g = 1.0; g <= 4.0; g += 1.0) {
        vec2 g_center = sun_pos + flare_axis * (g * 0.42);
        float g_dist = length(uv - g_center);
        float g_size = 0.04 * g;
        float g_ring = exp(-pow((g_dist - g_size) * 35.0, 2.0)) * 0.20;
        vec3 g_color = palette(g * 0.25 + time * 0.1);
        ghosts += g_color * g_ring;
    }

    return (solar_core + ray_col + streak_col + ghosts) * 0.55;
}

// =========================================================================
// 3. STAR TREK HYPERSPACE PARALLAX STARFIELD
// =========================================================================
vec3 render_startrek_starfield(vec2 p, float t, float warp_factor) {
    if (warp_factor <= 0.005) return vec3(0.0);
    
    vec3 stars = vec3(0.0);
    float r = length(p);
    float phi = atan(p.y, p.x);
    float phi_norm = (phi / TWO_PI) + 0.5;

    // 3 Parallax Depth Tiers: Fast Foreground, Midground, Deep Starlight
    for (int tier = 0; tier < 3; tier++) {
        float f_tier = float(tier);
        float num_rays = 44.0 + f_tier * 22.0;
        float sector = floor(phi_norm * num_rays);
        float s_hash = hash11(sector * 23.41 + f_tier * 91.17);
        float s_hash2 = hash11(sector * 67.89 + f_tier * 13.51);

        float ray_phi = (sector + 0.15 + 0.70 * s_hash) / num_rays * TWO_PI - PI;
        float ang_diff = abs(phi - ray_phi);
        if (ang_diff > PI) ang_diff = TWO_PI - ang_diff;
        float d_perp = r * sin(ang_diff);

        // Speed surges during hyperspace acceleration
        float speed = (2.4 + 5.5 * warp_factor) * (0.85 + 0.35 * f_tier);
        float z = fract(s_hash2 + t * speed * 0.18);
        
        float r_star = 0.04 + pow(z, 1.8) * 1.40;
        float streak_len = (0.04 + 0.38 * warp_factor) * pow(z, 1.2) * 0.70;
        
        float r_diff = r_star - r;
        if (r_diff >= -0.010 && r_diff <= streak_len && d_perp < 0.004) {
            float h = clamp(r_diff / max(1e-4, streak_len), 0.0, 1.0);
            
            float streak_core = exp(-d_perp * 1600.0) * (1.0 - h * 0.75);
            float streak_glow = exp(-d_perp * 400.0) * 0.25 * (1.0 - h * 0.6);
            
            float head_dist = length(p - vec2(cos(ray_phi), sin(ray_phi)) * r_star);
            float head = exp(-head_dist * 220.0) * 1.8;

            vec3 star_color = mix(vec3(0.35, 0.85, 1.2), vec3(0.95, 0.98, 1.0), 1.0 - h);
            if (s_hash > 0.82) star_color = mix(star_color, vec3(1.2, 0.95, 0.6), 0.6);

            stars += (star_color * (streak_core * 1.5 + streak_glow * 0.45) + vec3(1.3, 1.4, 1.7) * head) 
                   * (0.6 + 0.4 * s_hash);
        }
    }

    return stars * warp_factor;
}

// =========================================================================
// 4. MAIN SHADER: CLASSIC CYLINDRICAL MATRIX VORTEX TUNNEL
// =========================================================================
void main() {
    vec2 raw_uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Dynamic hyperspace warp & FOV speedup cycle (every 32s)
    float warp_cycle = mod(u_time * 0.12, 1.0);
    float warp_burst = smoothstep(0.70, 0.82, warp_cycle) * (1.0 - smoothstep(0.92, 1.0, warp_cycle));
    
    // FOV compression during hyperspace speedup
    float fov_scale = 1.0 - 0.22 * warp_burst;

    // Lens Barrel Distortion
    float r2 = dot(raw_uv, raw_uv);
    vec2 screen_uv = raw_uv * (1.0 + 0.07 * r2) * fov_scale;

    // Dynamic camera roll and sway
    float roll = 0.24 * sin(u_time * 0.40);
    float ca = cos(roll), sa = sin(roll);
    vec2 uv = mat2(ca, -sa, sa, ca) * screen_uv;

    // Cyber glitch horizontal slicing
    float glitch_trigger = step(0.968, sin(u_time * 2.9) * sin(u_time * 6.7));
    if (glitch_trigger > 0.5) {
        float slice = step(0.5, fract(gl_FragCoord.y * 0.02 + u_time * 8.0));
        if (slice > 0.5) {
            uv.x += 0.030 * sin(gl_FragCoord.y * 0.2 + u_time * 15.0);
        }
    }

    float r_len = length(uv);
    float phi = atan(uv.y, uv.x);

    vec3 final_color = vec3(0.0);

    // =========================================================================
    // LAYER 1: 3D CYLINDRICAL MATRIX VORTEX TUNNEL
    // =========================================================================
    float z = 1.0 / (r_len + 0.035);

    // Helical vortex coordinate
    float twist = z * 0.18 + sin(u_time * 0.55) * 0.38;
    float tunnel_phi = phi + twist;
    
    // Discretize into 44 angular columns and depth rows
    const float NUM_COLS = 44.0;
    float norm_u = (tunnel_phi / TWO_PI + 0.5) * NUM_COLS;
    float col_idx = floor(norm_u);
    float fract_u = fract(norm_u);

    // Forward flight down the vortex tunnel (accelerates during hyperspace warp)
    float tunnel_speed = 3.2 + 4.5 * warp_burst;
    float tunnel_v = z * 1.35 + u_time * tunnel_speed;
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

    // Cell character UV with margin padding (keeps character intact inside boundary)
    vec2 char_uv = vec2((fract_u - 0.12) / 0.76, (fract_v - 0.12) / 0.76);

    // Determine glyph: Numbers 0..9 (70..109) & Kanji (46..69) & Katakana (0..45) - NO RUNES
    int glyph = 0;
    bool is_deciphered = false;

    float decipher_cycle = fract(u_time * 0.18 + col_seed);
    int row_in_block = int(mod(row_idx, 16.0));
    if (decipher_cycle > 0.35 && row_in_block >= 0 && row_in_block < 8) {
        // Deciphered sacred glyph sequence: Kanji 46..69 & Numbers 70..109
        glyph = int(mod(col_seed * 23.0 + float(row_in_block) * 7.0, 70.0));
        is_deciphered = true;
    } else {
        // Fast-scrambling matrix digital code (Numbers 70..109, Katakana 0..45)
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
        vec3 decipher_shimmer = mix(neon_color, vec3(0.5, 1.0, 0.95), 0.35 + 0.15 * sin(u_time * 2.5 + float(glyph)));
        neon_color = decipher_shimmer * 1.25;
    }

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

    // =========================================================================
    // LAYER 2: 3D FLOATING HOLOGRAPHIC WORD BANNERS (INTACT, UNSCRAMBLED)
    // =========================================================================
    float ring_r = r_len * (1.6 + 0.4 * sin(u_time * 0.8));
    float ring_phi = phi - u_time * 0.5;
    const float RING_BANNERS = 8.0;
    float ring_u = (ring_phi / TWO_PI + 0.5) * RING_BANNERS;
    float ring_idx = floor(ring_u);
    float ring_fract_u = fract(ring_u);

    float ring_mask = smoothstep(0.12, 0.0, abs(ring_r - 0.72));
    if (ring_mask > 0.01) {
        // Floating holographic cyber banners (200..212) - Intact and completely visible
        int banner_glyph = 200 + int(mod(ring_idx + floor(u_time * 0.2), 13.0));
        vec2 banner_uv = vec2((ring_fract_u - 0.05) / 0.9, (fract(r_len * 4.0 - u_time * 0.2) - 0.1) / 0.8);
        vec2 b_sample = get_glyph_sample(banner_glyph, banner_uv, 0.0);
        
        vec3 holo_col = palette(ring_idx * 0.25 - u_time * 0.3) * 1.8;
        holo_col += vec3(0.4, 0.8, 1.0) * b_sample.y * 1.2;
        final_color += holo_col * (b_sample.x * 2.2 + b_sample.y * 0.7) * ring_mask;
    }

    // =========================================================================
    // LAYER 3: PSYCHEDELIC VORTEX CORE SINGULARITY
    // =========================================================================
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

    // =========================================================================
    // LAYER 4: STAR TREK PARALLAX STARFIELD (WARP STARS STREAMING FROM TUNNEL)
    // =========================================================================
    vec3 warp_stars = render_startrek_starfield(raw_uv, u_time, 0.35 + 0.65 * warp_burst);
    final_color += warp_stars;

    // =========================================================================
    // LAYER 5: MODERATED SUN FLARE EFFECTS
    // =========================================================================
    vec2 sun_origin = vec2(
        0.13 * sin(u_time * 0.36) + 0.05 * cos(u_time * 0.74),
        0.08 * cos(u_time * 0.43) + 0.04 * sin(u_time * 0.92)
    );
    vec3 sun_flare = compute_sun_flare(screen_uv, sun_origin, u_time);
    final_color += sun_flare;

    // =========================================================================
    // VFX PIPELINE
    // =========================================================================
    // 1. Radial Chromatic Aberration & Spectral Dispersion
    float ca_shift = r_len * 0.016;
    final_color.r *= (1.0 + ca_shift * 2.6);
    final_color.b *= (1.0 - ca_shift * 1.6);

    // 2. Phosphor CRT Scanlines & Cathode Glow
    float scanline = 0.90 + 0.10 * sin(gl_FragCoord.y * 2.6);
    final_color *= scanline;

    // 3. Cyber Matrix Glitch Color Inversion
    if (glitch_trigger > 0.5) {
        float slice = step(0.48, fract(gl_FragCoord.y * 0.015 + u_time * 5.0));
        if (slice > 0.5) {
            final_color = final_color.gbr * 1.35;
        }
    }

    // 4. Subtle Film Shimmer / Cyber Dust Grain
    float grain = (hash11(gl_FragCoord.x * 12.9898 + gl_FragCoord.y * 78.233 + fract(u_time) * 100.0) - 0.5) * 0.025;
    final_color += grain;

    // 5. Radial Vignette (Lens Darkening at Edges)
    float vignette = clamp(1.0 - 0.40 * pow(length(raw_uv), 2.2), 0.0, 1.0);
    final_color *= vignette;

    // 6. Contrast S-Curve with deep obsidian shadows and brilliant solar highlights
    vec3 c_pow = pow(max(final_color, vec3(0.0)), vec3(1.65));
    final_color = (c_pow / (c_pow + vec3(0.20))) * 1.40;

    fragColor = vec4(clamp(final_color, 0.0, 1.0), 1.0);
}
"#;
