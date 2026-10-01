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

// ==========================================
// 1. WARM VIVID COLOR PALETTES
// ==========================================
// Molten gold, sunset vermilion, fiery amber, peach, radiant magenta
vec3 warm_palette(float t) {
    vec3 a = vec3(0.68, 0.42, 0.22);
    vec3 b = vec3(0.55, 0.45, 0.32);
    vec3 c = vec3(1.0, 1.0, 1.0);
    vec3 d = vec3(0.00, 0.25, 0.52);
    return clamp(a + b * cos(TWO_PI * (c * t + d)), 0.0, 1.0);
}

vec3 warm_palette_secondary(float t) {
    vec3 a = vec3(0.72, 0.35, 0.18);
    vec3 b = vec3(0.48, 0.40, 0.35);
    vec3 c = vec3(1.2, 0.9, 0.7);
    vec3 d = vec3(0.05, 0.32, 0.65);
    return clamp(a + b * cos(TWO_PI * (c * t + d)), 0.0, 1.0);
}

// ==========================================
// 2. ANALYTICAL SDF PRIMITIVES
// ==========================================

// Exact Circle SDF
float sdCircle(vec2 p, float r) {
    return length(p) - r;
}

// Inigo Quilez Exact Heart SDF
float sdHeart(vec2 p) {
    p.x = abs(p.x);
    p.y += 0.45; // Center heart: sharp tip at bottom, rounded lobes at top
    if (p.y + p.x > 1.0) {
        vec2 q = p - vec2(0.25, 0.75);
        return length(q) - 0.35355339;
    }
    vec2 q1 = p - vec2(0.0, 1.0);
    vec2 q2 = p - 0.5 * max(p.x + p.y, 0.0);
    return sqrt(min(dot(q1, q1), dot(q2, q2))) * sign(p.x - p.y);
}

// Rounded Box / Square SDF
float sdBox(vec2 p, vec2 b, float r) {
    vec2 q = abs(p) - b + r;
    return length(max(q, 0.0)) + min(max(q.x, q.y), 0.0) - r;
}

// Octagon SDF
float sdOctagon(vec2 p, float r) {
    const vec3 k = vec3(-0.9238795325, 0.3826834323, 0.4142135623);
    p = abs(p);
    p -= 2.0 * min(dot(vec2(k.x, k.y), p), 0.0) * vec2(k.x, k.y);
    p -= 2.0 * min(dot(vec2(-k.x, k.y), p), 0.0) * vec2(-k.x, k.y);
    p -= vec2(clamp(p.x, -k.z * r, k.z * r), r);
    return length(p) * sign(p.y);
}

// Hexagon SDF
float sdHexagon(vec2 p, float r) {
    const vec3 k = vec3(-0.866025404, 0.5, 0.577350269);
    p = abs(p);
    p -= 2.0 * min(dot(k.xy, p), 0.0) * k.xy;
    p -= vec2(clamp(p.x, -k.z * r, k.z * r), r);
    return length(p) * sign(p.y);
}

// Star SDF (5-pointed)
float sdStar5(vec2 p, float r, float rf) {
    const vec2 k1 = vec2(0.80901699437, -0.58778525229);
    const vec2 k2 = vec2(-k1.x, k1.y);
    p.x = abs(p.x);
    p -= 2.0 * max(dot(k1, p), 0.0) * k1;
    p -= 2.0 * max(dot(k2, p), 0.0) * k2;
    p.x = abs(p.x);
    p.y -= r;
    vec2 ba = rf * vec2(-k1.y, k1.x) - vec2(0.0, 1.0);
    float h = clamp(dot(p, ba) / dot(ba, ba), 0.0, r);
    return length(p - ba * h) * sign(p.y * ba.x - p.x * ba.y);
}

// Random / Organic Multilobe Polygon
float sdOrganicPoly(vec2 p) {
    float a = atan(p.y, p.x);
    float r = 0.44 + 0.08 * sin(a * 5.0 + 1.2) 
                   + 0.05 * cos(a * 3.0 - 0.8) 
                   + 0.03 * sin(a * 7.0 + 2.4);
    return length(p) - r;
}

// Randomly moving Bezier curve control points
float sdBezierMoving(vec2 p, float t) {
    float a = atan(p.y, p.x);
    float r = length(p);
    float r_bez = 0.42 + 0.11 * sin(a * 4.0 + t * 2.5)
                        + 0.07 * cos(a * 6.0 - t * 3.2 + sin(t * 1.4))
                        + 0.04 * sin(a * 9.0 + t * 4.1);
    return r - r_bez;
}

// ==========================================
// 3. MORPHOLOGY ENGINE FOR SCENE A
// ==========================================
float evaluate_scene_a_shape(vec2 p, float anim_time) {
    float t = mod(max(0.0, anim_time), 46.0);
    
    if (t < 6.0) {
        // Unfilled circle: starts small (r=0.12), expands, gets bolder until solid circle
        float frac = t / 6.0;
        float r = mix(0.12, 0.52, smoothstep(0.0, 1.0, frac));
        float th = mix(0.008, 0.55, pow(frac, 1.8));
        // Mathematically exact annular ring shrinking hole until solid
        return max(length(p) - r, -(length(p) - max(0.0, r - th)));
    } else if (t < 17.0) {
        // Morph into Heart (stays 11s!) with gentle heartbeat
        float morph_t = smoothstep(0.0, 1.8, t - 6.0);
        float d_prev = sdCircle(p, 0.52);
        // Soft romantic pulse
        float pulse = 1.0 + 0.03 * sin(t * 4.0) * exp(-mod(t * 1.33, 1.0) * 2.5);
        vec2 p_heart = (p / 0.68) / pulse;
        float d_heart = sdHeart(p_heart) * 0.68 * pulse;
        return mix(d_prev, d_heart, morph_t);
    } else if (t < 22.0) {
        // Morph into Rounded Square
        float morph_t = smoothstep(0.0, 1.5, t - 17.0);
        vec2 p_heart = p / 0.68;
        float d_prev = sdHeart(p_heart) * 0.68;
        float d_square = sdBox(p, vec2(0.40), 0.10);
        return mix(d_prev, d_square, morph_t);
    } else if (t < 27.0) {
        // Morph into Octagon
        float morph_t = smoothstep(0.0, 1.5, t - 22.0);
        float d_prev = sdBox(p, vec2(0.40), 0.10);
        float d_oct = sdOctagon(p, 0.48);
        return mix(d_prev, d_oct, morph_t);
    } else if (t < 32.0) {
        // Morph into Hexagon
        float morph_t = smoothstep(0.0, 1.5, t - 27.0);
        float d_prev = sdOctagon(p, 0.48);
        float d_hex = sdHexagon(p, 0.50);
        return mix(d_prev, d_hex, morph_t);
    } else if (t < 37.0) {
        // Morph into Star
        float morph_t = smoothstep(0.0, 1.5, t - 32.0);
        float d_prev = sdHexagon(p, 0.50);
        float d_star = sdStar5(p, 0.56, 0.42);
        return mix(d_prev, d_star, morph_t);
    } else if (t < 41.0) {
        // Morph into Organic Polygon
        float morph_t = smoothstep(0.0, 1.5, t - 37.0);
        float d_prev = sdStar5(p, 0.56, 0.42);
        float d_org = sdOrganicPoly(p);
        return mix(d_prev, d_org, morph_t);
    } else {
        // Morph into Moving Bezier Curve with Lightning
        float morph_t = smoothstep(0.0, 1.5, t - 41.0);
        float d_prev = sdOrganicPoly(p);
        float d_bez = sdBezierMoving(p, t);
        return mix(d_prev, d_bez, morph_t);
    }
}

// Scene B: Radiant Sacred Rosette / Hypnotic Torus Metamorphosis
float evaluate_scene_b_shape(vec2 p, float anim_time) {
    float t = mod(max(0.0, anim_time), 46.0);
    float a = atan(p.y, p.x);
    float r = length(p);
    
    // Smoothly morphing petal order: 3 -> 5 -> 8 -> 12 -> spiral rosette
    float n_petals = mix(3.0, 12.0, smoothstep(0.0, 46.0, t));
    float petal_amp = 0.16 + 0.05 * sin(t * 0.8);
    float base_r = 0.45 + 0.06 * cos(t * 0.5);
    
    float rosette = base_r + petal_amp * cos(n_petals * a + t * 0.9) 
                           + 0.04 * sin(a * 4.0 - t * 1.5);
    return r - rosette;
}

// ==========================================
// 4. UNEXPECTED PROCEDURAL RANDOM EFFECTS
// ==========================================

// Effect 2: Solar Shockwave Lensing Ripple
vec2 fxShockwave(vec2 p, float t) {
    float cycle = mod(t * 0.32, 4.0);
    float r = length(p);
    float wave_front = cycle * 0.70;
    float d = abs(r - wave_front);
    float ripple = sin((r - wave_front) * 40.0) * exp(-d * 16.0) * smoothstep(2.5, 0.0, cycle);
    return p + normalize(p + 1e-4) * ripple * 0.032;
}

// Effect 3: Ethereal Glowing Embers Swarm
float fxEmbers(vec2 p, float t) {
    float sparks = 0.0;
    for (int i = 0; i < 7; i++) {
        float fi = float(i);
        vec2 pos = vec2(
            0.62 * sin(t * 0.60 + fi * 1.57) * cos(t * 0.30 + fi * 0.8),
            0.58 * cos(t * 0.48 + fi * 2.1) + 0.20 * sin(t * 1.10 + fi * 3.2)
        );
        float d = length(p - pos);
        sparks += exp(-d * 45.0) * 1.5 + exp(-d * 10.0) * 0.30;
    }
    return sparks;
}

// Effect 4: Volumetric Golden Sunbeams
float fxSunbeams(vec2 p, float t) {
    float phi = atan(p.y, p.x);
    float r = length(p);
    float rays = max(0.0, sin(phi * 8.0 + t * 0.30) * cos(phi * 5.0 - t * 0.20));
    return pow(rays, 3.5) * exp(-r * 1.2);
}

// Effect 5: Cosmic Auroral Magnetic Filaments
float fxMagneticFilaments(vec2 p, float t) {
    float r = length(p);
    float phi = atan(p.y, p.x);
    float lines = sin(phi * 12.0 + sin(r * 14.0 - t * 2.2));
    return smoothstep(0.70, 1.0, lines) * exp(-abs(r - 0.68) * 5.0);
}

// ==========================================
// 5. ACES FILM TONE MAPPING
// ==========================================
vec3 aces_tonemap(vec3 x) {
    const float a = 2.51;
    const float b = 0.03;
    const float c = 2.43;
    const float d = 0.59;
    const float e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), 0.0, 1.0);
}

// ==========================================
// 6. MAIN MULTI-LAYER PIPELINE
// ==========================================
void main() {
    vec2 raw_uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Two Interchanging Scene Cycles of 58.0 seconds each:
    // Scene 0: Layered Bezier Metamorphosis (Circle -> Heart -> Square -> Octagon -> Hexagon -> Star -> Polygon -> Bezier + Lightning)
    // Scene 1: Radiant Sacred Rosette / Hypnotic Torus Metamorphosis
    const float SCENE_DURATION = 58.0;
    float global_time = u_time * 0.95;
    int scene_idx = int(floor(global_time / SCENE_DURATION)) % 2;
    float scene_t = mod(global_time, SCENE_DURATION);

    // Apply Refractive Solar Shockwave Lensing (Effect 2)
    vec2 p = fxShockwave(raw_uv, u_time);

    // Dynamic subtle camera breathing & organic rotation
    float roll = 0.12 * sin(u_time * 0.30);
    float ca = cos(roll), sa = sin(roll);
    p = mat2(ca, -sa, sa, ca) * p;

    // Atmospheric warm background with subtle radial gradient
    vec3 col_accum = vec3(0.035, 0.012, 0.022) * (1.0 - 0.45 * length(p));

    // Multi-Layer Compositing (6 Concentric Out-of-Phase Layers)
    const int NUM_LAYERS = 6;
    float anim_t = max(0.0, scene_t - 4.5);

    for (int l = 0; l < NUM_LAYERS; l++) {
        float fl = float(l);
        float layer_t = max(0.0, anim_t - fl * 0.18);
        float scale = 1.0 + fl * 0.20 + 0.03 * sin(u_time * 1.5 + fl);
        
        float rot_l = (fl - 2.5) * 0.06 * sin(u_time * 0.40) + fl * 0.04;
        float cal = cos(rot_l), sal = sin(rot_l);
        vec2 p_layer = (mat2(cal, -sal, sal, cal) * p) / scale;
        
        float d_layer = 0.0;
        if (scene_idx == 0) {
            d_layer = evaluate_scene_a_shape(p_layer, layer_t) * scale;
        } else {
            d_layer = evaluate_scene_b_shape(p_layer, layer_t) * scale;
        }
        
        // Multi-tier optical profile
        float d_edge = abs(d_layer);
        float stroke = exp(-d_edge * (48.0 - fl * 3.0));
        float aura = exp(-d_edge * (8.0 - fl * 0.6));
        float fill = smoothstep(0.01, -0.20, d_layer) * 0.14;
        
        // Warm psychedelic color coordinate shifted per layer & time
        float col_coord = fract(u_time * 0.06 + fl * 0.14 + d_layer * 0.18);
        vec3 col_layer = (scene_idx == 0) ? warm_palette(col_coord) : warm_palette_secondary(col_coord);
        
        vec3 layer_light = col_layer * (stroke * 1.5 + aura * 0.55 + fill)
                         + vec3(1.0, 0.94, 0.82) * pow(stroke, 3.0) * 0.9;
                         
        float layer_w = 1.0 / (1.0 + fl * 0.28);
        col_accum += layer_light * layer_w * 0.42;
    }

    // Effect 3: Glowing Embers Swarm
    float embers = fxEmbers(p, u_time);
    col_accum += vec3(1.0, 0.62, 0.20) * embers * 0.55;

    // Effect 4: Volumetric Sunbeams
    float sunbeams = fxSunbeams(p, u_time);
    col_accum += vec3(0.9, 0.65, 0.25) * sunbeams * 0.35;

    // Effect 5: Magnetic Auroral Filaments
    float filaments = fxMagneticFilaments(p, u_time);
    col_accum += vec3(1.0, 0.35, 0.55) * filaments * 0.45;

    // ==========================================
    // MACRO PHASE COMPOSITING: INTRO / OUTRO / CROSSING
    // ==========================================
    // 0s-4.5s: Intro
    // 4.5s-50.0s: Main Layered Bezier Evolution
    // 50.0s-54.0s: Outro (Singularity & Supernova)
    // 54.0s-58.0s: Scene Interchange Crossing
    if (scene_t < 4.5) {
        // INTRO: Radiant solar ignition & opening shockwave
        float intro_fade = smoothstep(0.0, 3.5, scene_t);
        float core_flare = exp(-length(p) * 7.0) * (4.5 - scene_t) * 0.6;
        col_accum *= intro_fade;
        col_accum += vec3(1.0, 0.82, 0.50) * max(0.0, core_flare);
    } else if (scene_t >= 50.0 && scene_t < 54.0) {
        // OUTRO: Gravitational singularity collapse into Supernova pulse
        float outro_t = (scene_t - 50.0) / 4.0; // 0..1
        float supernova = exp(-length(p) * mix(7.0, 1.5, outro_t)) * pow(outro_t, 2.0) * 1.5;
        col_accum += vec3(1.0, 0.88, 0.65) * supernova;
    } else if (scene_t >= 54.0) {
        // SCENE INTERCHANGE CROSSING: Diagonal chromatic wipe into next scene
        float cross_t = (scene_t - 54.0) / 4.0; // 0..1
        float wipe_line = (raw_uv.x + raw_uv.y) * 0.707 + (cross_t * 3.2 - 1.6);
        float wipe_mask = smoothstep(-0.20, 0.20, wipe_line);
        float wipe_beam = exp(-abs(wipe_line) * 9.0) * 1.4;
        
        vec3 next_scene_hint = warm_palette_secondary(cross_t * 0.4 + length(p) * 0.4) * 0.35;
        col_accum = mix(col_accum, next_scene_hint, wipe_mask);
        col_accum += vec3(1.0, 0.90, 0.70) * wipe_beam;
    }

    // ACES Filmic Tone Mapping for rich vivid saturation without blowout
    vec3 mapped = aces_tonemap(col_accum * 1.35);

    // Subtle film vignette
    float vig = 1.0 - 0.28 * dot(raw_uv, raw_uv);
    mapped *= clamp(vig, 0.0, 1.0);

    fragColor = vec4(mapped, 1.0);
}
"#;
