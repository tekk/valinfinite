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

    // Continuous, buttery-smooth Shepard multi-scale infinite zoom synthesis
    // Using 4 overlapping logarithmic octaves (scale factor S = 3.2 per octave)
    const float S = 3.2;
    const float lnS = 1.1631508; // ln(3.2)
    const int NUM_OCTAVES = 4;
    const int ITERATIONS = 12;

    vec3 total_color = vec3(0.0);
    float total_weight = 0.0;

    // Zoom speed: steady, uninterrupted inward motion
    float zoom_speed = 0.35;
    float base_time = u_time * zoom_speed;

    // Psychedelic neon palette components
    vec3 pal_a = vec3(0.5, 0.5, 0.5);
    vec3 pal_b = vec3(0.5, 0.5, 0.5);
    vec3 pal_c = vec3(1.0, 1.0, 1.0);
    vec3 pal_d = vec3(0.0, 0.33, 0.67);

    // Completely continuous, mathematically smooth heartbeat glow pulsation
    float beat_osc = sin(u_time * 6.2831853 * 1.15);
    float glow_pulse = beat_osc * beat_osc * 0.45;

    // Gentle global breathing sway
    float sway = 0.06 * sin(u_time * 0.4);
    float ca = cos(sway);
    float sa = sin(sway);

    for (int o = 0; o < NUM_OCTAVES; o++) {
        float phase = fract(base_time + float(o) * 0.25);
        float scale = exp(phase * lnS);

        // Smooth Hanning window with quadratic taper for completely invisible octave crossfades
        float w = 0.5 - 0.5 * cos(6.2831853 * phase);
        w = w * w;

        vec2 z = uv * scale;
        float accum = 0.0;
        float min_trap = 1e8;
        float edge_trap = 1e8;

        for (int i = 0; i < ITERATIONS; i++) {
            // 1. Bilateral symmetry fold
            z.x = abs(z.x);

            // 2. Smooth heart cusp fold (cardioid cleft)
            z.y -= 0.58 * (sqrt(z.x + 0.035) - 0.187);

            // 3. Spherical inversion / Kaleidoscopic fractal chamber
            float r2 = dot(z, z) + 1e-4;
            if (r2 < 0.22) {
                z *= (1.0 / 0.22);
            } else if (r2 < 1.38) {
                z *= (1.0 / r2);
            }

            // 4. Subtle rotation and scale expansion
            z = mat2(ca, -sa, sa, ca) * z * 1.48 - vec2(0.18, 0.30);

            // 5. Heart orbit trap
            float hx = abs(z.x);
            float hy = z.y - 0.52 * (sqrt(hx + 0.035) - 0.187);
            float hd = length(vec2(hx, hy));

            min_trap = min(min_trap, hd);
            float et = abs(z.x * z.y);
            edge_trap = min(edge_trap, et);

            accum += exp(-3.5 * hd) + 0.5 * exp(-7.0 * et);
        }

        // Continuous multi-frequency color mapping
        float color_coord = accum * 0.18 + min_trap * 1.4 + u_time * 0.42 + float(o) * 0.25;
        vec3 col_layer = palette(color_coord, pal_a, pal_b, pal_c, pal_d);

        // Radiant neon aura surrounding heart contours
        float aura = exp(-1.1 * min_trap);
        vec3 neon = vec3(
            0.65 + 0.35 * sin(u_time * 1.6 + color_coord * 4.0),
            0.55 + 0.45 * cos(u_time * 1.9 + color_coord * 3.0),
            0.85 + 0.15 * sin(u_time * 2.2 + color_coord * 5.0)
        );
        col_layer = mix(col_layer, neon, aura * 0.7);

        // White-hot core singularity beam with smooth organic pulse
        float core_dist = length(uv) * scale;
        float core_beam = exp(-7.0 * core_dist) * (1.0 + glow_pulse);
        col_layer += vec3(1.2, 0.55, 0.95) * core_beam * 2.2;

        // Electric iridescent filament lines
        float edge_line = exp(-11.0 * edge_trap);
        col_layer += vec3(0.2, 0.95, 1.0) * edge_line * 0.9;

        total_color += col_layer * w;
        total_weight += w;
    }

    vec3 final_color = total_color / max(total_weight, 1e-5);

    // Subtle chromatic dispersion at peripheral edges
    float dist_sq = dot(uv, uv);
    final_color.r += 0.07 * sin(dist_sq * 9.0 + u_time * 2.0);
    final_color.b += 0.07 * cos(dist_sq * 8.0 - u_time * 2.5);

    // Soft peripheral vignette tailored for both portrait mobile & landscape
    float vignette = clamp(1.0 - 0.22 * dist_sq, 0.0, 1.0);
    final_color *= vignette;

    // S-curve contrast and punchy vibrance
    final_color = pow(max(final_color, vec3(0.0)), vec3(0.82));
    final_color = clamp(final_color, 0.0, 1.0);

    fragColor = vec4(final_color, 1.0);
}
"#;
