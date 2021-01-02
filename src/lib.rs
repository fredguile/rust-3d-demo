extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use web_sys::WebGlRenderingContext as GL;

#[macro_use]
extern crate lazy_static;

mod app_state;
mod gl_setup;
mod helpers;
mod programs;
mod shaders;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct RustClient {
    gl: GL,
    _program_color_2d: programs::Color2D,
    _program_color_2d_gradient: programs::Color2DGradient,
}

#[wasm_bindgen]
impl RustClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        console_error_panic_hook::set_once();

        let gl = gl_setup::initialize_webgl_context().unwrap();

        Self {
            _program_color_2d: programs::Color2D::new(&gl),
            _program_color_2d_gradient: programs::Color2DGradient::new(&gl),
            gl,
        }
    }

    pub fn update(
        &mut self,
        time: f32,
        canvas_width: f32,
        canvas_height: f32,
    ) -> Result<(), JsValue> {
        app_state::update_dynamic_data(time, canvas_width, canvas_height);
        Ok(())
    }

    pub fn render(&self) {
        self.gl.clear(GL::COLOR_BUFFER_BIT | GL::DEPTH_BUFFER_BIT);

        let curr_state = app_state::get_curr_state();

        self._program_color_2d.render(
            &self.gl,
            curr_state.control_bottom,
            curr_state.control_top,
            curr_state.control_left,
            curr_state.control_right,
            curr_state.canvas_width,
            curr_state.canvas_height,
        );

        self._program_color_2d_gradient.render(
            &self.gl,
            curr_state.control_bottom + 20.,
            curr_state.control_top - 20.,
            curr_state.control_left + 20.,
            curr_state.control_right - 20.,
            curr_state.canvas_width,
            curr_state.canvas_height,
        );
    }
}
