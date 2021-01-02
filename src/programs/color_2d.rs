use super::super::{helpers, shaders};
use js_sys::Float32Array;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlUniformLocation};

pub struct Color2D {
    program: WebGlProgram,
    rect_vertice_len: usize,
    vertices_buffer: WebGlBuffer,
    u_color: WebGlUniformLocation,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2D {
    pub fn new(gl: &GL) -> Self {
        let program = helpers::link_program(
            &gl,
            shaders::vertex::color_2d::SHADER,
            shaders::fragment::color_2d::SHADER,
        )
        .unwrap();

        let vertices_rect: [f32; 12] = [
            0., 1., // x, y
            0., 0., // x, y
            1., 1., // x, y
            1., 1., // x, y
            0., 0., // x, y
            1., 0., // x, y
        ];

        let vertices_array = helpers::create_typed_array::<Float32Array>(
            helpers::create_memory_buffer().unwrap(),
            vertices_rect.as_ptr() as u32 / 4,
            vertices_rect.len(),
        );

        let vertices_buffer = gl.create_buffer().ok_or("Failed to create buffer").unwrap();
        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vertices_buffer));
        gl.buffer_data_with_array_buffer_view(GL::ARRAY_BUFFER, &vertices_array, GL::STATIC_DRAW);

        Self {
            rect_vertice_len: vertices_rect.len(),
            vertices_buffer: vertices_buffer,
            u_color: gl.get_uniform_location(&program, "uColor").unwrap(),
            u_opacity: gl.get_uniform_location(&program, "uOpacity").unwrap(),
            u_transform: gl.get_uniform_location(&program, "uTransform").unwrap(),
            program,
        }
    }

    pub fn render(
        &self,
        gl: &GL,
        bottom: f32,
        top: f32,
        left: f32,
        right: f32,
        canvas_width: f32,
        canvas_height: f32,
    ) {
        gl.use_program(Some(&self.program));

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.vertices_buffer));
        gl.vertex_attrib_pointer_with_i32(0, 2, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(0);

        gl.uniform4f(
            Some(&self.u_color),
            0.,  //r
            0.5, //g
            0.5, //b
            1.0, //a
        );

        gl.uniform1f(Some(&self.u_opacity), 1.);

        let translation_matrix = helpers::translation_matrix(
            2. * left / canvas_width - 1.,
            2. * bottom / canvas_height - 1.,
            0.,
        );

        let scale_matrix = helpers::scaling_matrix(
            2. * (right - left) / canvas_width,
            2. * (top - bottom) / canvas_height,
            0.,
        );

        let transform_mat = helpers::mult_matrix_4(&scale_matrix, &translation_matrix);

        gl.uniform_matrix4fv_with_f32_array(Some(&self.u_transform), false, &transform_mat);

        gl.draw_arrays(GL::TRIANGLES, 0, (self.rect_vertice_len / 2) as i32);
    }
}
