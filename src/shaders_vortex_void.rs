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
// 1. MATHEMATICAL UTILITIES, HASHES & COLOR PALETTES
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

mat2 rot2D(float a) {
    float c = cos(a), s = sin(a);
    return mat2(c, -s, s, c);
}

// Inigo Quilez Cosine Color Palette
vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
    return clamp(a + b * cos(TWO_PI * (c * t + d)), 0.0, 1.0);
}

// Obsidian Cyber Palette (Deep violet, electric cyan, laser magenta, gold)
vec3 cyber_pal(float t) {
    return palette(t, vec3(0.5, 0.45, 0.55), vec3(0.5, 0.5, 0.5), vec3(1.0, 1.0, 1.0), vec3(0.65, 0.85, 0.15));
}

// Distance to 2D line segment
float sdSegment(vec2 p, vec2 a, vec2 b) {
    vec2 pa = p - a, ba = b - a;
    float h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

// ACES Filmic Tone Mapping Curve
vec3 aces_tonemap(vec3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

// =========================================================================
// SCENE 1: THE EVENT HORIZON (Relativistic Gravitational Singularity Void)
// Relativistic Kerr black hole with frame-dragging, Doppler accretion disk & polar plasma jets
// =========================================================================
vec3 scene_singularity(vec2 p, float t) {
    float r = length(p);
    float theta = atan(p.y, p.x);

    // Lense-Thirring frame dragging: azimuthal coordinate swirl near singularity
    float drag = 0.45 / (r * r + 0.08);
    float theta_drag = theta + drag * (t * 0.6);

    // Relativistic gravitational lensing deflection
    float rs = 0.16; // Schwarzschild event horizon radius
    vec2 p_lens = p * (1.0 - rs / (pow(r, 1.5) + 0.035));
    float r_lens = length(p_lens);

    vec3 col = vec3(0.002, 0.003, 0.008); // Deep obsidian background

    // Distant background starfield warped by gravity
    vec2 star_uv = p_lens * 35.0;
    vec2 star_id = floor(star_uv);
    vec2 star_f = fract(star_uv) - 0.5;
    float star_h = hash21(star_id);
    if (star_h > 0.94) {
        float star_b = exp(-35.0 * dot(star_f, star_f)) * (0.4 + 0.6 * sin(t * 3.0 + star_h * 10.0));
        col += vec3(0.6, 0.8, 1.0) * star_b * smoothstep(rs * 1.2, rs * 2.0, r_lens);
    }

    // Accretion disk: inclined 3D plane projection
    // Disk plane inclined at angle alpha (cos = 0.38)
    vec2 p_disk = p_lens;
    p_disk.y /= 0.38;
    float r_disk = length(p_disk);
    float ang_disk = atan(p_disk.y, p_disk.x) - t * 1.8;

    if (r_disk > rs * 1.15 && r_disk < 1.35) {
        // Spiral turbulence arms in the plasma
        float spiral = sin(r_disk * 22.0 - ang_disk * 3.0 + sin(r_disk * 10.0));
        float disk_density = smoothstep(rs * 1.15, rs * 1.8, r_disk) * (1.0 - smoothstep(0.9, 1.35, r_disk));
        disk_density *= (0.65 + 0.35 * spiral);

        // Relativistic Doppler beaming: approaching side (left: p_disk.x < 0) is violently boosted and blue-shifted
        float doppler = clamp(1.0 - 0.75 * (p_disk.x / (r_disk + 0.05)), 0.25, 2.8);

        // Multi-tier thermal spectrum: inner edge is ultra-hot Cherenkov blue-cyan, outer edge is amber-violet
        float temp = clamp((1.35 - r_disk) / (1.35 - rs * 1.15), 0.0, 1.0);
        vec3 disk_color = mix(
            vec3(0.85, 0.25, 0.95), // Outer violet
            vec3(0.15, 0.85, 1.00), // Mid electric cyan
            temp
        );
        disk_color = mix(disk_color, vec3(1.2, 1.4, 1.8), pow(temp, 3.5)); // Inner white-hot core

        float disk_glow = disk_density * doppler * (0.4 / (abs(p_lens.y) + 0.04));
        col += disk_color * disk_glow * 0.45;
    }

    // Razor-sharp Relativistic Photon Sphere at 1.5 * rs
    float photon_r = rs * 1.55;
    float photon_ring = exp(-90.0 * abs(r_lens - photon_r));
    col += vec3(0.5, 0.9, 1.5) * photon_ring * 2.2;

    // Relativistic Polar Plasma Jets: collimated magnetic helical vortex
    float jet_y = abs(p.y);
    float jet_twist = sin(p.y * 28.0 - t * 9.0) * 0.025 * smoothstep(0.1, 0.8, jet_y);
    float d_jet = abs(p.x - jet_twist);
    float jet_intensity = exp(-55.0 * d_jet) * smoothstep(0.12, 0.35, jet_y) * exp(-1.2 * jet_y);
    vec3 jet_col = mix(vec3(0.1, 0.7, 1.0), vec3(0.8, 0.2, 1.0), 0.5 + 0.5 * sin(p.y * 12.0 + t * 4.0));
    col += jet_col * jet_intensity * 2.8;

    // Magnetic field contour ripples propagating outward
    float ripple = sin(r * 32.0 - t * 5.0);
    col += vec3(0.15, 0.35, 0.85) * exp(-4.0 * r) * pow(max(0.0, ripple), 4.0) * 0.4;

    // Absolute Event Horizon / Schwarzschild Shadow: pitch obsidian void inside rs
    float shadow = smoothstep(rs * 0.92, rs * 1.04, r_lens);
    col *= shadow;

    return col;
}

// =========================================================================
// SCENE 2: THE CYBER TESSERACT (4D Rotational Quantum Hyper-Lattice)
// 16 vertices, 32 edges stereographically projected from 4D with traveling photon pulses
// =========================================================================
vec3 scene_tesseract(vec2 uv, float t) {
    vec3 col = vec3(0.003, 0.002, 0.006);

    // Dynamic isometric 4D rotation angles
    float th_xw = t * 0.55;
    float th_yz = t * 0.75;
    float c_xw = cos(th_xw), s_xw = sin(th_xw);
    float c_yz = cos(th_yz), s_yz = sin(th_yz);

    // Camera 3D orbital tilt
    mat2 rot_cam_x = rot2D(0.35 + 0.12 * sin(t * 0.4));
    mat2 rot_cam_y = rot2D(t * 0.30);

    // Precompute 16 projected 2D vertices
    vec2 p2d[16];
    vec3 p3d[16];

    for (int i = 0; i < 16; i++) {
        // Vertex coordinates in 4D: (±1, ±1, ±1, ±1)
        float vx = ((i & 1) != 0) ? 1.0 : -1.0;
        float vy = ((i & 2) != 0) ? 1.0 : -1.0;
        float vz = ((i & 4) != 0) ? 1.0 : -1.0;
        float vw = ((i & 8) != 0) ? 1.0 : -1.0;

        // Scale in 4D
        vec4 v4 = vec4(vx, vy, vz, vw) * 0.85;

        // 4D Rotation in XW and YZ planes
        float rx = v4.x * c_xw - v4.w * s_xw;
        float rw = v4.x * s_xw + v4.w * c_xw;
        float ry = v4.y * c_yz - v4.z * s_yz;
        float rz = v4.y * s_yz + v4.z * c_yz;

        // 4D -> 3D stereographic perspective projection
        float d4 = 2.65;
        float inv_w = 1.0 / (d4 - rw);
        vec3 v3 = vec3(rx, ry, rz) * inv_w;

        // 3D camera rotation
        v3.yz = rot_cam_x * v3.yz;
        v3.xz = rot_cam_y * v3.xz;

        // 3D -> 2D screen projection
        float z_cam = v3.z + 2.2;
        vec2 v2 = (v3.xy / z_cam) * 1.55;

        p2d[i] = v2;
        p3d[i] = v3;
    }

    // Render 32 edges of the 4D hypercube
    // In a hypercube, an edge exists between any two vertices differing by exactly 1 bit
    float pulse_phase = t * 2.2;

    for (int i = 0; i < 16; i++) {
        for (int bit = 0; bit < 4; bit++) {
            int j = i ^ (1 << bit);
            if (j > i) {
                vec2 va = p2d[i];
                vec2 vb = p2d[j];
                float d_edge = sdSegment(uv, va, vb);

                // Laser line core and aura
                float edge_w = exp(-110.0 * d_edge) * 1.8 + exp(-22.0 * d_edge) * 0.35;
                
                // Color variation by 4D edge orientation
                float edge_hue = float(bit) * 0.25 + 0.1;
                vec3 edge_col = cyber_pal(edge_hue);

                // Photon data pulse traveling along edge
                float edge_id_hash = float(i * 4 + bit) * 0.31;
                float traveling_h = fract(pulse_phase + edge_id_hash);
                vec2 pulse_pos = mix(va, vb, traveling_h);
                float d_pulse = length(uv - pulse_pos);
                float pulse_glow = exp(-75.0 * d_pulse) * 2.5;

                col += (edge_col * edge_w + vec3(1.0, 1.2, 1.5) * pulse_glow) * 0.45;
            }
        }
    }

    // Render 16 glowing quantum nodes at the vertices
    for (int i = 0; i < 16; i++) {
        vec2 v2 = p2d[i];
        float d_node = length(uv - v2);
        float node_core = exp(-280.0 * d_node) * 2.8;
        float node_aura = exp(-35.0 * d_node) * 0.85;
        float depth_fade = clamp((p3d[i].z + 1.2) / 2.4, 0.4, 1.3);

        vec3 node_col = mix(vec3(0.2, 0.9, 1.0), vec3(1.0, 0.3, 0.8), float(i) / 15.0);
        col += (node_col * node_aura + vec3(1.2) * node_core) * depth_fade * 0.6;
    }

    // Volumetric quantum interference grid floating in the background
    vec2 grid_uv = uv * 6.0;
    vec2 grid_lines = abs(fract(grid_uv - 0.5) - 0.5);
    float d_grid = min(grid_lines.x, grid_lines.y);
    float grid_glow = exp(-40.0 * d_grid) * exp(-1.8 * length(uv));
    col += vec3(0.12, 0.25, 0.65) * grid_glow * 0.4;

    return col;
}

// =========================================================================
// SCENE 3: THE CYBER-HEX CONDUIT (Relativistic Hyperspace Tunnel)
// 6-sided faceted metallic obsidian conduit with relativistic particle streaks & iris quantum gates
// =========================================================================
vec3 scene_hex_conduit(vec2 uv, float t) {
    // Dynamic banking and roll
    float roll = sin(t * 0.45) * 0.35;
    vec2 p = rot2D(roll) * uv;
    p += vec2(sin(t * 0.8) * 0.08, cos(t * 0.6) * 0.06);

    float r = length(p);
    float theta = atan(p.y, p.x) + t * 0.25;

    // Hexagonal geometry projection: 6-fold radial symmetry
    float hex_angle = mod(theta + PI / 6.0, PI / 3.0) - PI / 6.0;
    float k_hex = cos(hex_angle);
    float r_hex = r * k_hex;

    // Relativistic forward velocity depth along hex conduit
    float z_speed = 6.5;
    float z = 1.35 / (r_hex + 0.02) + t * z_speed;

    vec3 col = vec3(0.002, 0.004, 0.008);

    // 6 Longitudinal Corner Ribs (glowing energy conduits at hex vertices)
    float corner_dist = abs(sin(theta * 3.0));
    float rib_glow = exp(-28.0 * corner_dist) * smoothstep(0.08, 0.35, r);
    vec3 rib_col = mix(vec3(0.0, 1.0, 0.65), vec3(0.1, 0.85, 1.0), 0.5 + 0.5 * sin(z * 0.3));
    col += rib_col * rib_glow * 1.6;

    // Faceted metallic panel plates along the 6 hex walls
    float panel_z = fract(z * 0.25);
    float seam_z = abs(panel_z - 0.5);
    float panel_seam = exp(-45.0 * seam_z);
    col += vec3(0.3, 0.6, 1.0) * panel_seam * 0.6 * smoothstep(0.05, 0.3, r);

    // Laser telemetry circuit traces on the hex facets
    float circuit_x = fract(theta * (6.0 / PI) * 2.0);
    float circuit_z = fract(z * 0.8);
    float d_circuit = min(abs(circuit_x - 0.5), abs(circuit_z - 0.5));
    float circuit_glow = exp(-35.0 * d_circuit) * (0.3 + 0.7 * sin(z * 2.0 - t * 8.0));
    col += vec3(0.1, 0.7, 1.0) * circuit_glow * 0.35 * smoothstep(0.08, 0.4, r);

    // Concentric Hexagonal Iris Quantum Gates rushing towards viewer
    float gate_spacing = 7.0;
    float gate_z = mod(z, gate_spacing);
    float gate_proximity = exp(-1.8 * abs(gate_z - 3.5));
    float hex_rim = abs(r_hex - 0.38 - 0.05 * sin(t * 3.0));
    float gate_laser = exp(-60.0 * hex_rim) * gate_proximity;
    vec3 gate_col = mix(vec3(1.0, 0.2, 0.6), vec3(1.0, 0.8, 0.1), sin(z * 0.1) * 0.5 + 0.5);
    col += gate_col * gate_laser * 2.5;

    // Relativistic hyperspace particle streaks
    float num_streaks = 24.0;
    float streak_ang = floor(theta * (num_streaks / TWO_PI));
    float streak_h = hash11(streak_ang * 17.13);
    float streak_z = fract(z * 0.6 + streak_h * 5.0);
    float streak_trail = pow(streak_z, 6.0) * exp(-40.0 * abs(fract(theta * (num_streaks / TWO_PI)) - 0.5));
    col += vec3(0.8, 1.1, 1.5) * streak_trail * 1.5 * smoothstep(0.06, 0.25, r);

    // Deep central vortex singularity
    float core_shadow = smoothstep(0.04, 0.15, r);
    col *= core_shadow;
    col += vec3(0.05, 0.4, 0.8) * exp(-30.0 * r) * 1.4;

    return col;
}

// =========================================================================
// SCENE 4: THE QUANTUM GYROSCOPE (Celestial Mechanica Void)
// 3 concentric metallic toroidal gimbals spinning on 3 orthogonal axes with engraved tick marks & magnetic plasma core
// =========================================================================
vec3 scene_gyroscope(vec2 uv, float t) {
    vec3 col = vec3(0.003, 0.002, 0.006);

    // 3D Camera Setup
    vec3 ro = vec3(0.0, 0.0, 2.6);
    vec3 rd = normalize(vec3(uv, -1.5));

    // Dynamic camera slow pan
    mat2 cam_rot = rot2D(sin(t * 0.35) * 0.18);
    ro.xz = cam_rot * ro.xz;
    rd.xz = cam_rot * rd.xz;

    // Multi-Axis Gimbal Rotations
    mat2 rot_x = rot2D(t * 0.85);
    mat2 rot_y = rot2D(t * 1.15);
    mat2 rot_z = rot2D(t * 0.65);

    // Torus SDF evaluations across 3 rings
    // Ring 1 (Outer): Major R=0.92, Minor r=0.026
    // Ring 2 (Middle): Major R=0.68, Minor r=0.022
    // Ring 3 (Inner): Major R=0.46, Minor r=0.018
    const float R1 = 0.92, r1 = 0.026;
    const float R2 = 0.68, r2 = 0.022;
    const float R3 = 0.46, r3 = 0.018;

    // Raymarch the 3 gimbal rings
    float d_accum = 0.0;
    vec3 p_cur = ro;
    float hit_ring = 0.0; // 1, 2, or 3
    vec3 hit_pos = vec3(0.0);

    for (int step = 0; step < 48; step++) {
        p_cur = ro + rd * d_accum;

        // Ring 1 space: rotate around X axis
        vec3 p1 = p_cur;
        p1.yz = rot_x * p1.yz;
        float d1 = length(vec2(length(p1.xz) - R1, p1.y)) - r1;

        // Ring 2 space: rotate around Y axis
        vec3 p2 = p_cur;
        p2.xz = rot_y * p2.xz;
        float d2 = length(vec2(length(p2.xy) - R2, p2.z)) - r2;

        // Ring 3 space: rotate around Z axis
        vec3 p3 = p_cur;
        p3.xy = rot_z * p3.xy;
        float d3 = length(vec2(length(p3.yz) - R3, p3.x)) - r3;

        float d_min = min(d1, min(d2, d3));

        if (d_min < 0.003) {
            if (d_min == d1) hit_ring = 1.0;
            else if (d_min == d2) hit_ring = 2.0;
            else hit_ring = 3.0;
            hit_pos = p_cur;
            break;
        }

        d_accum += d_min * 0.85;
        if (d_accum > 4.5) break;
    }

    if (hit_ring > 0.5) {
        // Metallic obsidian shading with rim glint
        vec3 n_vec = normalize(hit_pos);
        float rim = 1.0 - abs(dot(rd, n_vec));
        vec3 ring_base = vec3(0.06, 0.08, 0.12);

        // Engraved telemetry tick marks along ring circumference
        vec3 p_loc = hit_pos;
        float ang_ring = 0.0;
        if (hit_ring == 1.0) {
            p_loc.yz = rot_x * p_loc.yz;
            ang_ring = atan(p_loc.z, p_loc.x);
        } else if (hit_ring == 2.0) {
            p_loc.xz = rot_y * p_loc.xz;
            ang_ring = atan(p_loc.y, p_loc.x);
        } else {
            p_loc.xy = rot_z * p_loc.xy;
            ang_ring = atan(p_loc.z, p_loc.y);
        }

        float ticks = exp(-35.0 * abs(sin(ang_ring * 32.0)));
        vec3 tick_col = cyber_pal(hit_ring * 0.3 + 0.1);

        col = ring_base + tick_col * ticks * 1.5 + vec3(0.5, 0.8, 1.2) * pow(rim, 3.5);
    }

    // Central Pulsating Quantum Core (Magnetic Plasma Sphere)
    float r_uv = length(uv);
    float core_r = 0.17;
    float core_surface = exp(-50.0 * abs(r_uv - core_r));
    float core_glow = exp(-12.0 * r_uv);

    // Volumetric celestial light shafts emanating from the core through the rings
    float godrays = pow(max(0.0, sin(atan(uv.y, uv.x) * 12.0 + t * 0.6)), 3.0) * exp(-2.5 * r_uv) * 0.45;
    col += vec3(0.4, 0.6, 1.0) * godrays;

    // Distant background celestial coordinate rings
    float astro_rings = exp(-40.0 * abs(fract(r_uv * 3.5) - 0.5)) * exp(-1.2 * r_uv);
    col += vec3(0.10, 0.22, 0.55) * astro_rings * 0.35;

    // Magnetic dipole flux loops arcing between core poles
    float dipole_field = abs(uv.x * uv.x - uv.y * 0.35);
    float flux_lines = exp(-40.0 * dipole_field) * (0.5 + 0.5 * sin(t * 6.0));

    vec3 core_col = mix(vec3(0.1, 0.9, 1.0), vec3(0.9, 0.2, 0.8), 0.5 + 0.5 * sin(t * 2.5));
    col += core_col * (core_surface * 2.0 + core_glow * 1.2 + flux_lines * 0.6);

    // Orbiting Quantum Satellites (energy orbs in tilted elliptical orbits)
    for (int k = 0; k < 3; k++) {
        float sat_t = t * 1.8 + float(k) * 2.094;
        vec2 sat_pos = vec2(cos(sat_t) * 0.75, sin(sat_t) * 0.35);
        sat_pos = rot2D(float(k) * 0.8) * sat_pos;
        float d_sat = length(uv - sat_pos);
        float sat_glow = exp(-70.0 * d_sat) * 2.2 + exp(-15.0 * d_sat) * 0.5;
        col += vec3(0.2, 1.0, 0.7) * sat_glow * 0.5;
    }

    return col;
}

// =========================================================================
// SCENE 5: THE HYPERBOLIC KALEIDOSCOPE VOID & IMPLOSION CLIMAX
// Non-Euclidean domain folding cathedral, culminating in violent gravitational collapse & warp rebirth
// =========================================================================
vec3 scene_kaleidoscope(vec2 uv, float t, float collapse_factor) {
    // Gravitational implosion coordinate compression during transition climax
    vec2 p = uv * (1.0 + collapse_factor * 10.0);

    // Hypnotic kaleidoscopic breathing rotation
    p = rot2D(sin(t * 0.4) * 0.35) * p;

    vec3 col = vec3(0.002, 0.002, 0.005);

    // Iterative non-Euclidean kaleidoscopic domain folding
    vec2 z = p;
    float scale = 1.0;
    float min_d = 100.0;

    for (int i = 0; i < 5; i++) {
        // Octahedral / tetrahedral folding reflections
        z = abs(z);
        if (z.x < z.y) z = z.yx;

        // Dynamic morphing fold angles
        float fold_ang = 0.52 + 0.08 * sin(t * 0.6 + float(i) * 1.2);
        z -= vec2(fold_ang, fold_ang * 0.65);
        z = rot2D(0.42 + 0.1 * sin(t * 0.8)) * z;

        z *= 1.45;
        scale *= 1.45;

        // Measure distance to fractal edges
        float d_edge = min(abs(z.x), abs(z.y)) / scale;
        min_d = min(min_d, d_edge);
    }

    // High-contrast obsidian crystal cathedral with electric edge luminescence
    float edge_glow = exp(-75.0 * min_d);
    float aura_glow = exp(-18.0 * min_d);

    // Obsidian face mask: carve deep black voids inside the crystal facets
    float obsidian_depth = smoothstep(0.015, 0.08, min_d);
    float facet_lighting = 1.0 - 0.78 * obsidian_depth;

    vec3 edge_col = cyber_pal(length(p) * 0.45 + t * 0.2);
    col += edge_col * (edge_glow * 2.5 + aura_glow * 0.6) * facet_lighting;

    // Central energy vortex core
    float r = length(p);
    col += vec3(0.8, 0.2, 1.0) * exp(-22.0 * r) * 1.8;

    // Implosion shockwave flash: focused relativistic warp shockwave ring
    if (collapse_factor > 0.01) {
        float wave_radius = collapse_factor * 1.2;
        float implosion_ring = exp(-50.0 * abs(r - wave_radius));
        col += vec3(0.8, 1.2, 2.0) * implosion_ring * 3.0;
        col += vec3(0.2, 0.6, 1.2) * exp(-15.0 * r) * collapse_factor * 2.0;
    }

    return col;
}

// =========================================================================
// MAIN PIPELINE & SEAMLESS TRANSITIONS ENGINE
// =========================================================================
void main() {
    // Coordinate normalization centered at origin
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Macro story cycle: 80.0 seconds total period
    // 5 Distinct brand-new scenes (16.0s each) with 3.6s smooth C1 transitions:
    // Scene 1: Event Horizon Singularity (0.0s - 16.0s)
    // Scene 2: Cyber Tesseract Hyper-Lattice (16.0s - 32.0s)
    // Scene 3: Cyber-Hex Conduit (32.0s - 48.0s)
    // Scene 4: Quantum Gyroscope Mechanica (48.0s - 64.0s)
    // Scene 5: Kaleidoscope Void & Implosion (64.0s - 80.0s)
    const float CYCLE = 80.0;
    const float DURATION = 16.0;
    const float BLEND = 3.6;

    float tau = mod(u_time, CYCLE);

    // Calculate normalized weights for each scene (C1 smooth partition of unity)
    // Scene interval centers: 8, 24, 40, 56, 72
    float w1 = 0.0, w2 = 0.0, w3 = 0.0, w4 = 0.0, w5 = 0.0;
    float trans_pulse = 0.0;
    float collapse_factor = 0.0;

    // Boundary 1->2 at t=16: blend in [16 - BLEND/2, 16 + BLEND/2] = [14.2, 17.8]
    // Boundary 2->3 at t=32: [30.2, 33.8]
    // Boundary 3->4 at t=48: [46.2, 49.8]
    // Boundary 4->5 at t=64: [62.2, 65.8]
    // Boundary 5->1 at t=80 / 0: [78.2, 80.0] and [0.0, 1.8]

    if (tau < 14.2) {
        w1 = 1.0;
    } else if (tau < 17.8) {
        float f = smoothstep(14.2, 17.8, tau);
        w1 = 1.0 - f;
        w2 = f;
        trans_pulse = 4.0 * w1 * w2;
    } else if (tau < 30.2) {
        w2 = 1.0;
    } else if (tau < 33.8) {
        float f = smoothstep(30.2, 33.8, tau);
        w2 = 1.0 - f;
        w3 = f;
        trans_pulse = 4.0 * w2 * w3;
    } else if (tau < 46.2) {
        w3 = 1.0;
    } else if (tau < 49.8) {
        float f = smoothstep(46.2, 49.8, tau);
        w3 = 1.0 - f;
        w4 = f;
        trans_pulse = 4.0 * w3 * w4;
    } else if (tau < 62.2) {
        w4 = 1.0;
    } else if (tau < 65.8) {
        float f = smoothstep(62.2, 65.8, tau);
        w4 = 1.0 - f;
        w5 = f;
        trans_pulse = 4.0 * w4 * w5;
    } else if (tau < 76.0) {
        w5 = 1.0;
    } else if (tau < 78.2) {
        w5 = 1.0;
        collapse_factor = smoothstep(76.0, 78.2, tau);
    } else {
        // Implosion collapse transition 5 -> 1
        float f = smoothstep(78.2, 81.8, tau > 70.0 ? tau : tau + 80.0);
        w5 = 1.0 - f;
        w1 = f;
        trans_pulse = 4.0 * w5 * w1;
        collapse_factor = 1.0 - f;
    }

    // Handle wrap-around for Boundary 5->1 in [0.0, 1.8]
    if (tau < 1.8) {
        float f = smoothstep(78.2, 81.8, tau + 80.0);
        w5 = 1.0 - f;
        w1 = f;
        trans_pulse = 4.0 * w5 * w1;
        collapse_factor = 1.0 - f;
    }

    // Smooth Optical Transition Distortion:
    // Spatial refractive ripple radiating outward during crossfade windows
    vec2 uv_distorted = uv;
    if (trans_pulse > 0.001) {
        float r_uv = length(uv);
        float wave = sin(r_uv * 26.0 - tau * 12.0) * exp(-2.8 * r_uv) * 0.045 * trans_pulse;
        uv_distorted += (r_uv > 0.001 ? normalize(uv) : vec2(0.0)) * wave;
    }

    // Accumulate weighted scenes (only executing active scenes)
    vec3 scene_color = vec3(0.0);

    if (w1 > 0.001) {
        scene_color += w1 * scene_singularity(uv_distorted, u_time);
    }
    if (w2 > 0.001) {
        scene_color += w2 * scene_tesseract(uv_distorted, u_time);
    }
    if (w3 > 0.001) {
        scene_color += w3 * scene_hex_conduit(uv_distorted, u_time);
    }
    if (w4 > 0.001) {
        scene_color += w4 * scene_gyroscope(uv_distorted, u_time);
    }
    if (w5 > 0.001) {
        scene_color += w5 * scene_kaleidoscope(uv_distorted, u_time, collapse_factor);
    }

    // Transition Chromatic Dispersion: subtle RGB flare during transitions
    if (trans_pulse > 0.001) {
        scene_color.r *= (1.0 + 0.15 * trans_pulse);
        scene_color.b *= (1.0 - 0.10 * trans_pulse);
    }

    // Subtle Phosphor CRT Scanlines
    float scanline = 0.96 + 0.04 * sin(gl_FragCoord.y * 2.2);
    scene_color *= scanline;

    // Radial Vignette
    float vignette = clamp(1.0 - 0.42 * pow(length(uv), 2.2), 0.0, 1.0);
    scene_color *= vignette;

    // ACES Filmic Tone Mapping for deep pitch-black shadows & electric highlights
    vec3 mapped = aces_tonemap(scene_color * 1.22);

    fragColor = vec4(mapped, 1.0);
}
"#;
