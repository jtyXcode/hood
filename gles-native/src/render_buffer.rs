use gl_sys::{
    GLboolean, GLenum, GLint, GLsizei, GLuint, GL_COLOR_ATTACHMENT0, GL_DEPTH_COMPONENT16, GL_FALSE, GL_INVALID_ENUM,
    GL_INVALID_OPERATION, GL_MAX_RENDERBUFFER_SIZE, GL_OUT_OF_MEMORY, GL_RENDERBUFFER, GL_RENDERBUFFER_ALPHA_SIZE,
    GL_RENDERBUFFER_BLUE_SIZE, GL_RENDERBUFFER_DEPTH_SIZE, GL_RENDERBUFFER_GREEN_SIZE, GL_RENDERBUFFER_HEIGHT,
    GL_RENDERBUFFER_INTERNAL_FORMAT, GL_RENDERBUFFER_RED_SIZE, GL_RENDERBUFFER_STENCIL_SIZE, GL_RENDERBUFFER_WIDTH, GL_RGB565,
    GL_RGB5_A1, GL_RGBA4, GL_STENCIL_INDEX8, GL_TRUE,
};

use context::{self, HUB};
use texture::Texture;
use utilities::record_error;

#[derive(Debug, Default)]
pub(crate) struct RenderBuffer {
    pub internal_format: GLenum,
    pub target: GLenum,

    pub texture: Option<Texture>,
    pub index: u32,

    pub attached_frame_buffer_index: u32,
    pub component_size: GLint,
}

pub(crate) fn init_texture(object: &mut RenderBuffer) {
    object.texture = Some(Texture::default());
    //    object.texture.as_mut().unwrap().init_state();
}

//pub fn allocate(&mut self, width: i32, height: i32, render_internal_format: RenderBufferInterFormat) -> bool {
//    false
//}
//
//pub fn set_texture(&mut self, texture: Texture) -> &mut Self {
//    self.texture = Some(texture);
//    self
//}
//
//pub fn is_allocated(&self) -> bool {
//    if self.texture.is_none() {
//        return false;
//    } else {
//        //todo: texture is allocated?
//        unimplemented!()
//    }
//}

fn validate_render_buffer_target(target: GLenum) -> bool {
    if target != GL_RENDERBUFFER {
        error!("invalid target = {}", target);
        record_error(GL_INVALID_ENUM);
        false
    } else {
        true
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint) {
    info!("glBindRenderbuffer(target = {:?}, renderbuffer = {})", target, renderbuffer);

    use gl_sys::GL_INVALID_VALUE;
    use std::ptr;

    context::bind_target_object_name(
        target,
        renderbuffer,
        validate_render_buffer_target,
        ptr::null_mut(),
        |object_name| {
            let mut pool_guard = HUB.render_buffer_pool.lock();
            let object = pool_guard.get_object_mut(object_name);
            if object.target == GL_INVALID_VALUE {
                object.target = target;
                init_texture(object);
            }
            object as *mut RenderBuffer
        },
        |object_ptr| context::set_active_render_buffer(target, renderbuffer, object_ptr),
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint) {
    info!("glDeleteRenderbuffers(n = {}, renderbuffers = {:p}", n, renderbuffers);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    info!(
        "glRenderbufferStorage(target = {:?}, pname = {:?}, params = {:p})",
        target, pname, params
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint) {
    info!("glGenRenderbuffers(n = {}, renderbuffers = {:p}", n, renderbuffers);

    context::generate_objects(n, renderbuffers, &HUB.render_buffer_pool);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glRenderbufferStorage(target: GLenum, internalformat: GLenum, width: GLsizei, height: GLsizei) {
    info!(
        "glRenderbufferStorage(target = {:?}, internalformat = {:?}, width = {}, height = {})",
        target, internalformat, width, height
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean {
    info!("glIsRenderbuffer(renderbuffer = {})", renderbuffer);

    context::is_valid_object(renderbuffer, &HUB.render_buffer_pool)
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
