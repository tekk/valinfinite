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
    if (local_uv.x <= 0.03 || local_uv.x >= 0.97 || local_uv.y <= 0.03 || local_uv.y >= 0.97) {
        return vec2(0.0);
    }
    float c = float(cell_idx % 16);
    float r = float(cell_idx / 16);
    vec2 atlas_uv = vec2((c + clamp(local_uv.x, 0.0, 1.0)) / 16.0, (r + clamp(local_uv.y, 0.0, 1.0)) / 16.0);
    
    // Stroke in red channel, hardware-blurred bloom in subtle higher mipmap
    float stroke = textureLod(u_fontTexture, atlas_uv, clamp(lod, 0.0, 1.5)).r;
    float glow = textureLod(u_fontTexture, atlas_uv, clamp(lod + 0.8, 0.0, 2.2)).r;
    return vec2(stroke, glow);
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
// 1.5. INTACT INTENDED WORDS CATALOG (CONSECUTIVE, UNSCRAMBLED GLYPHS)
// Words: VALIKA, PETO, TEKK, VALI, TEKKO, ヴァリカ, ペト, テック, テッコ
// Guaranteed intact without interleaving dots or random characters
// =========================================================================
int get_intended_word_glyph(int word_id, int pos) {
    // Word 0: VALIKA (6 letters) -> V(110), A(111), L(112), I(113), K(114), A(111)
    if (word_id == 0) {
        if (pos == 0) return 110;
        if (pos == 1) return 111;
        if (pos == 2) return 112;
        if (pos == 3) return 113;
        if (pos == 4) return 114;
        return 111;
    }
    // Word 1: PETO (4 letters) -> P(115), E(116), T(117), O(118)
    if (word_id == 1) {
        if (pos == 0) return 115;
        if (pos == 1) return 116;
        if (pos == 2) return 117;
        return 118;
    }
    // Word 2: TEKK (4 letters) -> T(117), E(116), K(114), K(114)
    if (word_id == 2) {
        if (pos == 0) return 117;
        if (pos == 1) return 116;
        if (pos == 2) return 114;
        return 114;
    }
    // Word 3: VALI (4 letters) -> V(110), A(111), L(112), I(113)
    if (word_id == 3) {
        if (pos == 0) return 110;
        if (pos == 1) return 111;
        if (pos == 2) return 112;
        return 113;
    }
    // Word 4: TEKKO (5 letters) -> T(117), E(116), K(114), K(114), O(118)
    if (word_id == 4) {
        if (pos == 0) return 117;
        if (pos == 1) return 116;
        if (pos == 2) return 114;
        if (pos == 3) return 114;
        return 118;
    }
    // Word 5: ヴァリカ (4 letters) -> 119, 120, 121, 122
    if (word_id == 5) {
        return 119 + pos;
    }
    // Word 6: ペト (2 letters) -> 123, 124
    if (word_id == 6) {
        return 123 + pos;
    }
    // Word 7: テック (3 letters) -> 125, 126, 127
    if (word_id == 7) {
        return 125 + pos;
    }
    // Word 8: テッコ (3 letters) -> 125, 126, 128
    if (pos == 0) return 125;
    if (pos == 1) return 126;
    return 128;
}

int get_word_length(int word_id) {
    if (word_id == 0) return 6; // VALIKA
    if (word_id == 1) return 4; // PETO
    if (word_id == 2) return 4; // TEKK
    if (word_id == 3) return 4; // VALI
    if (word_id == 4) return 5; // TEKKO
    if (word_id == 5) return 4; // ヴァリカ
    if (word_id == 6) return 2; // ペト
    if (word_id == 7) return 3; // テック
    return 3;                   // テッコ
}

// =========================================================================
// 1.6. STAR TREK HYPERSPACE PARALLAX STARFIELD
// (Radial warp streaks & sparkling star heads emerging during scene transitions)
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
        float num_rays = 48.0 + f_tier * 24.0;
        float sector = floor(phi_norm * num_rays);
        float s_hash = hash11(sector * 23.41 + f_tier * 91.17);
        float s_hash2 = hash11(sector * 67.89 + f_tier * 13.51);

        float ray_phi = (sector + 0.15 + 0.70 * s_hash) / num_rays * TWO_PI - PI;
        float ang_diff = abs(phi - ray_phi);
        if (ang_diff > PI) ang_diff = TWO_PI - ang_diff;
        float d_perp = r * sin(ang_diff);

        // Hyperspace speed forward: speeds up dramatically during warp
        float speed = (2.2 + 5.8 * warp_factor) * (0.85 + 0.35 * f_tier);
        float z = fract(s_hash2 + t * speed * 0.18);
        
        // Star emerges from center vanishing point (z=0 -> r=0.04) and races outward (z=1 -> r=1.45)
        float r_star = 0.04 + pow(z, 1.8) * 1.40;
        float streak_len = (0.04 + 0.40 * warp_factor) * pow(z, 1.2) * 0.75;
        
        float r_diff = r_star - r;
        if (r_diff >= -0.010 && r_diff <= streak_len && d_perp < 0.004) {
            float h = clamp(r_diff / max(1e-4, streak_len), 0.0, 1.0);
            
            // Needle-thin razor streak core (Star Trek style)
            float streak_core = exp(-d_perp * 1600.0) * (1.0 - h * 0.75);
            float streak_glow = exp(-d_perp * 400.0) * 0.25 * (1.0 - h * 0.6);
            
            float head_dist = length(p - vec2(cos(ray_phi), sin(ray_phi)) * r_star);
            float head = exp(-head_dist * 220.0) * 1.8;

            // Star Trek warp color: brilliant diamond white core, electric blue & cyan aura
            vec3 star_color = mix(vec3(0.35, 0.85, 1.2), vec3(0.95, 0.98, 1.0), 1.0 - h);
            if (s_hash > 0.82) star_color = mix(star_color, vec3(1.2, 0.95, 0.6), 0.6);

            stars += (star_color * (streak_core * 1.6 + streak_glow * 0.5) + vec3(1.4, 1.5, 1.8) * head) 
                   * (0.6 + 0.4 * s_hash);
        }
    }

    return stars * warp_factor;
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
        float fog = 1.0 - smoothstep(1.2, 18.0, depth);
        
        vec3 grid_col = mix(vec3(0.05, 0.85, 0.65), vec3(0.95, 0.25, 0.75), fract(world_x * 0.1 + t * 0.1));
        col += grid_col * grid_lines * 1.5 * fog;
        
        // Ground data packets flashing along lines
        float packet = step(0.92, hash21(floor(vec2(world_x, world_z)) + floor(t * 4.0)));
        col += vec3(1.2, 1.4, 1.6) * packet * exp(-gx * 10.0) * exp(-gz * 10.0) * fog * 2.0;
    }
    
    // Sky Matrix Rain (y_cam >= -0.05)
    // Strictly aligned columns: characters are centered and padded inside cell polygons
    float col_w = 26.0;
    float norm_x = (uv.x + 1.0) * 0.5 * col_w;
    float sky_col_idx = floor(norm_x);
    float sky_seed = hash11(sky_col_idx * 13.7 + 5.2);
    
    float rain_speed = 1.6 + 2.0 * sky_seed;
    float rain_y = (uv.y + 1.0) * 8.0 + t * rain_speed + sky_seed * 32.0;
    float rain_row = floor(rain_y);
    float rain_pos = mod(rain_row, 18.0);
    
    float drop_head = exp(-abs(rain_pos - 1.0) * 2.2);
    float drop_tail = max(0.0, 1.0 - rain_pos / 15.0);
    
    float cell_fx = fract(norm_x);
    float cell_fy = fract(rain_y);

    // Padding ensures glyphs remain strictly centered, intact, and never bleed across borders
    float pad_x = 0.16;
    float pad_y = 0.14;
    vec2 char_uv = vec2(
        (cell_fx - pad_x) / (1.0 - 2.0 * pad_x),
        (cell_fy - pad_y) / (1.0 - 2.0 * pad_y)
    );
    
    float char_mask = smoothstep(0.0, 0.12, char_uv.x) * (1.0 - smoothstep(0.88, 1.0, char_uv.x))
                    * smoothstep(0.0, 0.12, char_uv.y) * (1.0 - smoothstep(0.88, 1.0, char_uv.y));
    
    // Intended words appear intact and consecutive across rows without scrambling:
    int glyph = 0;
    bool is_intended_word = false;

    float col_h = hash11(sky_col_idx * 17.31 + 4.19);
    int word_id = int(mod(col_h * 9.0 + floor(t * 0.10), 9.0));
    int word_len = get_word_length(word_id);
    int row_in_block = int(mod(rain_row, 16.0));
    int word_start = int(mod(col_h * 11.0, 16.0 - float(word_len)));

    if (row_in_block >= word_start && row_in_block < word_start + word_len) {
        int pos = row_in_block - word_start;
        glyph = get_intended_word_glyph(word_id, pos);
        is_intended_word = true;
    } else {
        // Surrounding matrix characters: katakana (0..45), kanji (46..69), runes (70..109), symbols (130..148)
        glyph = int(mod(sky_seed * 73.0 + rain_row * 3.0 + floor(t * 3.0), 105.0));
        if (glyph >= 46 && glyph < 70 && hash11(float(glyph) + sky_col_idx) > 0.45) {
            glyph += 24; // shift into runes
        }
    }

    vec2 g_samp = (char_mask > 0.001) ? get_glyph_sample(glyph, char_uv, 0.5) * char_mask : vec2(0.0);
    
    vec3 rain_col = mix(vec3(0.1, 0.9, 0.4), vec3(0.2, 0.85, 1.0), sky_seed);
    if (is_intended_word) {
        // Intact intended words pulse with crystalline cyber-sapphire & gold luminescence
        vec3 word_lum = mix(vec3(0.2, 0.95, 1.2), vec3(1.2, 1.1, 0.85), 0.5 + 0.5 * sin(t * 2.5 + float(glyph)));
        rain_col = word_lum * 1.35;
    } else if (rain_pos < 1.3) {
        rain_col = vec3(1.25, 1.35, 1.5); // laser head (moderated flare)
    }
    
    float glyph_vis = g_samp.x * 1.6 + g_samp.x * g_samp.y * 0.8;
    float rain_light = (drop_head * 1.4 + drop_tail * 0.6) * glyph_vis;
    if (is_intended_word) rain_light = max(rain_light, 0.95 * (g_samp.x * 1.8 + g_samp.x * g_samp.y * 0.9));
    col += rain_col * rain_light;
    
    // Glowing neon horizon line (moderated flare)
    float horiz_dist = abs(y_cam);
    float horiz_glow = exp(-horiz_dist * 25.0) * 0.45 + exp(-horiz_dist * 5.0) * 0.12;
    vec3 horiz_col = vec3(0.3, 0.9, 1.2) * (0.8 + 0.2 * sin(t * 1.5));
    col += horiz_col * horiz_glow;
    
    return col;
}

// =========================================================================
// 3. SCENE 2: THE HYPER-SPEED HELICAL VORTEX WORMHOLE TUNNEL
// =========================================================================
vec3 render_scene2_vortex(vec2 uv, float t) {
    vec3 col = vec3(0.0);
    float r_len = length(uv);
    float phi = atan(uv.y, uv.x);
    
    float z = 1.0 / (r_len + 0.035);
    float twist = z * 0.18 + sin(t * 0.55) * 0.38;
    float tunnel_phi = phi + twist;
    
    const float NUM_COLS = 40.0;
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

    // Padding ensures characters remain centered and never escape cylindrical polygon cells
    float pad_u = 0.18;
    float pad_v = 0.15;
    vec2 char_uv = vec2(
        (fract_u - pad_u) / (1.0 - 2.0 * pad_u),
        (fract_v - pad_v) / (1.0 - 2.0 * pad_v)
    );
    float char_bounds = smoothstep(0.0, 0.14, char_uv.x) * (1.0 - smoothstep(0.86, 1.0, char_uv.x))
                      * smoothstep(0.0, 0.14, char_uv.y) * (1.0 - smoothstep(0.86, 1.0, char_uv.y));

    // Cryptographic deciphering
    // Cryptographic deciphering into intact intended words
    int glyph = 0;
    bool is_intended_word = false;

    float col_h = hash11(col_idx * 19.17 + 8.31);
    int word_id = int(mod(col_h * 9.0 + floor(t * 0.10), 9.0));
    int word_len = get_word_length(word_id);

    int row_in_block = int(mod(row_idx, 16.0));
    int word_start = int(mod(col_h * 11.0, 16.0 - float(word_len)));

    float decipher_cycle = fract(t * 0.18 + col_seed);
    if (decipher_cycle > 0.30 && row_in_block >= word_start && row_in_block < word_start + word_len) {
        int pos = row_in_block - word_start;
        glyph = get_intended_word_glyph(word_id, pos);
        is_intended_word = true;
    } else {
        // Surrounding matrix stream: Katakana (0..45), Kanji (46..69), and Numbers (70..109)
        // Strictly numbers instead of runes!
        float char_sel = hash11(col_seed * 53.1 + row_idx * 11.3);
        if (char_sel < 0.45) {
            glyph = int(mod(col_seed * 46.0 + row_idx, 46.0)); // Katakana
        } else if (char_sel < 0.75) {
            glyph = 70 + int(mod(col_seed * 10.0 + row_idx, 10.0)); // Numbers 0-9
        } else {
            glyph = 46 + int(mod(col_seed * 24.0 + row_idx, 24.0)); // Kanji
        }
    }

    float lod = clamp((z - 2.0) * 0.15, 0.0, 3.0);
    vec2 glyph_sample = (char_bounds > 0.001) ? get_glyph_sample(glyph, char_uv, lod) * char_bounds : vec2(0.0);
    float stroke = glyph_sample.x;
    float glow = glyph_sample.y;

    float color_phase = z * 0.08 + col_idx * 0.06 + t * 0.35;
    vec3 neon_color = palette(color_phase);

    if (is_intended_word) {
        vec3 decipher_shimmer = mix(vec3(0.2, 0.95, 1.2), vec3(1.15, 1.1, 0.85), 0.5 + 0.5 * sin(t * 2.5 + float(glyph)));
        neon_color = decipher_shimmer * 1.35;
    } else if (is_head) {
        neon_color = mix(neon_color, vec3(1.25, 1.35, 1.5), 0.85); // moderated flare
    }

    float stream_light = 0.22 + 0.78 * tail_intensity;
    if (is_intended_word) stream_light = max(stream_light, 1.1);
    else if (is_head) stream_light = 1.3;
    float glyph_vis = stroke * 1.8 + stroke * glow * 0.9;
    vec3 tunnel_color = neon_color * glyph_vis * stream_light;

    float fog = 1.0 - smoothstep(1.8, 32.0, z);
    col += tunnel_color * fog;

    // Floating 3D holographic word banners (200..208)
    float ring_r = r_len * (1.6 + 0.4 * sin(t * 0.8));
    float ring_phi = phi - t * 0.5;
    const float RING_BANNERS = 8.0;
    float ring_u = (ring_phi / TWO_PI + 0.5) * RING_BANNERS;
    float ring_idx = floor(ring_u);
    float ring_fract_u = fract(ring_u);

    float ring_mask = 1.0 - smoothstep(0.0, 0.12, abs(ring_r - 0.72));
    if (ring_mask > 0.01) {
        int banner_glyph = 200 + int(mod(ring_idx + floor(t * 0.2), 9.0));
        vec2 banner_uv = vec2((ring_fract_u - 0.08) / 0.84, (fract(r_len * 4.0 - t * 0.2) - 0.12) / 0.76);
        float b_bounds = smoothstep(0.0, 0.08, banner_uv.x) * (1.0 - smoothstep(0.92, 1.0, banner_uv.x))
                       * smoothstep(0.0, 0.08, banner_uv.y) * (1.0 - smoothstep(0.92, 1.0, banner_uv.y));
        vec2 b_sample = (b_bounds > 0.001) ? get_glyph_sample(banner_glyph, banner_uv, 0.0) * b_bounds : vec2(0.0);
        
        vec3 holo_col = palette(ring_idx * 0.25 - t * 0.3) * 1.5;
        holo_col += vec3(0.4, 0.8, 1.0) * b_sample.y * 1.0;
        col += holo_col * (b_sample.x * 1.8 + b_sample.y * 0.5) * ring_mask;
    }

    // Deep central singularity glow (moderated flare)
    col += vec3(0.15, 0.65, 1.0) * exp(-r_len * 3.8) * 0.30;
    
    return col;
}

// =========================================================================
// 4. SCENE 3: DYNAMIC QUANTUM PLEXUS & NEURAL CONSTELLATION MESH
// (Continuous global node constellation: moving vertices, distance-based
//  adaptive mesh lines, traveling data pulses, and zero cell boundaries!)
// =========================================================================
vec3 render_scene3_plexus(vec2 uv, float t) {
    vec3 col = vec3(0.0);

    // Subtle gentle ambient space drift
    float rot_a = t * 0.035;
    mat2 rot_m = mat2(cos(rot_a), -sin(rot_a), sin(rot_a), cos(rot_a));
    vec2 p = rot_m * uv;

    // Aspect ratio scaling to maintain balanced node grid in landscape & portrait
    float ar = u_resolution.x / u_resolution.y;
    vec2 aspect_scale = (ar >= 1.0) ? vec2(ar * 0.82, 0.92) : vec2(0.92, (1.0 / ar) * 0.82);

    // Compute 20 primary nodes
    const int NUM_NODES = 20;
    vec2 nodes[NUM_NODES];
    vec3 node_cols[NUM_NODES];
    float node_seeds[NUM_NODES];

    for (int k = 0; k < NUM_NODES; k++) {
        float fk = float(k);
        float s1 = hash11(fk * 17.31 + 4.19);
        float s2 = hash11(fk * 31.73 + 8.61);
        node_seeds[k] = s1;

        // 5 columns x 4 rows
        int c = k % 5;
        int r = k / 5;
        vec2 norm_pos = vec2(
            (float(c) - 2.0) * 0.22,
            (float(r) - 1.5) * 0.22
        );
        vec2 anchor = norm_pos * aspect_scale;

        // Multi-frequency organic wandering
        vec2 wander = vec2(
            sin(t * (0.42 + 0.28 * s1) + s2 * TWO_PI) * 0.16 + sin(t * 0.20 + fk * 1.3) * 0.05,
            cos(t * (0.38 + 0.32 * s2) + s1 * TWO_PI) * 0.14 + cos(t * 0.24 + fk * 1.7) * 0.05
        );
        nodes[k] = anchor + wander;
        node_cols[k] = palette(s1 * 0.75 + t * 0.06);
    }

    // 1. Draw glowing vertices (nodes)
    for (int i = 0; i < NUM_NODES; i++) {
        float d_node = length(p - nodes[i]);
        if (d_node < 0.14) {
            // Incandescent core
            float core = exp(-d_node * 75.0) * 2.0;
            // Soft outer corona
            float corona = exp(-d_node * 22.0) * 0.40;
            // High-tech holographic ring
            float ring_radius = 0.022 + 0.004 * sin(t * 3.0 + node_seeds[i] * 6.28);
            float ring = (1.0 - smoothstep(0.0, 0.004, abs(d_node - ring_radius))) * 0.75;

            vec3 nc = node_cols[i];
            col += core * vec3(1.3, 1.6, 2.0) + corona * nc + ring * mix(nc, vec3(1.0), 0.45);
        }
    }

    // 2. Draw dynamic distance-based connecting mesh lines
    const float D_MAX = 0.35;
    const float D_MAX_SQ = D_MAX * D_MAX;

    for (int i = 0; i < NUM_NODES; i++) {
        for (int j = i + 1; j < NUM_NODES; j++) {
            vec2 pi = nodes[i];
            vec2 pj = nodes[j];
            vec2 e = pj - pi;
            float dist_sq = dot(e, e);

            if (dist_sq < D_MAX_SQ) {
                float dist_ij = sqrt(dist_sq);
                // Connection weight drops smoothly with distance
                float link_w = 1.0 - smoothstep(0.10, D_MAX, dist_ij);
                link_w = link_w * link_w;

                // Point-to-segment distance
                vec2 w = p - pi;
                float h_proj = clamp(dot(w, e) / dist_sq, 0.0, 1.0);
                float d_line = length(w - e * h_proj);

                if (d_line < 0.035) {
                    // Anti-aliased laser filament & volumetric aura
                    float filament = (1.0 - smoothstep(0.001, 0.005, d_line)) * link_w;
                    float glow = exp(-d_line * 110.0) * 0.45 * link_w;

                    // High-speed data pulse packet along connection (confined to line corridor!)
                    float p_seed = fract(node_seeds[i] * 19.3 + node_seeds[j] * 7.7);
                    float p_pos = fract(t * (0.95 + 0.65 * p_seed) + p_seed);
                    float pulse = exp(-abs(h_proj - p_pos) * 35.0) * exp(-d_line * 140.0) * link_w * 2.0;

                    vec3 line_col = mix(node_cols[i], node_cols[j], h_proj);
                    col += line_col * (filament * 1.8 + glow * 0.65) + vec3(1.4, 1.8, 2.0) * pulse;
                }
            }
        }
    }

    // 3. Background deeper sub-constellation nodes (Parallax depth)
    for (int b = 0; b < 10; b++) {
        float fb = float(b);
        float bs = hash11(fb * 29.17 + 13.51);
        vec2 b_anchor = vec2(
            (mod(fb, 5.0) - 2.0) * 0.28,
            (floor(fb / 5.0) - 0.5) * 0.35
        ) * aspect_scale;
        vec2 b_wander = vec2(
            sin(t * 0.25 + bs * TWO_PI) * 0.12,
            cos(t * 0.28 + bs * 3.14) * 0.10
        );
        float bd = length(p - (b_anchor + b_wander));
        if (bd < 0.07) {
            float bg_core = exp(-bd * 65.0) * 0.55;
            col += vec3(0.4, 0.2, 0.85) * bg_core;
        }
    }

    return col;
}

// =========================================================================
// 5. PARALLAX TINY SPARKLES SYSTEM
// (Starts moving upward from the bottom, then turns to the side and
//  continuously curves/changes directions across parallax depth tiers)
// =========================================================================
vec3 render_sparkles_parallax(vec2 uv, float t, float scene_time) {
    vec3 sparkle_col = vec3(0.0);

    // Elapsed scene time for trajectory orientation
    float st = max(0.0, scene_time);

    // Integrated trajectory offset:
    // At st = 0: d(offset)/dt is purely vertical (moving upward from bottom).
    // As st progresses: horizontal velocity kicks in, turning to the side,
    // and oscillating smoothly to keep changing direction continuously.
    float side_turn = 1.0 - exp(-st * 0.28);
    float disp_x = side_turn * (st * 0.55 + sin(st * 0.5) * 1.1) + sin(t * 0.45) * 0.4;
    float disp_y = st * 0.95 + cos(st * 0.35) * 0.5;
    vec2 flow_disp = vec2(disp_x, disp_y);

    // 3 Parallax tiers: Foreground, Midground, Deep Background
    for (int tier = 0; tier < 3; tier++) {
        float f_tier = float(tier);
        float scale = 14.0 + f_tier * 10.0;
        float speed = 1.0 / (1.0 + f_tier * 0.6);
        float weight = 1.0 / (1.0 + f_tier * 0.5);

        vec2 sp_uv = uv * scale - flow_disp * speed * 1.8;
        vec2 sp_id = floor(sp_uv);
        vec2 sp_f = fract(sp_uv);

        float h1 = hash21(sp_id + vec2(2.13, 8.45) + f_tier * 17.0);
        float h2 = hash21(sp_id + vec2(9.87, 4.31) + f_tier * 31.0);

        // Position within cell
        vec2 sp_pos = 0.15 + 0.70 * vec2(h1, h2);
        float d_sp = length(sp_f - sp_pos);

        // Twinkle shimmer
        float twinkle = 0.35 + 0.65 * sin(t * (5.5 + 4.5 * h1) + h2 * TWO_PI);

        // Crisp microscopic core & airy photonic bloom
        float pt_core = exp(-d_sp * 120.0) * 2.2;
        float pt_halo = exp(-d_sp * 18.0) * 0.35;

        // Anamorphic tiny starburst diffraction spikes
        float spike_x = exp(-abs(sp_f.x - sp_pos.x) * 55.0) * exp(-abs(sp_f.y - sp_pos.y) * 10.0);
        float spike_y = exp(-abs(sp_f.y - sp_pos.y) * 55.0) * exp(-abs(sp_f.x - sp_pos.x) * 10.0);
        float spikes = (spike_x + spike_y) * 0.30;

        // Dynamic spectral color
        vec3 col_particle = mix(vec3(0.2, 0.9, 1.0), vec3(1.0, 0.82, 0.4), h2);
        if (h1 > 0.78) col_particle = vec3(1.0, 0.35, 0.85); // Electric magenta spark
        else if (h1 < 0.22) col_particle = vec3(0.3, 1.0, 0.65); // Cyber emerald spark

        sparkle_col += (pt_core * vec3(1.5, 1.6, 1.8) + (pt_halo + spikes) * col_particle) * twinkle * weight;
    }

    return sparkle_col;
}

// =========================================================================
// 6. MAIN COMPOSITING PIPELINE WITH DYNAMIC ACT CHOREOGRAPHY
// =========================================================================
void main() {
    vec2 raw_uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Barrel distortion / Curved cyber lens
    float r2 = dot(raw_uv, raw_uv);
    vec2 screen_uv = raw_uv * (1.0 + 0.06 * r2);

    // =========================================================================
    // MACRO STORY CYCLE (72.0s PERIOD)
    // Act 1: The Gateway to Cyberspace (0.0s - 20.0s)
    // Crossing 1: Defragmentation Scan Beam (18.5s - 22.5s)
    // Act 2: Helical Vortex Wormhole (21.0s - 43.0s)
    // Wormhole Fade Out: Before Act 3 renders, the wormhole gracefully fades out (39.0s - 43.0s)
    // Act 3: Dynamic Plexus Constellation Mesh & Direction-Changing Sparkles (42.0s - 65.0s)
    // Return of the Wormhole: As Act 3 finishes, wormhole returns in full kinetic rush (63.0s - 72.0s)
    // =========================================================================
    const float CYCLE_DURATION = 72.0;
    float macro_t = mod(u_time, CYCLE_DURATION);

    // Dynamic camera rolls evolving per act
    float roll1 = 0.05 * sin(u_time * 0.35);
    float roll2 = 0.28 * sin(u_time * 0.40);
    float roll3 = 0.10 * sin(u_time * 0.22);

    // Scene weights with smoothstep easing (partition of unity)
    // Act 1: Gateway Rain (0s - 22s)
    float w_act1 = (1.0 - smoothstep(18.0, 22.0, macro_t)) + smoothstep(70.0, 72.0, macro_t);
    w_act1 = clamp(w_act1, 0.0, 1.0);

    // Act 2: Wormhole Vortex (20s - 43s, and returning 63s - 72s)
    // Note: Wormhole fades out completely before Act 3 reaches full intensity!
    float w_wormhole_dive = smoothstep(19.0, 23.0, macro_t) * (1.0 - smoothstep(39.0, 43.0, macro_t));
    float w_wormhole_return = smoothstep(62.0, 67.0, macro_t) * (1.0 - smoothstep(70.0, 72.0, macro_t));
    float w_act2 = clamp(w_wormhole_dive + w_wormhole_return, 0.0, 1.0);

    // Act 3: Dynamic Plexus Constellation Mesh (41.0s - 66.0s)
    float w_act3 = smoothstep(41.0, 45.0, macro_t) * (1.0 - smoothstep(62.0, 66.0, macro_t));
    w_act3 = clamp(w_act3, 0.0, 1.0);

    float sum_w = w_act1 + w_act2 + w_act3 + 1e-4;
    w_act1 /= sum_w;
    w_act2 /= sum_w;
    w_act3 /= sum_w;

    float cam_roll = roll1 * w_act1 + roll2 * w_act2 + roll3 * w_act3;
    float ca = cos(cam_roll), sa = sin(cam_roll);
    vec2 p_oriented = mat2(ca, -sa, sa, ca) * screen_uv;

    // Cyber glitch horizontal slicing
    float glitch_trigger = step(0.97, sin(u_time * 3.1) * sin(u_time * 7.3));
    if (glitch_trigger > 0.5) {
        float slice = step(0.5, fract(gl_FragCoord.y * 0.02 + u_time * 8.0));
        if (slice > 0.5) {
            p_oriented.x += 0.03 * sin(gl_FragCoord.y * 0.2 + u_time * 15.0);
        }
    }

    // =========================================================================
    // STAR TREK HYPERSPACE WARP DYNAMICS (EMERGES AS SCENE CHANGES)
    // - Narrower FOV (telephoto speedup zoom towards vanishing point)
    // - Forward velocity acceleration down the hyperspace conduit
    // - Radial motion parallax starfield streaks
    // =========================================================================
    // Transition 1: Gateway -> Wormhole (17.5s - 22.5s)
    float warp1 = smoothstep(17.5, 20.0, macro_t) * (1.0 - smoothstep(20.5, 23.0, macro_t));
    // Transition 2: Wormhole -> Plexus (38.5s - 41.0s)
    float warp2 = smoothstep(38.5, 41.0, macro_t) * (1.0 - smoothstep(41.5, 44.0, macro_t));
    // Transition 3: Plexus -> Wormhole Return (61.5s - 64.0s)
    float warp3 = smoothstep(61.5, 64.0, macro_t) * (1.0 - smoothstep(64.5, 67.0, macro_t));
    // Transition 4: Wormhole -> Gateway Loop (69.5s - 72.0s & 0.0s - 2.0s)
    float warp4 = smoothstep(69.5, 71.5, macro_t) + (1.0 - smoothstep(0.0, 2.0, macro_t));
    float warp_factor = clamp(warp1 + warp2 + warp3 + warp4, 0.0, 1.0);

    // Narrower FOV during hyperspace speedup (smaller FOV = coordinate compression into center)
    float fov_scale = 1.0 + 0.38 * warp_factor;
    vec2 p = p_oriented / fov_scale;

    // Hyperspace speedup time parameter
    float hyper_time = u_time + warp_factor * 2.2;

    // =========================================================================
    // SCENE CHANGING CROSSING WARPS & CAUSTIC BEAMS (MODERATED FLARES)
    // =========================================================================
    vec2 p_warped = p;
    vec3 crossing_fx = vec3(0.0);

    // Crossing 1 (18.5s - 22.5s): Horizontal Defragmentation Scan Beam (Toned down)
    if (macro_t >= 18.5 && macro_t < 22.5) {
        float ct = (macro_t - 18.5) / 4.0;
        float sweep = p.y - (ct * 2.4 - 1.2);
        p_warped.x += sin(sweep * 25.0) * exp(-abs(sweep) * 8.0) * 0.025;
        
        float beam = exp(-abs(sweep) * 14.0) * 0.55;
        crossing_fx += vec3(0.2, 0.75, 1.0) * beam;
    }
    // Crossing 2 (39.5s - 43.5s): Singularity Void Dissolve (Toned down)
    else if (macro_t >= 39.5 && macro_t < 43.5) {
        float ct = (macro_t - 39.5) / 4.0;
        float r = length(p);
        float wave = abs(r - ct * 1.35);
        p_warped += normalize(p + 1e-4) * sin((r - ct * 1.35) * 28.0) * exp(-wave * 10.0) * 0.025 * (1.0 - ct);
        
        float pulse_beam = exp(-wave * 8.0) * (1.0 - ct * 0.3) * 0.45;
        crossing_fx += vec3(0.15, 0.65, 0.95) * pulse_beam;
    }
    // Crossing 3 (62.5s - 66.5s): Return of Wormhole Gravitational Warp (Toned down)
    else if (macro_t >= 62.5 && macro_t < 66.5) {
        float ct = (macro_t - 62.5) / 4.0;
        float r = length(p);
        float wave = abs(r - (1.0 - ct) * 1.2);
        p_warped += normalize(p + 1e-4) * sin((r - (1.0 - ct) * 1.2) * 24.0) * exp(-wave * 8.0) * 0.02 * ct;
        crossing_fx += vec3(0.25, 0.55, 1.0) * exp(-wave * 7.0) * 0.45;
    }

    // Evaluate each scene with accelerated hyperspace flow
    vec3 color_scene1 = (w_act1 > 0.001) ? render_scene1_gateway(p_warped, hyper_time) : vec3(0.0);
    vec3 color_scene2 = (w_act2 > 0.001) ? render_scene2_vortex(p_warped, hyper_time) : vec3(0.0);
    vec3 color_scene3 = (w_act3 > 0.001) ? render_scene3_plexus(p_warped, hyper_time) : vec3(0.0);

    // Gently composite the scenes
    vec3 final_color = color_scene1 * w_act1 
                     + color_scene2 * w_act2 
                     + color_scene3 * w_act3;

    // Apply crossing optical beams (moderated flare)
    final_color += crossing_fx;

    // Star Trek Parallaxing Warp Starfield (emerges as scene changes)
    vec3 startrek_stars = render_startrek_starfield(p_warped, u_time, warp_factor);
    final_color += startrek_stars;

    // Parallax tiny sparkles layer:
    // Starts moving upward from bottom, turns to side, and keeps continuously changing direction
    float plexus_scene_t = macro_t - 41.0;
    vec3 sparkles = render_sparkles_parallax(p, u_time, plexus_scene_t);
    final_color += sparkles * (w_act3 * 1.45 + 0.18);

    // =========================================================================
    // VFX POST-PROCESSING PIPELINE
    // =========================================================================
    // 1. Radial Chromatic Aberration
    float r_lens = length(p);
    float ca_shift = r_lens * 0.015;
    final_color.r *= (1.0 + ca_shift * 2.2);
    final_color.b *= (1.0 - ca_shift * 1.5);

    // 2. Phosphor CRT Scanlines
    float scanline = 0.92 + 0.08 * sin(gl_FragCoord.y * 2.6);
    final_color *= scanline;

    // 3. Cyber Matrix Glitch Color Inversion
    if (glitch_trigger > 0.5) {
        float slice = step(0.48, fract(gl_FragCoord.y * 0.015 + u_time * 5.0));
        if (slice > 0.5) {
            final_color = final_color.gbr * 1.25;
        }
    }

    // 4. Subtle Film Shimmer / Cyber Dust Grain
    float grain = (hash11(gl_FragCoord.x * 12.9898 + gl_FragCoord.y * 78.233 + fract(u_time) * 100.0) - 0.5) * 0.022;
    final_color += grain;

    // 5. Radial Vignette
    float vignette = clamp(1.0 - 0.38 * pow(length(raw_uv), 2.2), 0.0, 1.0);
    final_color *= vignette;

    // 6. ACES Filmic Tone Mapping for deep pitch-black shadows & electric highlights
    vec3 mapped = aces_tonemap(final_color * 1.25);

    fragColor = vec4(mapped, 1.0);
}
"#;
