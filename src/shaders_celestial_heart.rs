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
    // Aspect-ratio normalized coordinates, centered at origin
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Continuous, hypnotic camera vortex rotation
    float cam_rot = u_time * 0.40;
    float ca_cam = cos(cam_rot);
    float sa_cam = sin(cam_rot);
    vec2 rot_uv = mat2(ca_cam, -sa_cam, sa_cam, ca_cam) * uv;

    float dist_sq = dot(rot_uv, rot_uv);
    float r_len = sqrt(dist_sq);

    // Shepard multi-scale infinite zoom synthesis (4 overlapping logarithmic octaves)
    const float S = 3.2;
    const float lnS = 1.1631508; // ln(3.2)
    const int NUM_OCTAVES = 4;
    const int ITERATIONS = 13;

    vec3 total_color = vec3(0.0);
    float total_weight = 0.0;

    // Increased pace: exhilarating, fast-paced infinite plunge
    float zoom_speed = 0.65;
    float base_time = u_time * zoom_speed;

    // Rich psychedelic neon palette components
    vec3 pal_a = vec3(0.5, 0.5, 0.5);
    vec3 pal_b = vec3(0.5, 0.5, 0.5);
    vec3 pal_c = vec3(1.0, 1.0, 1.0);
    vec3 pal_d = vec3(0.0, 0.33, 0.67);

    // Dynamic internal breathing twist
    float twist = 0.35 + 0.12 * sin(u_time * 0.75);
    float ca_tw = cos(twist);
    float sa_tw = sin(twist);
    mat2 rot_tw = mat2(ca_tw, -sa_tw, sa_tw, ca_tw);

    for (int o = 0; o < NUM_OCTAVES; o++) {
        float phase = fract(base_time + float(o) * 0.25);
        float scale = exp(phase * lnS);

        // Smooth Hanning window with quadratic taper for completely seamless infinite loop
        float w = 0.5 - 0.5 * cos(6.2831853 * phase);
        w = w * w;

        vec2 z = rot_uv * scale;
        float accum = 0.0;
        float min_trap = 1e8;
        float edge_trap = 1e8;

        for (int i = 0; i < ITERATIONS; i++) {
            // 1. Bilateral symmetry fold
            z.x = abs(z.x);

            // 2. Smooth heart cusp fold (cardioid cleft)
            z.y -= 0.60 * (sqrt(max(0.0, z.x) + 0.035) - 0.187);

            // 3. Spherical inversion (creates recursive chambers)
            float r2 = dot(z, z) + 1e-4;
            if (r2 < 0.22) {
                z *= (1.0 / 0.22);
            } else if (r2 < 1.40) {
                z *= (1.0 / r2);
            }

            // 4. Secondary box fold (fractal heart sub-structures)
            z = abs(z) - vec2(0.08, 0.16);

            // 5. Dynamic vortex rotation & scale expansion
            z = rot_tw * z * 1.50 - vec2(0.16, 0.28);

            // 6. Heart orbit traps
            float hx = abs(z.x);
            float hy = z.y - 0.52 * (sqrt(hx + 0.035) - 0.187);
            float hd = length(vec2(hx, hy));

            min_trap = min(min_trap, hd);
            float et = abs(z.x * z.y);
            edge_trap = min(edge_trap, et);

            accum += exp(-3.5 * hd) + 0.45 * exp(-7.0 * et);
        }

        // Dynamic multi-frequency color mapping
        float color_coord = accum * 0.20 + min_trap * 1.4 + u_time * 0.55 + float(o) * 0.25;
        vec3 col = palette(color_coord, pal_a, pal_b, pal_c, pal_d);

        // DARK ZONES: Deep cosmic shadows and negative space chasms
        float chasm = 0.5 + 0.5 * cos(accum * 1.5 - min_trap * 3.5 + u_time * 1.6);
        float shadow = pow(chasm, 2.2);

        // Glowing neon aura along heart contours
        float aura = exp(-1.8 * min_trap);
        vec3 neon = vec3(
            0.5 + 0.5 * sin(u_time * 1.8 + color_coord * 4.0),
            0.5 + 0.5 * cos(u_time * 2.2 + color_coord * 3.0),
            0.5 + 0.5 * sin(u_time * 2.6 + color_coord * 5.0 + 2.0)
        );

        // Electric iridescent filament lines piercing through the darkness
        float edge_line = exp(-12.0 * edge_trap);
        vec3 filament_col = vec3(
            0.5 + 0.5 * cos(color_coord * 6.0 + 0.0),
            0.5 + 0.5 * cos(color_coord * 6.0 + 2.0),
            0.5 + 0.5 * cos(color_coord * 6.0 + 4.0)
        );

        // Focused white-hot core singularity beam
        float core_beam = exp(-8.0 * (r_len * scale));

        vec3 layer_col = (col * 0.8 + neon * aura * 0.8) * shadow
                       + edge_line * filament_col * 1.6
                       + vec3(1.5, 0.8, 1.3) * core_beam;

        total_color += layer_col * w;
        total_weight += w;
    }

    vec3 final_color = total_color / max(total_weight, 1e-5);

    // Peripheral chromatic dispersion
    final_color.r += 0.05 * sin(dist_sq * 10.0 + u_time * 2.5) * sqrt(dist_sq);
    final_color.b += 0.05 * cos(dist_sq * 9.0 - u_time * 3.0) * sqrt(dist_sq);

    // HIGH CONTRAST: Sigmoid S-curve with deep pitch-black shadows & electric highlights
    vec3 c_pow = pow(max(final_color, vec3(0.0)), vec3(1.6));
    final_color = (c_pow / (c_pow + vec3(0.16))) * 1.30;
    final_color = clamp(final_color, 0.0, 1.0);

    fragColor = vec4(final_color, 1.0);
}
"#;
