import http.server
import socketserver
import subprocess
import threading
import urllib.parse
import base64
import os
import re
import sys
import time

PORT = 8199
FRAMES_PER_SCENE = 30
FRAME_DIR = "/tmp/valinfinite_gif_frames"
OUTPUT_DIR = "/home/tekk/dev/wasm-infinite-gpu-fractal/screenshots"

os.makedirs(FRAME_DIR, exist_ok=True)
os.makedirs(OUTPUT_DIR, exist_ok=True)

# Extract fragment shaders from git
def get_shader(branch):
    cmd = ["git", "show", f"{branch}:src/shaders.rs"]
    out = subprocess.check_output(cmd, encoding="utf-8")
    m = re.search(r'pub const FRAGMENT_SHADER_SOURCE:\s*&str\s*=\s*r#"([\s\S]+?)"#;', out)
    if not m:
        raise ValueError(f"Could not extract fragment shader from {branch}")
    return m.group(1).strip()

shaders = {
    'v1': get_shader('v1'),
    'v2': get_shader('v2'),
    'v3': get_shader('v3'),
    'v4': get_shader('v4')
}

done_events = {
    'v1': threading.Event(),
    'v2': threading.Event(),
    'v3': threading.Event(),
    'v4': threading.Event()
}

class GIFHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass # Quiet

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        params = urllib.parse.parse_qs(parsed.query)
        scene = params.get('scene', ['v1'])[0]
        fs_code = shaders.get(scene, shaders['v1'])

        # Escape backticks if any
        fs_js = fs_code.replace('\\', '\\\\').replace('`', '\\`').replace('$', '\\$')

        # Time step config
        # v1: Shepard zoom step
        # v2: Matrix rain
        # v3: Mandelbrot zoom
        # v4: Vortex void
        time_steps = {
            'v1': 0.08,
            'v2': 0.07,
            'v3': 0.35,
            'v4': 0.08
        }
        start_times = {
            'v1': 1.0,
            'v2': 1.0,
            'v3': 24.0,
            'v4': 1.0
        }
        step = time_steps.get(scene, 0.08)
        t_start = start_times.get(scene, 1.0)

        html = f"""<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><style>body,html{{margin:0;padding:0;overflow:hidden;background:#000;}}canvas{{display:block;width:240px;height:426px;}}</style></head>
<body>
<canvas id="c" width="240" height="426"></canvas>
<script>
const scene = "{scene}";
const fsSource = `{fs_js}`;
const vsSource = `#version 300 es
in vec2 a_pos;
void main() {{ gl_Position = vec4(a_pos, 0.0, 1.0); }}
`;

const canvas = document.getElementById('c');
const gl = canvas.getContext('webgl2', {{ alpha: false, preserveDrawingBuffer: true, antialias: false }});

function createShader(gl, type, src) {{
    const s = gl.createShader(type);
    gl.shaderSource(s, src);
    gl.compileShader(s);
    if (!gl.getShaderParameter(s, gl.COMPILE_STATUS)) {{
        console.error(gl.getShaderInfoLog(s));
    }}
    return s;
}}

const vs = createShader(gl, gl.VERTEX_SHADER, vsSource);
const fs = createShader(gl, gl.FRAGMENT_SHADER, fsSource);
const prog = gl.createProgram();
gl.attachShader(prog, vs);
gl.attachShader(prog, fs);
gl.linkProgram(prog);
gl.useProgram(prog);

const buf = gl.createBuffer();
gl.bindBuffer(gl.ARRAY_BUFFER, buf);
gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([
    -1,-1, 1,-1, -1,1,
    -1,1, 1,-1, 1,1
]), gl.STATIC_DRAW);

const pos = gl.getAttribLocation(prog, 'a_position') !== -1 ? gl.getAttribLocation(prog, 'a_position') : gl.getAttribLocation(prog, 'a_pos');
if (pos !== -1) {{
    gl.enableVertexAttribArray(pos);
    gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0);
}}

const uRes = gl.getUniformLocation(prog, 'u_resolution');
const uTime = gl.getUniformLocation(prog, 'u_time');
const uFont = gl.getUniformLocation(prog, 'u_fontTexture');

if (uFont) {{
    // Generate atlas for v2
    const atlasCanvas = document.createElement('canvas');
    atlasCanvas.width = 1024;
    atlasCanvas.height = 1024;
    const ctx = atlasCanvas.getContext('2d');
    ctx.fillStyle = '#000000';
    ctx.fillRect(0, 0, 1024, 1024);

    const katakana = ['ア','イ','ウ','エ','オ','カ','キ','ク','ケ','コ','サ','シ','ス','セ','ソ','タ','チ','ツ','テ','ト','ナ','ニ','ヌ','ネ','ノ','ハ','ヒ','フ','ヘ','ホ','マ','ミ','ム','メ','モ','ヤ','ユ','ヨ','ラ','リ','ル','レ','ロ','ワ','ヲ','ン'];
    const kanji = ['愛','心','命','夢','魂','恋','龍','光','星','零','壱','幻','闇','炎','雷','鏡','永','破','絆','宙','響','創','神','無'];
    const runes = ['§','※','⚡','✦','Ξ','Ψ','Ω','λ','π','∞','░','▒','▓','⌗','◊','∿','⌖','⌬','⎊','⎋','⏣','⟁','⟐','⧖','⨂','<>','::','0','1','2','3','4','5','6','7','8','9','+','*','#'];
    const symbols = ['α','β','γ','δ','ε','ζ','η','θ','μ','ξ','⌖','⌬','⎊','⎋','⏣','⟁','⟐','⧖','⨂','∑','∫'];
    const banners = {{ 200:'光・速', 201:'宇・宙', 202:'電・脳', 203:'時・空', 204:'真・実', 205:'幻・影', 206:'混・沌', 207:'覚・醒', 208:'永・遠', 209:'生・命', 210:'愛・心', 211:'命・夢', 212:'無・限' }};

    function drawCell(idx, text, sz) {{
        const c = idx % 16, r = Math.floor(idx / 16);
        ctx.font = 'bold ' + sz + 'px "Courier New", monospace, sans-serif';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillStyle = '#ffffff';
        ctx.fillText(text, c * 64 + 32, r * 64 + 32);
    }}
    katakana.forEach((ch, i) => drawCell(i, ch, 40));
    kanji.forEach((ch, i) => drawCell(46 + i, ch, 40));
    runes.forEach((ch, i) => drawCell(70 + i, ch, 36));
    symbols.forEach((ch, i) => drawCell(110 + i, ch, 42));
    for (const [idxStr, text] of Object.entries(banners)) {{
        drawCell(parseInt(idxStr, 10), text, text.length > 5 ? 14 : 18);
    }}

    const tex = gl.createTexture();
    gl.activeTexture(gl.TEXTURE0);
    gl.bindTexture(gl.TEXTURE_2D, tex);
    gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, atlasCanvas);
    gl.generateMipmap(gl.TEXTURE_2D);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
    gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
    gl.uniform1i(uFont, 0);
}}

async function capture() {{
    const totalFrames = {FRAMES_PER_SCENE};
    const step = {step};
    for (let i = 0; i < totalFrames; i++) {{
        const t = {t_start} + i * step;
        gl.viewport(0, 0, 240, 426);
        gl.uniform2f(uRes, 240, 426);
        gl.uniform1f(uTime, t);
        gl.drawArrays(gl.TRIANGLES, 0, 6);

        const dataUrl = canvas.toDataURL('image/png');
        await fetch('/save?scene=' + scene + '&frame=' + i, {{
            method: 'POST',
            body: dataUrl
        }});
    }}
    await fetch('/done?scene=' + scene, {{ method: 'POST' }});
}}

window.addEventListener('load', () => setTimeout(capture, 150));
</script>
</body>
</html>"""
        self.send_response(200)
        self.send_header('Content-Type', 'text/html; charset=utf-8')
        self.end_headers()
        self.wfile.write(html.encode('utf-8'))

    def do_POST(self):
        parsed = urllib.parse.urlparse(self.path)
        params = urllib.parse.parse_qs(parsed.query)
        scene = params.get('scene', ['v1'])[0]

        if parsed.path == '/save':
            frame = int(params.get('frame', [0])[0])
            length = int(self.headers['Content-Length'])
            data = self.rfile.read(length).decode('utf-8')
            img_b64 = data.split(',')[1] if ',' in data else data
            img_bytes = base64.b64decode(img_b64)

            s_dir = os.path.join(FRAME_DIR, scene)
            os.makedirs(s_dir, exist_ok=True)
            with open(os.path.join(s_dir, f"frame_{frame:03d}.png"), "wb") as f:
                f.write(img_bytes)

            self.send_response(200)
            self.end_headers()
            self.wfile.write(b"OK")
        elif parsed.path == '/done':
            done_events[scene].set()
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b"DONE")

def run_server():
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(('127.0.0.1', PORT), GIFHandler) as httpd:
        httpd.serve_forever()

t = threading.Thread(target=run_server, daemon=True)
t.start()
print("Recorder server online.")

target_scenes = sys.argv[1:] if len(sys.argv) > 1 else ['v1', 'v2', 'v3', 'v4']
for scene in target_scenes:
    print(f"Recording {scene}...")
    chrome_cmd = [
        "google-chrome-stable",
        "--headless",
        "--disable-gpu",
        "--use-gl=angle",
        "--use-angle=swiftshader",
        "--window-size=240,426",
        f"http://127.0.0.1:{PORT}/?scene={scene}"
    ]
    proc = subprocess.Popen(chrome_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    
    # Wait for completion (max 20 seconds)
    success = done_events[scene].wait(timeout=20)
    proc.terminate()
    proc.wait()
    
    if not success:
        print(f"Timeout recording {scene}!")
        sys.exit(1)
        
    print(f"Frames recorded for {scene}. Encoding GIF...")
    s_dir = os.path.join(FRAME_DIR, scene)
    gif_out = os.path.join(OUTPUT_DIR, f"{scene}_mobile.gif")
    
    # High-quality 2-pass palettegen GIF encoding with ffmpeg
    ff_cmd = [
        "ffmpeg", "-y",
        "-framerate", "14",
        "-i", os.path.join(s_dir, "frame_%03d.png"),
        "-vf", "fps=14,scale=240:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128:reserve_transparent=0[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3",
        gif_out
    ]
    subprocess.check_call(ff_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    size_kb = os.path.getsize(gif_out) / 1024
    print(f"Generated {gif_out} ({size_kb:.1f} KB)")

print("All GIFs generated successfully!")
