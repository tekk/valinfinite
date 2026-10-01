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

PORT = 8211
REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
FRAME_DIR = "/tmp/vortex_void_frames"
OUTPUT_DIR = os.path.join(REPO_ROOT, "screenshots")

os.makedirs(FRAME_DIR, exist_ok=True)
os.makedirs(OUTPUT_DIR, exist_ok=True)

with open(os.path.join(REPO_ROOT, "src", "shaders_vortex_void.rs"), "r") as f:
    src = f.read()

m_fs = re.search(r'pub const FRAGMENT_SHADER_SOURCE:\s*&str\s*=\s*r#"([\s\S]+?)"#;', src).group(1).strip()
m_vs = re.search(r'pub const VERTEX_SHADER_SOURCE:\s*&str\s*=\s*r#"([\s\S]+?)"#;', src).group(1).strip()

fs_json = json.dumps(m_fs)
vs_json = json.dumps(m_vs)

done_event = threading.Event()
preview_done = threading.Event()

class Handler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, format, *args):
        pass

    def do_GET(self):
        parsed = urllib.parse.urlparse(self.path)
        if parsed.path == '/preview':
            html = f"""<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><style>body,html{{margin:0;padding:0;overflow:hidden;background:#000;}}canvas{{display:block;width:1280px;height:720px;}}</style></head>
<body>
<canvas id="c" width="1280" height="720"></canvas>
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
const pos = gl.getAttribLocation(prog, 'a_position');
if (pos >= 0) {{ gl.enableVertexAttribArray(pos); gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0); }}

const uRes = gl.getUniformLocation(prog, 'u_resolution');
const uTime = gl.getUniformLocation(prog, 'u_time');
gl.viewport(0, 0, 1280, 720);
gl.uniform2f(uRes, 1280, 720);
gl.uniform1f(uTime, 8.5); // Singularity with glowing accretion disk & photon sphere
gl.drawArrays(gl.TRIANGLES, 0, 6);

const data = canvas.toDataURL('image/png');
fetch('/save_preview', {{ method: 'POST', body: data }});
</script></body></html>"""
            self.send_response(200)
            self.send_header('Content-Type', 'text/html; charset=utf-8')
            self.end_headers()
            self.wfile.write(html.encode('utf-8'))
        elif parsed.path == '/gif':
            html = f"""<!DOCTYPE html>
<html>
<head><meta charset="utf-8"><style>body,html{{margin:0;padding:0;overflow:hidden;background:#000;}}canvas{{display:block;width:240px;height:426px;}}</style></head>
<body>
<canvas id="c" width="240" height="426"></canvas>
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
const pos = gl.getAttribLocation(prog, 'a_position');
if (pos >= 0) {{ gl.enableVertexAttribArray(pos); gl.vertexAttribPointer(pos, 2, gl.FLOAT, false, 0, 0); }}

const uRes = gl.getUniformLocation(prog, 'u_resolution');
const uTime = gl.getUniformLocation(prog, 'u_time');
gl.viewport(0, 0, 240, 426);
gl.uniform2f(uRes, 240, 426);

// Multi-scene showcase with smooth transitions:
// 12 frames Scene 1 (Singularity) -> Transition 1->2 -> 12 frames Scene 2 (Tesseract) -> 12 frames Scene 3 (Hex Conduit) -> 12 frames Scene 4 (Gyroscope)
const frameTimes = [];
// Scene 1 & Transition 1->2 (t = 8.0 to 17.5)
for (let i = 0; i < 15; i++) frameTimes.push(8.0 + i * 0.65);
// Scene 2 & Transition 2->3 (t = 24.0 to 33.5)
for (let i = 0; i < 15; i++) frameTimes.push(24.0 + i * 0.65);
// Scene 3 & Transition 3->4 (t = 40.0 to 49.5)
for (let i = 0; i < 15; i++) frameTimes.push(40.0 + i * 0.65);

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
            self.send_response(200)
            self.send_header('Content-Type', 'text/html; charset=utf-8')
            self.end_headers()
            self.wfile.write(html.encode('utf-8'))

    def do_POST(self):
        parsed = urllib.parse.urlparse(self.path)
        length = int(self.headers['Content-Length'])
        data = self.rfile.read(length).decode('utf-8')
        img_b64 = data.split(',')[1] if ',' in data else data
        img_bytes = base64.b64decode(img_b64)

        if parsed.path == '/save_preview':
            with open(os.path.join(OUTPUT_DIR, "vortex_void_preview.png"), "wb") as f:
                f.write(img_bytes)
            preview_done.set()
        elif parsed.path == '/save_frame':
            params = urllib.parse.parse_qs(parsed.query)
            frame = int(params.get('frame', [0])[0])
            with open(os.path.join(FRAME_DIR, f"frame_{frame:03d}.png"), "wb") as f:
                f.write(img_bytes)
        elif parsed.path == '/done':
            done_event.set()

        self.send_response(200)
        self.end_headers()
        self.wfile.write(b"OK")

def run_server():
    socketserver.TCPServer.allow_reuse_address = True
    with socketserver.TCPServer(('127.0.0.1', PORT), Handler) as httpd:
        httpd.serve_forever()

threading.Thread(target=run_server, daemon=True).start()

print("Generating vortex_void_preview.png...")
chrome_cmd = [
    "chromium", "--headless=new", "--no-sandbox",
    "--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader",
    "--user-data-dir=/tmp/c_vv_prev",
    "--window-size=1280,720",
    f"http://127.0.0.1:{PORT}/preview"
]
proc = subprocess.Popen(chrome_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
preview_done.wait(timeout=20)
proc.terminate()
try: proc.wait(timeout=2)
except: proc.kill()

print("Generating vortex_void_mobile.gif...")
for f in os.listdir(FRAME_DIR):
    os.remove(os.path.join(FRAME_DIR, f))

chrome_cmd = [
    "chromium", "--headless=new", "--no-sandbox",
    "--use-gl=angle", "--use-angle=swiftshader", "--enable-unsafe-swiftshader",
    "--user-data-dir=/tmp/c_vv_gif",
    "--window-size=240,426",
    f"http://127.0.0.1:{PORT}/gif"
]
proc = subprocess.Popen(chrome_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
done_event.wait(timeout=35)
proc.terminate()
try: proc.wait(timeout=2)
except: proc.kill()

gif_out = os.path.join(OUTPUT_DIR, "vortex_void_mobile.gif")
ff_cmd = [
    "ffmpeg", "-y",
    "-framerate", "14",
    "-i", os.path.join(FRAME_DIR, "frame_%03d.png"),
    "-vf", "fps=14,scale=240:-1:flags=lanczos,split[s0][s1];[s0]palettegen=max_colors=128:reserve_transparent=0[p];[s1][p]paletteuse=dither=bayer:bayer_scale=3",
    gif_out
]
subprocess.check_call(ff_cmd, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
size_kb = os.path.getsize(gif_out) / 1024
print(f"Generated {gif_out} ({size_kb:.1f} KB)")
print("Done! Previews generated successfully.")
