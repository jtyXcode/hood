use gl_sys::{
    GLboolean, GLenum, GLubyte, GL_EXTENSIONS, GL_FALSE, GL_INVALID_ENUM, GL_NO_ERROR, GL_RENDERER, GL_SHADING_LANGUAGE_VERSION,
    GL_TRUE, GL_VENDOR, GL_VERSION,
};

use context::{self, HUB};

#[derive(Debug)]
pub(crate) struct Error {
    pub raw: GLenum,
}

impl Default for Error {
    fn default() -> Self {
        Self { raw: GL_NO_ERROR }
    }
}

// todo refactor to record_error(err, msg)?

//#[cfg(feature = "Release")]
#[inline]
pub(crate) fn record_error(error: GLenum) {
    let mut error_guard = HUB.error.lock();
    if error_guard.raw == GL_NO_ERROR {
        error_guard.raw = error;
    }
}
//
//#[cfg(feature = "Debug")]
//#[inline]
//pub(crate) fn record_error(error: GLenum) {
//    let mut error_guard = HUB.error.lock();
//    if error_guard.raw == GL_NO_ERROR {
//        error_guard.raw = error;
//    }
//}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetError() -> GLenum {
    info!("glGetError");

    let mut error = HUB.error.lock();
    let current_error = error.raw;
    error.raw = GL_NO_ERROR;
    current_error
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetString(name: GLenum) -> *const GLubyte {
    info!("glGetString(name: {:?})", name);

    static STRINGS: [&'static str; 5] = [
        "Hood\0",
        "OpenGL ES 2.0 Over Hood\0",
        "OpenGL ES 2.0\0",
        "OpenGL ES GLSL ES 1.00\0",
        "GL_OES_get_program_binary\0",
    ];

    match name {
        GL_VENDOR => STRINGS[0],
        GL_RENDERER => STRINGS[1],
        GL_VERSION => STRINGS[2],
        GL_SHADING_LANGUAGE_VERSION => STRINGS[3],
        GL_EXTENSIONS => STRINGS[4],
        _ => {
            error!("GL_INVALID_ENUM");
            record_error(GL_INVALID_ENUM);
            return core::ptr::null();
        }
    }
    .as_ptr()
}
