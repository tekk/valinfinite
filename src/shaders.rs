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

// Cosine color palette generator (Inigo Quilez)
vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
    return a + b * cos(TWO_PI * (c * t + d));
}

// Target catalog helper: 8 dense boundary regions across the Mandelbrot set
vec2 get_target_c(int id) {
    int target_idx = int(mod(float(id), 8.0));
    if (target_idx == 0) return vec2(-0.743838635, 0.132019139); // Seahorse Valley Spiral
    if (target_idx == 1) return vec2(0.000980397, 0.822128414);  // Quad Spiral Dendrite
    if (target_idx == 2) return vec2(-0.088846200, 0.654217423); // Triple Spiral Valley
    if (target_idx == 3) return vec2(-1.749771911, -0.000000322);// Mini-Brot Satellite Antenna
    if (target_idx == 4) return vec2(-0.743632452, 0.131962586); // Seahorse Double Spiral
    if (target_idx == 5) return vec2(0.300634684, 0.022717098);  // Elephant Valley Boundary
    if (target_idx == 6) return vec2(-0.101112679, 0.956295445); // Scepter Valley Deep
    return vec2(-0.775980227, 0.136894813);                     // North Seahorse Mini-Brot
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Deep multi-target fractal zoom trajectory with staged dynamic rotation & continuous filament tracking:
    // Dynamic 1x speed:
    // Phase 1 (Zoom-In): 24.0s plunge (1x -> 64,000x), stopping gradually with C1 easing and multi-stage rotation speeds
    // Phase 2 (Zoom-Out): 8.0s exponential acceleration with sigmoid easing return (64,000x -> 1x)
    // Total cycle period = 32.0s
    // Note: Zoom is calibrated to 64,000x to maintain absolute razor sharpness without FP16 block quantization rectangles!
    const float T_IN = 24.0;
    const float T_OUT = 8.0;
    const float PERIOD = T_IN + T_OUT; // 32.0s

    float cycle_idx = floor(u_time / PERIOD);
    float cycle_time = mod(u_time, PERIOD);

    const float MAX_ZOOM = 64000.0;
    const float LN_MAX = 11.066638; // ln(64000.0)

    float zoom = 1.0;
    float rot_angle = 0.0;
    float s_norm = 0.0; // 0.0 (zoomed out) -> 1.0 (deepest zoom)
    float g_out = 0.0;

    if (cycle_time < T_IN) {
        // --- PHASE 1: ZOOM-IN (Deep plunge, stopping gradually with easing) ---
        float p = cycle_time / T_IN; // 0.0 -> 1.0
        
        // Continuous C1 easing curve:
        // Steady plunge for p <= 0.65, then smooth quadratic deceleration to zero velocity at p = 1.0
        const float p0 = 0.65;
        const float v0 = 2.0 / (1.0 + p0); // 1.212121
        const float a_dec = 1.0 / (1.0 - p0 * p0); // 1.7316017
        
        if (p <= p0) {
            s_norm = v0 * p;
        } else {
            float rem = 1.0 - p;
            s_norm = 1.0 - a_dec * rem * rem;
        }
        
        zoom = exp(s_norm * LN_MAX);

        // Multi-stage rotation with frequently changing angular velocity:
        // Base smooth roll from 0.0 to 3.4 radians
        float s_smooth = s_norm * s_norm * (3.0 - 2.0 * s_norm);
        
        // Staged harmonic oscillations that dynamically speed up, hold, and surge rotation:
        // sin^2(pi*p) window guarantees zero velocity at start (p=0) and terminal stop (p=1)
        float window = sin(PI * p);
        float h_rot = (window * window) * (
            0.28 * sin(4.0 * PI * p) +
            0.16 * sin(8.0 * PI * p) +
            0.08 * sin(14.0 * PI * p)
        );
        rot_angle = s_smooth * 3.4 + h_rot;
    } else {
        // --- PHASE 2: ZOOM-OUT (Exponential acceleration with easing, cruise, easing stop) ---
        float q = (cycle_time - T_IN) / T_OUT; // 0.0 -> 1.0
        
        // Normalized logistic sigmoid:
        const float k_sig = 7.5;
        float x = k_sig * (2.0 * q - 1.0);
        float sig = 1.0 / (1.0 + exp(-x));
        const float sig0 = 0.00055278; // 1.0 / (1.0 + exp(7.5))
        const float sig1 = 0.99944722; // 1.0 / (1.0 + exp(-7.5))
        g_out = clamp((sig - sig0) / (sig1 - sig0), 0.0, 1.0);
        
        // Fractional zoom remaining: starts at 1.0 (64,000x), ends at 0.0 (1x)
        s_norm = 1.0 - g_out;
        zoom = exp(s_norm * LN_MAX);

        // Dynamic 360-degree turnaround spin synchronized with zoom-out easing:
        // Starts at 3.4 rad, smoothly finishes at 6.2831853 (2*PI = 0.0 mod 2*PI)
        rot_angle = 3.4 + g_out * (TWO_PI - 3.4);
    }

    // Curated catalog of dense boundary regions across the Mandelbrot set
    // Seamless camera transit: Current target for this plunge, and next target for following plunge
    int curr_id = int(mod(cycle_idx, 8.0));
    int next_id = int(mod(cycle_idx + 1.0, 8.0));
    vec2 c_curr = get_target_c(curr_id);
    vec2 c_next = get_target_c(next_id);

    // Continuous center tracking:
    // During zoom-in, locked strictly on c_curr.
    // During zoom-out, as camera pulls out wide (g_out >= 0.4), center smoothly glides from c_curr to c_next.
    // By the time zoom-out finishes (g_out >= 0.92), center has already arrived at c_next with zero derivative.
    // Thus when cycle loops to p=0, center is already at the new target with zero hop!
    vec2 center;
    if (cycle_time < T_IN) {
        center = c_curr;
    } else {
        float pan_t = smoothstep(0.40, 0.92, g_out);
        center = mix(c_curr, c_next, pan_t);
    }

    // Continuous filament tracking:
    // Steers gently around boundary filaments during deep zoom, and smoothly fades to zero at the zoom-out overview
    float wander_weight = smoothstep(0.08, 0.40, s_norm);
    vec2 boundary_wander = vec2(
        sin(u_time * 0.55) * 0.12 + sin(u_time * 1.3) * 0.03,
        cos(u_time * 0.48) * 0.12 + cos(u_time * 1.1) * 0.03
    ) * (2.85 / zoom) * wander_weight;

    float ca = cos(rot_angle);
    float sa = sin(rot_angle);
    mat2 rot = mat2(ca, -sa, sa, ca);

    vec2 p = (rot * uv) * (2.85 / zoom);

    // Upright Mandelbrot complex plane coordinate: p.y = real (+Y towards cleft), p.x = imag
    vec2 c = vec2(center.x + p.y + boundary_wander.x, center.y + p.x + boundary_wander.y);

    // Mandelbrot escape-time iteration with interior orbit trap analysis
    vec2 z = vec2(0.0);
    float n = 0.0;
    const float MAX_ITER = 240.0;
    float dist_sq = 0.0;
    float min_trap = 1e6;
    vec2 last_z = vec2(0.0);

    for (float i = 0.0; i < MAX_ITER; i += 1.0) {
        dist_sq = dot(z, z);
        if (dist_sq > 4.0) {
            n = i;
            break;
        }
        min_trap = min(min_trap, dist_sq);
        last_z = z;
        z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
    }

    vec3 col;
    if (dist_sq <= 4.0) {
        // Fractal interior: rich glowing quantum nebula & orbit trap magnetic field
        // (Ensures there is always visual structure on screen, never a dead black void)
        float trap_dist = sqrt(min_trap);
        float core_glow = exp(-trap_dist * 3.2);
        float trap_angle = atan(last_z.y, last_z.x);
        float interior_coord = trap_angle / TWO_PI + u_time * 0.06 + trap_dist * 0.8;
        
        vec3 int_pal_a = vec3(0.09, 0.05, 0.16);
        vec3 int_pal_b = vec3(0.14, 0.08, 0.24);
        vec3 int_pal_c = vec3(1.0, 1.0, 1.0);
        vec3 int_pal_d = vec3(0.18, 0.45, 0.75);
        vec3 int_nebula = palette(interior_coord, int_pal_a, int_pal_b, int_pal_c, int_pal_d);
        
        // Edge glow along the boundary rim of the interior
        float edge_rim = exp(-min_trap * 12.0) * 0.55;
        vec3 rim_col = vec3(0.2, 0.85, 1.0) * edge_rim;
        
        col = int_nebula * (0.40 + 0.60 * core_glow) + rim_col;
    } else {
        // Continuous smooth fractional iteration (renormalized escape time)
        float smooth_n = n - log2(max(1.0, 0.5 * log2(dist_sq)));

        // Multi-frequency psychedelic coloring
        float col_coord = smooth_n * 0.042 + u_time * 0.08;

        vec3 pal_a = vec3(0.5, 0.5, 0.5);
        vec3 pal_b = vec3(0.5, 0.5, 0.5);
        vec3 pal_c = vec3(1.0, 1.0, 1.0);
        vec3 pal_d = vec3(0.0, 0.33, 0.67);

        col = palette(col_coord, pal_a, pal_b, pal_c, pal_d);

        // Electric boundary glow along fractal filaments
        float boundary_glow = exp(-0.032 * (MAX_ITER - n));
        vec3 electric_neon = vec3(
            0.5 + 0.5 * sin(col_coord * 6.0),
            0.5 + 0.5 * cos(col_coord * 5.0 + 1.0),
            0.5 + 0.5 * sin(col_coord * 7.0 + 2.0)
        );

        col = mix(col, electric_neon * 1.5, boundary_glow * 0.85);

        // High-contrast S-curve tone mapping (deep rich colors & electric highlights)
        col = pow(max(col, vec3(0.0)), vec3(1.30)) * 1.25;
    }

    fragColor = vec4(clamp(col, 0.0, 1.0), 1.0);
}
"#;
