import http.server
import socketserver
import subprocess
import threading
import urllib.parse
import base64
import os
import re
import sys
import json
import time

PORT = 8260
REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
FRAME_DIR = "/tmp/regenerate_frames"
OUTPUT_DIR = os.path.join(REPO_ROOT, "screenshots")

os.makedirs(FRAME_DIR, exist_ok=True)
os.makedirs(OUTPUT_DIR, exist_ok=True)

def extract_shader(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    m_fs = re.search(r'pub const FRAGMENT_SHADER_SOURCE:\s*&str\s*=\s*r#"([\s\S]+?)"#;', content)
    m_vs = re.search(r'pub const VERTEX_SHADER_SOURCE:\s*&str\s*=\s*r#"([\s\S]+?)"#;', content)
    if not m_fs:
        raise ValueError(f"Could not extract fragment shader from {filepath}")
    vs = m_vs.group(1).strip() if m_vs else "#version 300 es\nin vec2 a_position;\nvoid main() { gl_Position = vec4(a_position, 0.0, 1.0); }"
    return vs, m_fs.group(1).strip()

ANIMATIONS = {
    "celestial_heart": {
        "shader_path": os.path.join(REPO_ROOT, "src", "shaders_celestial_heart.rs"),
        "needs_font": False,
        "preview_t": 12.0,
        "gif_times": [6.0 + i * 0.18 for i in range(36)],
        "preview_name": "celestial_heart_preview.png",
        "gif_name": "celestial_heart_mobile.gif"
    },
    "matrix_vortex": {
        "shader_path": os.path.join(REPO_ROOT, "src", "shaders_matrix_vortex.rs"),
        "needs_font": True,
        "preview_t": 10.0,
        "gif_times": [8.0 + i * 0.12 for i in range(40)],
        "preview_name": "matrix_vortex_preview.png",
        "gif_name": "matrix_vortex_mobile.gif"
    },
    "vortex_void": {
        "shader_path": os.path.join(REPO_ROOT, "src", "shaders_vortex_void.rs"),
        "needs_font": False,
        "preview_t": 8.5,
        "gif_times": (
            [8.0 + i * 0.65 for i in range(14)] +
            [24.0 + i * 0.65 for i in range(14)] +
            [40.0 + i * 0.65 for i in range(14)]
        ),
        "preview_name": "vortex_void_preview.png",
        "gif_name": "vortex_void_mobile.gif"
    },
    "celestial_odyssey": {
        "shader_path": os.path.join(REPO_ROOT, "src", "shaders_celestial_odyssey.rs"),
        "needs_font": False,
        "preview_t": 28.0,
        "gif_times": (
            [8.0 + i * 0.25 for i in range(14)] +
            [26.0 + i * 0.25 for i in range(14)] +
            [50.0 + i * 0.25 for i in range(14)]
        ),
        "preview_name": "celestial_odyssey_preview.png",
        "gif_name": "celestial_odyssey_mobile.gif"
    },
    "matrix_saga": {
        "shader_path": os.path.join(REPO_ROOT, "src", "shaders_matrix_saga.rs"),
        "needs_font": True,
        "preview_t": 7.5,
        "gif_times": (
            [7.0 + i * 0.14 for i in range(14)] +
            [24.0 + i * 0.14 for i in range(14)] +
            [48.0 + i * 0.16 for i in range(14)]
        ),
        "preview_name": "matrix_saga_preview.png",
        "gif_name": "matrix_saga_mobile.gif"
    },
    "cosmic_infinity": {
        "shader_path": os.path.join(REPO_ROOT, "src", "shaders_cosmic_mandelbrot.rs"),
        "needs_font": False,
        "preview_t": 21.0,
        "gif_times": [14.0 + (i / 40.0) * 18.0 for i in range(40)],
        "preview_name": "cosmic_infinity_preview.png",
        "gif_name": "cosmic_infinity_mobile.gif"
    },
    "vortex_metamorphosis": {
        "shader_path": os.path.join(REPO_ROOT, "src", "shaders_vortex_metamorphosis.rs"),
        "needs_font": False,
        "preview_t": 15.0,
        "gif_times": [8.0 + i * 0.35 for i in range(40)],
        "preview_name": "vortex_metamorphosis_preview.png",
        "gif_name": "vortex_metamorphosis_mobile.gif"
    }
}

current_anim_key = None
current_vs = ""
current_fs = ""
current_needs_font = False
current_preview_t = 0.0
current_gif_times = []

done_event = threading.Event()
preview_done = threading.Event()

class Handler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        vs_json = json.dumps(current_vs)
        fs_json = json.dumps(current_fs)

        font_setup = ""
        if current_needs_font:
            font_setup = """
            const atlasCanvas = document.createElement('canvas');
            atlasCanvas.width = 1024; atlasCanvas.height = 1024;
            const ctx = atlasCanvas.getContext('2d');
            ctx.fillStyle = '#000000'; ctx.fillRect(0, 0, 1024, 1024);
            const katakana = ['ア','イ','ウ','エ','オ','カ','キ','ク','ケ','コ','サ','シ','ス','セ','ソ','タ','チ','ツ','テ','ト','ナ','ニ','ヌ','ネ','ノ','ハ','ヒ','フ','ヘ','ホ','マ','ミ','ム','メ','モ','ヤ','ユ','ヨ','ラ','リ','ル','レ','ロ','ワ','ヲ','ン'];
            const kanji = ['愛','心','命','夢','魂','恋','龍','光','星','零','壱','幻','闇','炎','雷','鏡','永','破','絆','宙','響','創','神','無'];
            const numbers = ['0','1','2','3','4','5','6','7','8','9'];
            const letters = ['V','A','L','I','K','P','E','T','O','ヴ','ァ','リ','カ','ペ','ト','テ','ッ','ク','コ'];
            const banners = { 200: 'VALIKA', 201: 'PETO', 202: 'VALI', 203: 'TEKK', 204: 'TEKKO', 205: 'ヴァリカ', 206: 'ペト', 207: 'テック', 208: 'テッコ', 209: 'VALIKA', 210: 'TEKK', 211: 'PETO', 212: 'VALI' };
            const cols = 16; const cellW = 1024 / cols; const cellH = 1024 / cols;
            ctx.textAlign = 'center'; ctx.textBaseline = 'middle'; ctx.fillStyle = '#ffffff';
            function drawCell(idx, text, fontSize) {
                const c = idx % cols; const r = Math.floor(idx / cols);
                const x = c * cellW + cellW / 2; const y = r * cellH + cellH / 2;
                ctx.font = 'bold ' + fontSize + 'px "Courier New", monospace, sans-serif';
                ctx.fillText(text, x, y, cellW * 0.9);
            }
            katakana.forEach((ch, i) => drawCell(i, ch, 40));
            kanji.forEach((ch, i) => drawCell(46 + i, ch, 40));
            for (let i = 0; i < 40; i++) drawCell(70 + i, numbers[i % 10], 40);
            letters.forEach((ch, i) => drawCell(110 + i, ch, 40));
            for (let i = 0; i < 20; i++) drawCell(130 + i, numbers[(i * 3 + 1) % 10], 40);
            for (const [idxStr, text] of Object.entries(banners)) {
                const idx = parseInt(idxStr, 10);
                const size = text.length > 5 ? 13 : (text.length > 3 ? 16 : 22);
                ctx.fillStyle = 'rgba(255, 255, 255, 0.75)';
                drawCell(idx, text, size);
                ctx.fillStyle = '#ffffff';
            }
            const tex = gl.createTexture();
            gl.activeTexture(gl.TEXTURE0); gl.bindTexture(gl.TEXTURE_2D, tex);
            gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, atlasCanvas);
            gl.generateMipmap(gl.TEXTURE_2D);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
            gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
            const uFont = gl.getUniformLocation(prog, 'u_fontTexture');
            if (uFont) gl.uniform1i(uFont, 0);
            """

        if parsed.path == '/preview':
            html = f"""<!DOCTYPE html>
<html><head><meta charset="utf-8"><style>body,html{{margin:0;overflow:hidden;background:#000;}}canvas{{display:block;width:1280px;height:720px;}}</style></head>
<body><canvas id="c" width="1280" height="720"></canvas>
<script>
const vsSource = {vs_json};
const fsSource = {fs_json};
const canvas = document.getElementById('c');
const gl = canvas.getContext('webgl2', {{ alpha: false, preserveDrawingBuffer: true }});
const vs = gl.createShader(gl.VERTEX_SHADER); gl.shaderSource(vs, vsSource); gl.compileShader(vs);
const fs = gl.createShader(gl.FRAGMENT_SHADER); gl.shaderSource(fs, fsSource); gl.compileShader(fs);
const prog = gl.createProgram(); gl.attachShader(prog, vs); gl.attachShader(prog, fs); gl.linkProgram(prog); gl.useProgram(prog);

const buf = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, buf);
gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1,-1, 1,-1, -1,1, -1,1, 1,-1, 1,1]), gl.STATIC_DRAW);
const pos = gl.getAttribLocation(prog, 'a_position') !== -1 ? gl.getAttribLocation(prog, 'a_position') : gl.getAttribLocation(prog, 'a_pos');
if (pos >= 0) {{ gl.enableVertexAttribArray(pos); gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0); }}
{font_setup}

const uRes = gl.getUniformLocation(prog, 'u_resolution');
const uTime = gl.getUniformLocation(prog, 'u_time');
gl.viewport(0, 0, 1280, 720);
gl.uniform2f(uRes, 1280, 720);
gl.uniform1f(uTime, {current_preview_t});
gl.drawArrays(gl.TRIANGLES, 0, 6);

const data = canvas.toDataURL('image/png');
fetch('/save_preview', {{ method: 'POST', body: data }});
</script></body></html>"""
            self.send_response(200); self.send_header('Content-Type', 'text/html; charset=utf-8'); self.end_headers()
            self.wfile.write(html.encode('utf-8'))
        elif parsed.path == '/gif':
            times_json = json.dumps(current_gif_times)
            html = f"""<!DOCTYPE html>
<html><head><meta charset="utf-8"><style>body,html{{margin:0;overflow:hidden;background:#000;}}canvas{{display:block;width:240px;height:426px;}}</style></head>
<body><canvas id="c" width="240" height="426"></canvas>
<script>
const vsSource = {vs_json};
const fsSource = {fs_json};
const canvas = document.getElementById('c');
const gl = canvas.getContext('webgl2', {{ alpha: false, preserveDrawingBuffer: true }});
const vs = gl.createShader(gl.VERTEX_SHADER); gl.shaderSource(vs, vsSource); gl.compileShader(vs);
const fs = gl.createShader(gl.FRAGMENT_SHADER); gl.shaderSource(fs, fsSource); gl.compileShader(fs);
const prog = gl.createProgram(); gl.attachShader(prog, vs); gl.attachShader(prog, fs); gl.linkProgram(prog); gl.useProgram(prog);

const buf = gl.createBuffer(); gl.bindBuffer(gl.ARRAY_BUFFER, buf);
gl.bufferData(gl.ARRAY_BUFFER, new Float32Array([-1,-1, 1,-1, -1,1, -1,1, 1,-1, 1,1]), gl.STATIC_DRAW);
const pos = gl.getAttribLocation(prog, 'a_position') !== -1 ? gl.getAttribLocation(prog, 'a_position') : gl.getAttribLocation(prog, 'a_pos');
if (pos >= 0) {{ gl.enableVertexAttribArray(pos); gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0); }}
{font_setup}

const uRes = gl.getUniformLocation(prog, 'u_resolution');
const uTime = gl.getUniformLocation(prog, 'u_time');
gl.viewport(0, 0, 240, 426);
gl.uniform2f(uRes, 240, 426);

const frameTimes = {times_json};

async function captureFrames() {{
    for (let i = 0; i < frameTimes.length; i++) {{
        const t = frameTimes[i];
        gl.uniform1f(uTime, t);
        gl.drawArrays(gl.TRIANGLES, 0, 6);
        const dataUrl = canvas.toDataURL('image/png');
        await fetch('/save_frame?frame=' + i, {{ method: 'POST', body: dataUrl }});
    }}
    await fetch('/done', {{ method: 'POST' }});
}}
window.addEventListener('load', () => setTimeout(captureFrames, 100));
</script></body></html>"""
            self.send_response(200); self.send_header('Content-Type', 'text/html; charset=utf-8'); self.end_headers()
            self.wfile.write(html.encode('utf-8'))

    def do_POST(self):
        parsed = urllib.parse.urlparse(self.path)
        length = int(self.headers['Content-Length'])
        data = self.rfile.read(length).decode('utf-8')
        img_b64 = data.split(',')[1] if ',' in data else data
        img_bytes = base64.b64decode(img_b64)

        if parsed.path == '/save_preview':
            prev_file = os.path.join(OUTPUT_DIR, ANIMATIONS[current_anim_key]["preview_name"])
            with open(prev_file, "wb") as f:
                f.write(img_bytes)
            preview_done.set()
        elif parsed.path == '/save_frame':
            params = urllib.parse.parse_qs(parsed.query)
            frame = int(params.get('frame', [0])[0])
            with open(os.path.join(FRAME_DIR, f"frame_{frame:03d}.png"), "wb") as f:
                f.write(img_bytes)
        elif parsed.path == '/done':
            done_event.set()

        self.send_response(200); self.end_headers(); self.wfile.write(b"OK")

def start_server():
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(('127.0.0.1', PORT), Handler) as httpd:
        httpd.serve_forever()

threading.Thread(target=start_server, daemon=True).start()

def process_animation(key):
    global current_anim_key, current_vs, current_fs, current_needs_font, current_preview_t, current_gif_times
    cfg = ANIMATIONS[key]
    current_anim_key = key
    current_vs, current_fs = extract_shader(cfg["shader_path"])
    current_needs_font = cfg["needs_font"]
    current_preview_t = cfg["preview_t"]
    current_gif_times = cfg["gif_times"]

    print(f"\n==================================================")
    print(f"Processing: {key}")
    print(f"==================================================")

    # 1. Generate landscape preview PNG (1280x720)
    print(f"Generating landscape preview: {cfg['preview_name']} at t={current_preview_t}...")
    preview_done.clear()
    chrome_cmd = [
        "chromium", "--headless=new", "--no-sandbox",
        "--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader",
        f"--user-data-dir=/tmp/c_prev_{key}",
        "--window-size=1280,720",
        f"http://127.0.0.1:{PORT}/preview"
    ]
    proc = subprocess.Popen(chrome_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if not preview_done.wait(timeout=25):
        print(f"ERROR: Preview generation timed out for {key}!")
    proc.terminate()
    try: proc.wait(timeout=2)
    except: proc.kill()

    prev_path = os.path.join(OUTPUT_DIR, cfg["preview_name"])
    if os.path.exists(prev_path):
        size_kb = os.path.getsize(prev_path) / 1024
        print(f"Created {prev_path} ({size_kb:.1f} KB)")
    else:
        print(f"FAILED to create {prev_path}")

    # 2. Generate mobile animated GIF (240x426)
    print(f"Generating mobile animated GIF: {cfg['gif_name']} ({len(current_gif_times)} frames)...")
    for f in os.listdir(FRAME_DIR):
        os.remove(os.path.join(FRAME_DIR, f))

    done_event.clear()
    chrome_cmd = [
        "chromium", "--headless=new", "--no-sandbox",
        "--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader",
        f"--user-data-dir=/tmp/c_gif_{key}",
        "--window-size=240,426",
        f"http://127.0.0.1:{PORT}/gif"
    ]
    proc = subprocess.Popen(chrome_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if not done_event.wait(timeout=40):
        print(f"ERROR: Frame capture timed out for {key}!")
    proc.terminate()
    try: proc.wait(timeout=2)
    except: proc.kill()

    gif_path = os.path.join(OUTPUT_DIR, cfg["gif_name"])
    ff_cmd = [
        "ffmpeg", "-y",
        "-framerate", "14",
        "-i", os.path.join(FRAME_DIR, "frame_%03d.png"),
        "-vf", "fps=14,scale=240:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128:reserve_transparent=0[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3",
        gif_path
    ]
    subprocess.check_call(ff_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if os.path.exists(gif_path):
        size_kb = os.path.getsize(gif_path) / 1024
        print(f"Created {gif_path} ({size_kb:.1f} KB)")
    else:
        print(f"FAILED to create {gif_path}")

for key in ANIMATIONS.keys():
    process_animation(key)

print("\nAll 7 animation previews and animated GIFs generated successfully!")
