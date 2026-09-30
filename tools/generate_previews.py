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

PORT = 8225
OUTPUT_DIR = "/home/tekk/dev/wasm-infinite-gpu-fractal/screenshots"
os.makedirs(OUTPUT_DIR, exist_ok=True)

def extract_fragment_shader(filepath):
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    m = re.search(r'pub const FRAGMENT_SHADER_SOURCE:\s*&str\s*=\s*r#"([\s\S]+?)"#;', content)
    if not m:
        raise ValueError(f"Could not extract fragment shader from {filepath}")
    return m.group(1).strip()

vortex_fs = extract_fragment_shader("/home/tekk/dev/wasm-infinite-gpu-fractal/src/shaders_matrix_vortex.rs")
saga_fs = extract_fragment_shader("/home/tekk/dev/wasm-infinite-gpu-fractal/src/shaders_matrix_saga.rs")

# Shared server state
current_task = None
done_event = threading.Event()
frame_dir = None

class PreviewHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        if parsed.path == '/':
            fs_code = vortex_fs if current_task == 'vortex' else saga_fs
            fs_js = fs_code.replace('\\', '\\\\').replace('`', '\\`').replace('$', '\\$')
            
            # Script to run depending on task
            if current_task == 'vortex':
                # 42 frames: continuous cylindrical vortex flight
                frame_script = """
                const totalFrames = 42;
                const t_start = 8.0;
                const step = 0.08;
                for (let i = 0; i < totalFrames; i++) {
                    const t = t_start + i * step;
                    gl.viewport(0, 0, 240, 426);
                    gl.uniform2f(uRes, 240, 426);
                    gl.uniform1f(uTime, t);
                    gl.drawArrays(gl.TRIANGLES, 0, 6);
                    const dataUrl = canvas.toDataURL('image/png');
                    await fetch('/save?frame=' + i, { method: 'POST', body: dataUrl });
                }
                await fetch('/done', { method: 'POST' });
                """
            else:
                # 42 frames for saga: 14 frames Gateway (t=7.0), 14 frames Wormhole (t=24.0), 14 frames Plexus (t=48.0)
                frame_script = """
                const frames = [];
                for (let i = 0; i < 14; i++) frames.push(7.0 + i * 0.12);
                for (let i = 0; i < 14; i++) frames.push(24.0 + i * 0.12);
                for (let i = 0; i < 14; i++) frames.push(48.0 + i * 0.15);

                for (let i = 0; i < frames.length; i++) {
                    const t = frames[i];
                    gl.viewport(0, 0, 240, 426);
                    gl.uniform2f(uRes, 240, 426);
                    gl.uniform1f(uTime, t);
                    gl.drawArrays(gl.TRIANGLES, 0, 6);
                    const dataUrl = canvas.toDataURL('image/png');
                    await fetch('/save?frame=' + i, { method: 'POST', body: dataUrl });
                }
                await fetch('/done', { method: 'POST' });
                """

            html = f"""<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><style>body,html{{margin:0;padding:0;overflow:hidden;background:#000;}}canvas{{display:block;width:240px;height:426px;}}</style></head>
<body>
<canvas id="c" width="240" height="426"></canvas>
<script>
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

// Generate Font Atlas
const atlasCanvas = document.createElement('canvas');
atlasCanvas.width = 1024;
atlasCanvas.height = 1024;
const ctx = atlasCanvas.getContext('2d');
ctx.fillStyle = '#000000';
ctx.fillRect(0, 0, 1024, 1024);

const katakana = ['ア','イ','ウ','エ','オ','カ','キ','ク','ケ','コ','サ','シ','ス','セ','ソ','タ','チ','ツ','テ','ト','ナ','ニ','ヌ','ネ','ノ','ハ','ヒ','フ','ヘ','ホ','マ','ミ','ム','メ','モ','ヤ','ユ','ヨ','ラ','リ','ル','レ','ロ','ワ','ヲ','ン'];
const kanji = ['愛','心','命','夢','魂','恋','龍','光','星','零','壱','幻','闇','炎','雷','鏡','永','破','絆','宙','響','創','神','無'];
const numbers = ['0','1','2','3','4','5','6','7','8','9'];
const letters = ['V','A','L','I','K','P','E','T','O','ヴ','ァ','リ','カ','ペ','ト','テ','ッ','ク','コ'];
const banners = {{
    200: 'VALIKA', 201: 'PETO', 202: 'VALI', 203: 'TEKK', 204: 'TEKKO',
    205: 'ヴァリカ', 206: 'ペト', 207: 'テック', 208: 'テッコ',
    209: 'VALIKA', 210: 'TEKK', 211: 'PETO', 212: 'VALI'
}};

const cols = 16;
const cellW = 1024 / cols;
const cellH = 1024 / cols;
ctx.textAlign = 'center';
ctx.textBaseline = 'middle';
ctx.fillStyle = '#ffffff';

function drawCell(idx, text, fontSize) {{
    const c = idx % cols;
    const r = Math.floor(idx / cols);
    const x = c * cellW + cellW / 2;
    const y = r * cellH + cellH / 2;
    ctx.font = 'bold ' + fontSize + 'px "Courier New", monospace, sans-serif';
    ctx.fillText(text, x, y, cellW * 0.9);
}}

katakana.forEach((ch, i) => drawCell(i, ch, 40));
kanji.forEach((ch, i) => drawCell(46 + i, ch, 40));
for (let i = 0; i < 40; i++) drawCell(70 + i, numbers[i % 10], 40);
letters.forEach((ch, i) => drawCell(110 + i, ch, 40));
for (let i = 0; i < 20; i++) drawCell(130 + i, numbers[(i * 3 + 1) % 10], 40);

for (const [idxStr, text] of Object.entries(banners)) {{
    const idx = parseInt(idxStr, 10);
    const size = text.length > 5 ? 13 : (text.length > 3 ? 16 : 22);
    ctx.fillStyle = 'rgba(255, 255, 255, 0.75)';
    drawCell(idx, text, size);
    ctx.fillStyle = '#ffffff';
}}

const tex = gl.createTexture();
gl.activeTexture(gl.TEXTURE0);
gl.bindTexture(gl.TEXTURE_2D, tex);
gl.texImage2D(gl.TEXTURE_2D, 0, gl.RGBA, gl.RGBA, gl.UNSIGNED_BYTE, atlasCanvas);
gl.generateMipmap(gl.TEXTURE_2D);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR_MIPMAP_LINEAR);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
gl.uniform1i(uFont, 0);

async function capture() {{
    {frame_script}
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

        if parsed.path == '/save':
            frame = int(params.get('frame', [0])[0])
            length = int(self.headers['Content-Length'])
            data = self.rfile.read(length).decode('utf-8')
            img_b64 = data.split(',')[1] if ',' in data else data
            img_bytes = base64.b64decode(img_b64)

            with open(os.path.join(frame_dir, f"frame_{frame:03d}.png"), "wb") as f:
                f.write(img_bytes)

            self.send_response(200)
            self.end_headers()
            self.wfile.write(b"OK")
        elif parsed.path == '/done':
            done_event.set()
            self.send_response(200)
            self.end_headers()
            self.wfile.write(b"DONE")

def run_server():
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(('127.0.0.1', PORT), PreviewHandler) as httpd:
        httpd.serve_forever()

server_thread = threading.Thread(target=run_server, daemon=True)
server_thread.start()

def record_gif(task_name, out_filename):
    global current_task, done_event, frame_dir
    current_task = task_name
    done_event.clear()
    frame_dir = f"/tmp/{task_name}_frames"
    os.makedirs(frame_dir, exist_ok=True)
    for f in os.listdir(frame_dir):
        os.remove(os.path.join(frame_dir, f))

    chrome_cmd = [
        "chromium",
        "--headless=new",
        "--no-sandbox",
        "--use-gl=angle",
        "--use-angle=swiftshader",
        "--enable-unsafe-swiftshader",
        "--user-data-dir=" + f"/tmp/chrome_p_{task_name}",
        "--window-size=240,426",
        f"http://127.0.0.1:{PORT}/"
    ]
    proc = subprocess.Popen(chrome_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    success = done_event.wait(timeout=25)
    proc.terminate()
    try:
        proc.wait(timeout=3)
    except:
        proc.kill()

    if not success:
        print(f"Failed to record {task_name} frames!")
        return False

    gif_out = os.path.join(OUTPUT_DIR, out_filename)
    ff_cmd = [
        "ffmpeg", "-y",
        "-framerate", "14",
        "-i", os.path.join(frame_dir, "frame_%03d.png"),
        "-vf", "fps=14,scale=240:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128:reserve_transparent=0[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3",
        gif_out
    ]
    subprocess.check_call(ff_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    size_kb = os.path.getsize(gif_out) / 1024
    print(f"Generated {gif_out} ({size_kb:.1f} KB)")
    return True

print("=== Recording matrix_vortex_mobile.gif ===")
record_gif("vortex", "matrix_vortex_mobile.gif")

print("=== Recording matrix_saga_mobile.gif ===")
record_gif("saga", "matrix_saga_mobile.gif")

print("All GIFs generated successfully!")
