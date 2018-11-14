use std::ptr;

use hal;

use gl_sys::{
    GLboolean, GLenum, GLint, GLsizei, GLuint, GL_COLOR_ATTACHMENT0, GL_DEPTH_ATTACHMENT, GL_FALSE,
    GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME, GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE,
    GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE, GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL, GL_FRAMEBUFFER_COMPLETE,
    GL_FRAMEBUFFER_DEFAULT, GL_NONE, GL_RENDERBUFFER, GL_STENCIL_ATTACHMENT, GL_TEXTURE, GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP,
    GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TRUE,
};

use context::{self, HUB};

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub(crate) struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self { x, y, width, height }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Idle,
    Clear,
    ClearDraw,
    Draw,
    Delete,
}
impl Default for State {
    fn default() -> Self {
        State::Idle
    }
}

#[derive(Debug, Default)]
pub(crate) struct FrameBuffer {
    pub dimensions: Rect,
    pub target: GLenum,
    pub write_buffer_index: u32,
    state: State,
    pub is_updated: bool,
    pub is_size_updated: bool,
    pub is_system_frame_buffer: bool,
    pub is_bound_to_texture: bool,
    //    attachment_colors: Vec<Attachment>,
    //    pub attachment_depth: Attachment,
    //    pub attachment_stencil: Attachment,
    //    pub command_buffer_manager: *mut CommandBufferManager,
}

pub(crate) fn check_frame_buffer_status(object: &mut FrameBuffer) -> GLenum {
    use gl_sys::{GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS, GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT};

    // todo: refactor
    GL_FRAMEBUFFER_COMPLETE
}

#[inline]
fn validate_frame_buffer_target(target: GLenum) -> bool {
    use gl_sys::GL_FRAMEBUFFER;
    context::validate_invalid_enum(target, &[GL_FRAMEBUFFER], "invalid framebuffer target")
}

#[inline]
fn validate_is_default_frame_buffer_active() -> bool {
    use gl_sys::GL_INVALID_OPERATION;

    //    if context::is_default_frame_buffer(HUB.active_frame_buffer.lock().name) {
    //        record_error(GL_INVALID_OPERATION);
    ////        return;
    //    }
    true
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glBindFramebuffer(target: GLenum, framebuffer: GLuint) {
    info!("glBindFramebuffer(target = {:?}, framebuffer = {})", target, framebuffer);

    use gl_sys::GL_INVALID_VALUE;

    context::bind_target_object_name(
        target,
        framebuffer,
        validate_frame_buffer_target,
        context::get_default_frame_buffer(target),
        |object_name| {
            let mut pool_guard = HUB.frame_buffer_pool.lock();
            let object = pool_guard.get_object_mut(object_name);
            if object.target == GL_INVALID_VALUE {
                object.target = target;
            }
            object as *mut FrameBuffer
        },
        |object_ptr| {
            let mut object_guard = HUB.active_frame_buffer.lock();
            object_guard.name = framebuffer;
            object_guard.ptr = object_ptr;
        },
    )
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glCheckFramebufferStatus(target: GLenum) -> GLenum {
    info!("glCheckFramebufferStatus(target = {:?})", target);

    use gl_sys::GL_ZERO;

    if !validate_frame_buffer_target(target) {
        return GL_ZERO;
    }

    let active_object = HUB.active_frame_buffer.lock();
    if context::is_default_frame_buffer(active_object.name) {
        GL_FRAMEBUFFER_COMPLETE
    } else {
        let mut guard = HUB.frame_buffer_pool.lock();
        let frame_buffer = guard.get_object_mut(active_object.name);
        check_frame_buffer_status(frame_buffer)
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint) {
    info!("glDeleteFramebuffers(n = {}, framebuffers = {:p})", n, framebuffers);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glFramebufferRenderbuffer(
    target: GLenum,
    attachment: GLenum,
    renderbuffertarget: GLenum,
    renderbuffer: GLuint,
) {
    info!(
        "glFramebufferRenderbuffer(target = {:?}, attachment = {:?}, renderbuffertarget = {:?}, renderbuffer = {})",
        target, attachment, renderbuffertarget, renderbuffer
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glFramebufferTexture2D(target: GLenum, attachment: GLenum, textarget: GLenum, texture: GLuint, level: GLint) {
    info!(
        "glFramebufferTexture2D(target = {:?}, attachment = {:?}, textarget = {:?}, texture = {}, level = {})",
        target, attachment, textarget, texture, level
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint) {
    info!("glGenFramebuffers(n = {}, framebuffers = {:p})", n, framebuffers);

    context::generate_objects(n, framebuffers, &HUB.frame_buffer_pool);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetFramebufferAttachmentParameteriv(target: GLenum, attachment: GLenum, pname: GLenum, params: *mut GLint) {
    info!(
        "glGetFramebufferAttachmentParameteriv(target = {:?}, attachment = {:?}, pname = {:?} params = {:p}",
        target, attachment, pname, params
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean {
    info!("glIsFramebuffer(framebuffer = {})", framebuffer);

    context::is_valid_object(framebuffer, &HUB.frame_buffer_pool)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use gl_sys::*;

    #[test]
    fn test_all_in_one() {
        &*HUB;
    }
}