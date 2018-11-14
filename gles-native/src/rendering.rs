#![allow(non_snake_case)]

use std::ptr;

use context::{is_nullptr, HUB};
use frame_buffer::glCheckFramebufferStatus;
use gl_sys::{
    GLbitfield, GLenum, GLint, GLsizei, GLvoid, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_FRAMEBUFFER,
    GL_FRAMEBUFFER_COMPLETE, GL_FRONT_AND_BACK, GL_INVALID_ENUM, GL_INVALID_FRAMEBUFFER_OPERATION, GL_INVALID_VALUE,
    GL_STENCIL_BUFFER_BIT, GL_TRIANGLES, GL_TRIANGLE_FAN, GL_TRIANGLE_STRIP, GL_UNSIGNED_BYTE, GL_UNSIGNED_SHORT,
};
use utilities::record_error;

#[no_mangle]
pub extern "C" fn glFinish() {
    info!("glFinish()");

    if !flush() {
        return;
    }
}

#[no_mangle]
pub extern "C" fn glFlush() {
    info!("glFlush()");

    flush();
}

#[no_mangle]
pub extern "C" fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei) {
    info!("glDrawArrays(mode = {}, first = {}, count = {})", mode, first, count);

    if mode > GL_TRIANGLE_FAN {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if count < 0 {
        record_error(GL_INVALID_VALUE);
        return;
    }
    if 0 == count {
        return;
    }

    if glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE {
        record_error(GL_INVALID_FRAMEBUFFER_OPERATION);
        return;
    }
}

#[no_mangle]
pub extern "C" fn glDrawElements(mode: GLenum, count: GLsizei, type_: GLenum, indices: *const GLvoid) {
    info!(
        "glDrawElements(mode = {}, count = {}, type = {}, indices = {:p}",
        mode, count, type_, indices
    );

    if mode > GL_TRIANGLE_FAN || !(type_ == GL_UNSIGNED_BYTE || type_ == GL_UNSIGNED_SHORT) {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if count < 0 {
        record_error(GL_INVALID_VALUE);
        return;
    }
    if 0 == count {
        return;
    }

    if !is_nullptr(indices as *mut GLvoid, "indices is nullptr") {
        return;
    }

    if glCheckFramebufferStatus(GL_FRAMEBUFFER) != GL_FRAMEBUFFER_COMPLETE {
        record_error(GL_INVALID_FRAMEBUFFER_OPERATION);
        return;
    }
}

#[no_mangle]
pub extern "C" fn glClear(mask: GLbitfield) {
    info!("glClear(mask = {})", mask);

    let clearColorEnabled = (mask & GL_COLOR_BUFFER_BIT) == 0;
    let clearDepthEnabled = (mask & GL_DEPTH_BUFFER_BIT) == 0;
    let clearStencilEnabled = (mask & GL_STENCIL_BUFFER_BIT) == 0;

    if !clearColorEnabled && !clearDepthEnabled && !clearStencilEnabled {
        error!("invalid value {} for glClear", mask);
        record_error(GL_INVALID_VALUE);
        return;
    }
}

fn flush() -> bool {
    unimplemented!()
}

#[inline]
fn is_draw_mode_triangle(mode: GLenum) -> bool {
    mode == GL_TRIANGLE_STRIP || mode == GL_TRIANGLE_FAN || mode == GL_TRIANGLES
}
