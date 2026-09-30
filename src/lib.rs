use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
    HtmlCanvasElement, Performance, WebGl2RenderingContext, WebGlBuffer, WebGlProgram,
    WebGlShader, WebGlUniformLocation,
};

mod shaders;

#[wasm_bindgen]
pub struct FractalApp {
    gl: WebGl2RenderingContext,
    program: WebGlProgram,
    _buffer: WebGlBuffer,
    u_resolution: WebGlUniformLocation,
    u_time: WebGlUniformLocation,
    canvas: HtmlCanvasElement,
    performance: Performance,
    dimensions: Rc<Cell<(i32, i32)>>,
}

impl FractalApp {
    pub fn update_viewport(&self) {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };

        // Mobile optimization: Cap devicePixelRatio to 1.5 to guarantee smooth 60-120 FPS
        // on high-DPI AMOLED screens (like Samsung Galaxy S22 with DPR 3.0)
        let raw_dpr = window.device_pixel_ratio();
        let dpr = raw_dpr.min(1.5).max(1.0);

        let client_width = self.canvas.client_width() as f64;
        let client_height = self.canvas.client_height() as f64;

        let target_width = (client_width * dpr).round() as u32;
        let target_height = (client_height * dpr).round() as u32;

        if target_width > 0 && target_height > 0 {
            self.canvas.set_width(target_width);
            self.canvas.set_height(target_height);
            self.gl
                .viewport(0, 0, target_width as i32, target_height as i32);
            self.dimensions
                .set((target_width as i32, target_height as i32));
        }
    }

    pub fn render_frame(&self) {
        let (width, height) = self.dimensions.get();

        self.gl.use_program(Some(&self.program));

        let current_time_sec = (self.performance.now() * 0.001) as f32;

        self.gl
            .uniform2f(Some(&self.u_resolution), width as f32, height as f32);
        self.gl.uniform1f(Some(&self.u_time), current_time_sec);

        self.gl
            .draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
    }
}

#[wasm_bindgen]
impl FractalApp {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<FractalApp, JsValue> {
        console_error_panic_hook::set_once();

        let window = web_sys::window().ok_or("No global `window` exists")?;
        let document = window.document().ok_or("No `document` on window")?;
        let performance = window
            .performance()
            .ok_or("No `performance` object found on window")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| format!("Canvas with id '{}' not found", canvas_id))?
            .dyn_into::<HtmlCanvasElement>()?;

        // Context options tuned for maximum performance on mobile (Samsung Galaxy S22)
        let context_options = js_sys::Object::new();
        js_sys::Reflect::set(&context_options, &"alpha".into(), &false.into())?;
        js_sys::Reflect::set(&context_options, &"depth".into(), &false.into())?;
        js_sys::Reflect::set(&context_options, &"stencil".into(), &false.into())?;
        js_sys::Reflect::set(&context_options, &"antialias".into(), &false.into())?;
        js_sys::Reflect::set(
            &context_options,
            &"powerPreference".into(),
            &"high-performance".into(),
        )?;
        js_sys::Reflect::set(
            &context_options,
            &"preserveDrawingBuffer".into(),
            &false.into(),
        )?;

        let gl = canvas
            .get_context_with_context_options("webgl2", &context_options)?
            .ok_or("WebGL 2.0 is not supported on this device/browser")?
            .dyn_into::<WebGl2RenderingContext>()?;

        let vert_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::VERTEX_SHADER,
            shaders::VERTEX_SHADER_SOURCE,
        )?;
        let frag_shader = compile_shader(
            &gl,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            shaders::FRAGMENT_SHADER_SOURCE,
        )?;
        let program = link_program(&gl, &vert_shader, &frag_shader)?;

        gl.use_program(Some(&program));

        // Fullscreen quad covering clip-space [-1.0, 1.0]
        let vertices: [f32; 12] = [
            -1.0, -1.0, 1.0, -1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 1.0,
        ];

        let buffer = gl.create_buffer().ok_or("Failed to create WebGL buffer")?;
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        let a_position = gl.get_attrib_location(&program, "a_position");
        if a_position >= 0 {
            gl.enable_vertex_attrib_array(a_position as u32);
            gl.vertex_attrib_pointer_with_i32(
                a_position as u32,
                2,
                WebGl2RenderingContext::FLOAT,
                false,
                0,
                0,
            );
        }

        let u_resolution = gl
            .get_uniform_location(&program, "u_resolution")
            .ok_or("Uniform `u_resolution` not found")?;
        let u_time = gl
            .get_uniform_location(&program, "u_time")
            .ok_or("Uniform `u_time` not found")?;

        let dimensions = Rc::new(Cell::new((canvas.width() as i32, canvas.height() as i32)));

        let app = FractalApp {
            gl,
            program,
            _buffer: buffer,
            u_resolution,
            u_time,
            canvas,
            performance,
            dimensions,
        };

        // Initialize viewport dimensions
        app.update_viewport();

        Ok(app)
    }

    pub fn resize(&self) {
        self.update_viewport();
    }

    pub fn draw(&self) {
        self.render_frame();
    }
}

#[wasm_bindgen]
pub fn start_fractal(canvas_id: &str) -> Result<(), JsValue> {
    let app = Rc::new(FractalApp::new(canvas_id)?);

    // Initial sizing
    app.update_viewport();
    app.render_frame();

    let window = web_sys::window().ok_or("No window found")?;

    // Listen to resize events on window to avoid querying DOM dimensions on every frame
    {
        let app_resize = app.clone();
        let resize_closure = Closure::wrap(Box::new(move || {
            app_resize.update_viewport();
        }) as Box<dyn FnMut()>);

        window.add_event_listener_with_callback(
            "resize",
            resize_closure.as_ref().unchecked_ref(),
        )?;
        window.add_event_listener_with_callback(
            "orientationchange",
            resize_closure.as_ref().unchecked_ref(),
        )?;
        resize_closure.forget(); // Keep listener alive for app lifetime
    }

    // Zero-overhead requestAnimationFrame render loop
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();

    let app_clone = app.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        app_clone.render_frame();

        if let Some(win) = web_sys::window() {
            if let Some(ref callback) = *f.borrow() {
                let _ = win.request_animation_frame(callback.as_ref().unchecked_ref());
            }
        }
    }) as Box<dyn FnMut()>));

    window.request_animation_frame(
        g.borrow()
            .as_ref()
            .ok_or("Closure not initialized")?
            .as_ref()
            .unchecked_ref(),
    )?;

    Ok(())
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        let info = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error compiling shader"));
        gl.delete_shader(Some(&shader));
        Err(info)
    }
}

fn link_program(
    gl: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader program"))?;

    gl.attach_shader(&program, vert_shader);
    gl.attach_shader(&program, frag_shader);
    gl.link_program(&program);

    if gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        let info = gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error linking program"));
        gl.delete_program(Some(&program));
        Err(info)
    }
}
