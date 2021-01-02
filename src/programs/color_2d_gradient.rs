use super::super::{helpers, shaders};
use js_sys::{Float32Array, Uint16Array};
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext as GL, WebGlUniformLocation};

pub struct Color2DGradient {
    program: WebGlProgram,
    color_buffer: WebGlBuffer,
    indices_count: i32,
    indices_buffer: WebGlBuffer,
    vertices_buffer: WebGlBuffer,
    u_opacity: WebGlUniformLocation,
    u_transform: WebGlUniformLocation,
}

impl Color2DGradient {
    pub fn new(gl: &GL) -> Self {
        let program = helpers::link_program(
            &gl,
            shaders::vertex::color_2d_gradient::SHADER,
            shaders::fragment::color_2d_gradient::SHADER,
        )
        .unwrap();

        let vertices_rect: [f32; 8] = [
            0., 1., // x, y
            0., 0., // x, y
            1., 1., // x, y
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

        let indices_rect: [u16; 6] = [0, 1, 2, 2, 1, 3];

        let indices_array = helpers::create_typed_array::<Uint16Array>(
            helpers::create_memory_buffer().unwrap(),
            indices_rect.as_ptr() as u32 / 2,
            indices_rect.len(),
        );

        let buffer_indices = gl
            .create_buffer()
            .ok_or("Failed to create indices buffer")
            .unwrap();
        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&buffer_indices));
        gl.buffer_data_with_array_buffer_view(
            GL::ELEMENT_ARRAY_BUFFER,
            &indices_array,
            GL::STATIC_DRAW,
        );

        Self {
            color_buffer: gl
                .create_buffer()
                .ok_or("Failed to create color buffer")
                .unwrap(),
            indices_count: indices_array.length() as i32,
            indices_buffer: buffer_indices,
            vertices_buffer,
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

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&self.color_buffer));
        gl.vertex_attrib_pointer_with_i32(1, 4, GL::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(1);

        let colors: [f32; 16] = [
            1., 0., 0., 1., // r/g/b/a
            0., 1., 0., 1., // r/g/b/a
            0., 0., 1., 1., // r/g/b/a
            1., 1., 1., 1., // r/g/b/a
        ];

        let color_vals_array = helpers::create_typed_array::<Float32Array>(
            helpers::create_memory_buffer().unwrap(),
            colors.as_ptr() as u32 / 4,
            colors.len(),
        );

        gl.buffer_data_with_array_buffer_view(
            GL::ARRAY_BUFFER,
            &color_vals_array,
            GL::DYNAMIC_DRAW,
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

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&self.indices_buffer));

        gl.draw_elements_with_i32(GL::TRIANGLES, self.indices_count, GL::UNSIGNED_SHORT, 0);
    }
}
