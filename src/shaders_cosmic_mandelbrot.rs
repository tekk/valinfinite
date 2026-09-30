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

// Cosine color palette generator (Inigo Quilez)
vec3 palette(float t, vec3 a, vec3 b, vec3 c, vec3 d) {
    return a + b * cos(6.2831853 * (c * t + d));
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Deep fractal zoom trajectory with gradual ease-in stop & exponential ease-out return:
    // Phase 1 (Zoom-In): 24.0s plunge into Seahorse Valley (1x -> 64,000x), stopping gradually with easing
    // Phase 2 (Zoom-Out): 8.0s exponential acceleration with easing, cruising at full speed, then easing stop (64,000x -> 1x)
    // Total cycle period = 32.0s
    const float T_IN = 24.0;
    const float T_OUT = 8.0;
    const float PERIOD = T_IN + T_OUT; // 32.0s

    float cycle_time = mod(u_time * 0.90, PERIOD);
    const float MAX_ZOOM = 64000.0;
    const float LN_MAX = 11.066638; // ln(64000.0)

    float zoom = 1.0;
    float rot_angle = 0.0;

    if (cycle_time < T_IN) {
        // --- PHASE 1: ZOOM-IN (Deep plunge, stopping gradually with easing) ---
        float p = cycle_time / T_IN; // 0.0 -> 1.0
        
        // Continuous C1 easing curve:
        // Steady plunge for p <= 0.65, then smooth deceleration to zero velocity at p = 1.0
        float s_in;
        const float p0 = 0.65;
        const float v0 = 2.0 / (1.0 + p0); // 1.212121
        const float a_dec = 1.0 / (1.0 - p0 * p0); // 1.7316017
        
        if (p <= p0) {
            s_in = v0 * p;
        } else {
            float rem = 1.0 - p;
            s_in = 1.0 - a_dec * rem * rem;
        }
        
        zoom = exp(s_in * LN_MAX);

        // Gentle spiral rotation inward, easing to zero rotation speed at the peak
        rot_angle = s_in * 2.2 + sin(u_time * 0.20) * 0.20 * (1.0 - 0.5 * s_in);
    } else {
        // --- PHASE 2: ZOOM-OUT (Exponential acceleration with easing, cruise, easing stop) ---
        float q = (cycle_time - T_IN) / T_OUT; // 0.0 -> 1.0
        
        // Normalized logistic sigmoid:
        // Speed increases exponentially with easing (d/dq ~ e^(k*q)),
        // eases into full zoom-out speed around q = 0.5,
        // and decelerates with easing to a total stop at q = 1.0
        const float k_sig = 7.5;
        float x = k_sig * (2.0 * q - 1.0);
        float sig = 1.0 / (1.0 + exp(-x));
        const float sig0 = 0.00055278; // 1.0 / (1.0 + exp(7.5))
        const float sig1 = 0.99944722; // 1.0 / (1.0 + exp(-7.5))
        float g_out = clamp((sig - sig0) / (sig1 - sig0), 0.0, 1.0);
        
        // Fractional zoom remaining: starts at 1.0 (64,000x), ends at 0.0 (1x)
        float s_out = 1.0 - g_out;
        zoom = exp(s_out * LN_MAX);

        // Dynamic 360-degree turnaround spin synchronized with zoom-out easing
        rot_angle = 2.2 + g_out * (6.2831853 - 2.2);
    }

    // Target coordinates: The Seahorse Spiral Valley of the Mandelbrot
    // Upright orientation: cusp (cleft) at top (+Y), antenna/bulb at bottom (-Y)
    const vec2 center = vec2(-0.743643887, 0.131825904);

    float ca = cos(rot_angle);
    float sa = sin(rot_angle);
    mat2 rot = mat2(ca, -sa, sa, ca);

    vec2 p = rot * uv * (2.85 / zoom);

    // Upright Mandelbrot complex plane coordinate: p.y = real (+Y towards cleft), p.x = imag
    vec2 c = vec2(center.x + p.y, center.y + p.x);

    // Mandelbrot escape-time iteration
    vec2 z = vec2(0.0);
    float n = 0.0;
    const float MAX_ITER = 180.0;
    float dist_sq = 0.0;

    for (float i = 0.0; i < MAX_ITER; i += 1.0) {
        dist_sq = dot(z, z);
        if (dist_sq > 4.0) {
            n = i;
            break;
        }
        z = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
    }

    vec3 col;
    if (dist_sq <= 4.0) {
        // Fractal interior: deep velvet cosmic void
        col = vec3(0.012, 0.004, 0.022);
    } else {
        // Continuous smooth fractional iteration (renormalized escape time)
        float smooth_n = n - log2(max(1.0, 0.5 * log2(dist_sq)));

        // Multi-frequency psychedelic coloring
        float col_coord = smooth_n * 0.038 + u_time * 0.12;

        vec3 pal_a = vec3(0.5, 0.5, 0.5);
        vec3 pal_b = vec3(0.5, 0.5, 0.5);
        vec3 pal_c = vec3(1.0, 1.0, 1.0);
        vec3 pal_d = vec3(0.0, 0.33, 0.67);

        col = palette(col_coord, pal_a, pal_b, pal_c, pal_d);

        // Electric boundary glow along fractal filaments
        float boundary_glow = exp(-0.035 * (MAX_ITER - n));
        vec3 electric_neon = vec3(
            0.5 + 0.5 * sin(col_coord * 6.0),
            0.5 + 0.5 * cos(col_coord * 5.0 + 1.0),
            0.5 + 0.5 * sin(col_coord * 7.0 + 2.0)
        );

        col = mix(col, electric_neon * 1.5, boundary_glow * 0.85);

        // High-contrast S-curve tone mapping (deep blacks & electric highlights)
        col = pow(max(col, vec3(0.0)), vec3(1.35)) * 1.25;
    }

    fragColor = vec4(clamp(col, 0.0, 1.0), 1.0);
}
"#;
