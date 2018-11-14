#![allow(non_snake_case)]

use gl_sys::{GLclampf, GLfloat, GLint, GLsizei, GL_INVALID_VALUE};

use frame_buffer::Rect;
use utilities::record_error;

#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct DepthRange {
    pub min: GLfloat,
    pub max: GLfloat,
}

#[derive(Debug, Default)]
pub(crate) struct ViewportTransformation {
    pub depth_range: DepthRange,
    pub viewport_rectangle: Rect,
    pub viewport_count: u32,
    pub scissor_count: u32,
}

#[no_mangle]
pub extern "C" fn glDepthRangef(z_near: GLclampf, z_far: GLclampf) {
    info!("glDepthRangef(z_near = {}, z_far = {}", z_near, z_far);
}

#[no_mangle]
pub extern "C" fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    info!("glViewport(x = {}, y = {}, width = {}, height = {})", x, y, width, height);

    if width < 0 || height < 0 {
        record_error(GL_INVALID_VALUE);
        return;
    }
}
