#![allow(non_snake_case)]

use gl_sys::{
    GLenum, GLint, GLsizei, GLvoid, GL_ALPHA, GL_FRAMEBUFFER_COMPLETE, GL_INVALID_ENUM, GL_INVALID_FRAMEBUFFER_OPERATION,
    GL_INVALID_OPERATION, GL_INVALID_VALUE, GL_LUMINANCE, GL_LUMINANCE_ALPHA, GL_PACK_ALIGNMENT, GL_RGB, GL_RGBA,
    GL_UNPACK_ALIGNMENT, GL_UNSIGNED_BYTE, GL_UNSIGNED_SHORT_4_4_4_4, GL_UNSIGNED_SHORT_5_5_5_1, GL_UNSIGNED_SHORT_5_6_5,
};

use context::{self, HUB};
use rendering::glFinish;
use utilities::record_error;

#[derive(Debug, Default)]
pub(crate) struct PixelStorageState {}

#[no_mangle]
pub extern "C" fn glPixelStorei(pname: GLenum, param: GLint) {
    info!("glPixelStorei(pname = {}, param = {}", pname, param);

    if pname != GL_PACK_ALIGNMENT && pname != GL_UNPACK_ALIGNMENT {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if param != 1 && param != 2 && param != 4 && param != 8 {
        record_error(GL_INVALID_VALUE);
        return;
    }
}

#[no_mangle]
pub extern "C" fn glReadPixels(
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *mut GLvoid,
) {
    info!(
        "glReadPixels(x = {}, y = {}, width = {}, height = {}, format = {}, type = {}, pixels = {:?})",
        x, y, width, height, format, type_, pixels
    );

    if width < 0 || height < 0 {
        record_error(GL_INVALID_VALUE);
        return;
    }

    if format != GL_ALPHA && format != GL_RGB && format != GL_RGBA && format != GL_LUMINANCE && format != GL_LUMINANCE_ALPHA {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if type_ != GL_UNSIGNED_BYTE
        && type_ != GL_UNSIGNED_SHORT_5_6_5
        && type_ != GL_UNSIGNED_SHORT_4_4_4_4
        && type_ != GL_UNSIGNED_SHORT_5_5_5_1
    {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if (type_ == GL_UNSIGNED_SHORT_5_6_5 && format != GL_RGB)
        || (type_ == GL_UNSIGNED_SHORT_5_5_5_1 || type_ == GL_UNSIGNED_SHORT_4_4_4_4 && format != GL_RGBA)
        || (type_ == GL_UNSIGNED_BYTE && format != GL_RGBA)
    {
        record_error(GL_INVALID_OPERATION);
        return;
    }
}
