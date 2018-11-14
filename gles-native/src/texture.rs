#![allow(non_snake_case)]

use gl_sys::{
    GLboolean, GLenum, GLfloat, GLint, GLsizei, GLuint, GLvoid, GL_ALPHA, GL_INVALID_ENUM, GL_INVALID_VALUE, GL_LUMINANCE,
    GL_LUMINANCE_ALPHA, GL_RGB, GL_RGBA, GL_TEXTURE0, GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP, GL_TEXTURE_CUBE_MAP_NEGATIVE_Z,
    GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER, GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T,
    GL_UNSIGNED_BYTE, GL_UNSIGNED_SHORT_4_4_4_4, GL_UNSIGNED_SHORT_5_5_5_1, GL_UNSIGNED_SHORT_5_6_5,
};
use std::ptr;
use utilities::record_error;

#[derive(Debug, Default)]
pub struct Texture {}

fn validate_texture_parameters<T>(target: GLenum, pname: GLenum, params: *mut T) -> bool {
    !validate_texture_target_parameter(target, pname) || !validate_texture_parameter_ptr(params)
}

/// Return true if (target, pname) meet all the rules, otherwise false and record GL error flag
fn validate_texture_target_parameter(target: GLenum, pname: GLenum) -> bool {
    validate_texture_target(target) && validate_texture_parameter(pname)
}

fn validate_texture_target(target: GLenum) -> bool {
    if target != GL_TEXTURE_2D && target != GL_TEXTURE_CUBE_MAP {
        error!("invalid target: {}", target);
        record_error(GL_INVALID_ENUM);
        false
    } else {
        true
    }
}

fn validate_texture_parameter(pname: GLenum) -> bool {
    if pname != GL_TEXTURE_WRAP_S
        && pname != GL_TEXTURE_WRAP_T
        && pname != GL_TEXTURE_MIN_FILTER
        && pname != GL_TEXTURE_MAG_FILTER
    {
        error!("invalid pname: {}", pname);
        record_error(GL_INVALID_ENUM);
        false
    } else {
        true
    }
}

fn validate_texture_parameter_ptr<T>(params: *mut T) -> bool {
    validate_ptr(params, "params is nullptr")
}

/// Return true if ptr is not nullptr, otherwise false
fn validate_ptr<T>(ptr: *mut T, message: &str) -> bool {
    if ptr != ptr::null_mut() {
        warn!("{}", message);
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn glActiveTexture(texture: GLenum) {
    info!("glActiveTexture(texture = {:?})", texture);
}

#[no_mangle]
pub extern "C" fn glBindTexture(target: GLenum, texture: GLuint) {
    info!("glBindTexture(target = {:?}, texture = {})", target, texture);

    if target != GL_TEXTURE_2D && target != GL_TEXTURE_CUBE_MAP {
        error!("invalid value: {}", target);
        record_error(GL_INVALID_ENUM);
        return;
    }
}

#[no_mangle]
pub extern "C" fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    info!("glDeleteTextures(n = {}, textures = {:p})", n, textures);

    if n < 0 {
        error!("invalid count = {}", n);
        record_error(GL_INVALID_VALUE);
        return;
    }

    if !validate_ptr(textures as *mut GLuint, "textures is nullptr") {}
}

#[no_mangle]
pub extern "C" fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    info!("glGenTextures(n = {}, textures = {:p})", n, textures);

    if n < 0 {
        error!("Invalid size = {:?}", n);
        record_error(GL_INVALID_VALUE);
        return;
    }
}

#[no_mangle]
pub extern "C" fn glGenerateMipmap(target: GLenum) {
    info!("glGenerateMipmap(target = {:?})", target);

    if !validate_texture_target(target) {
        return;
    }
}

#[no_mangle]
pub extern "C" fn glIsTexture(texture: GLuint) -> GLboolean {
    info!("glIsTexture:(texture = {})", texture);

    unimplemented!()
}

#[no_mangle]
pub extern "C" fn glTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    r#type: GLenum,
    pixels: *const GLvoid,
) {
    info!(
        "glTexImage2D(target = {:?}, level = {:?}, internalformat = {:?}, width = {:?}, height = {:?}, \
         border = {:?}, format = {:?}, type = {:?}, pixels = {:?})",
        target, level, internalformat, width, height, border, format, r#type, pixels
    );

    if target != GL_TEXTURE_2D && (target < GL_TEXTURE_CUBE_MAP_POSITIVE_X || target > GL_TEXTURE_CUBE_MAP_NEGATIVE_Z) {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if format != GL_ALPHA && format != GL_RGB && format != GL_RGBA && format != GL_LUMINANCE && format != GL_LUMINANCE_ALPHA {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if r#type != GL_UNSIGNED_BYTE
        && r#type != GL_UNSIGNED_SHORT_5_6_5
        && r#type != GL_UNSIGNED_SHORT_4_4_4_4
        && r#type != GL_UNSIGNED_SHORT_5_5_5_1
    {
        record_error(GL_INVALID_ENUM);
        return;
    }

    if (target >= GL_TEXTURE_CUBE_MAP_POSITIVE_X && target <= GL_TEXTURE_CUBE_MAP_NEGATIVE_Z) && (width != height) {
        record_error(GL_INVALID_VALUE);
        return;
    }

    if internalformat != GL_ALPHA
        && internalformat != GL_RGB
        && internalformat != GL_RGBA
        && internalformat != GL_LUMINANCE
        && internalformat != GL_LUMINANCE_ALPHA
    {
        record_error(GL_INVALID_VALUE);
        return;
    }
}

#[no_mangle]
pub extern "C" fn glTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: *const GLvoid,
) {
    info!("glTexSubImage2D(target = {:?}, level = {:?}, xoffset = {:?}, yoffset = {:?}, width = {:?}, height = {:?}, format = {:?}, type = {:?}, pixels = {:?})", target, level, xoffset, yoffset, width, height, format, type_, pixels);
}

#[no_mangle]
pub extern "C" fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    info!(
        "glTexParameterf(target = {:?}, pname = {:?}, param = {:?})",
        target, pname, param
    );

    glTexParameteri(target, pname, param as _);
}

#[no_mangle]
pub extern "C" fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) {
    info!(
        "glTexParameterfv(target = {:?}, pname = {:?}, param = {:p})",
        target, pname, params
    );

    glTexParameteri(target, pname, unsafe { *params.offset(0) } as _);
}

#[no_mangle]
pub extern "C" fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    info!(
        "glTexParameteri(target = {:?}, pname = {:?}, param = {:?})",
        target, pname, param
    );

    if !validate_texture_target_parameter(target, pname) {
        return;
    }
}

#[no_mangle]
pub extern "C" fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    info!(
        "glTexParameteriv(target = {:?}, pname = {:?}, param = {:p})",
        target, pname, params
    );

    glTexParameteri(target, pname, unsafe { *params.offset(0) });
}

#[no_mangle]
pub extern "C" fn glCompressedTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    info!(
        "glCompressedTexImage2D(target = {:?}, level = {:?}, internalformat = {:?}, width = {:?}, height = {:?}， border = {:?}, imageSize = {:?}, data = {:?})",
        target, level, internalformat, width, height, border, imageSize, data
    );
}

#[no_mangle]
pub extern "C" fn glCompressedTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    imageSize: GLsizei,
    data: *const GLvoid,
) {
    info!(
        "glCompressedTexSubImage2D(target = {:?}, level = {:?}, xoffset = {:?}, yoffset = {:?}, width = {:?}, height ={:?}, format = {:?}, imageSize = {:?}, data = {:?})",
        target, level, xoffset, yoffset, width, height, format, imageSize, data
    );
}

#[no_mangle]
pub extern "C" fn glCopyTexImage2D(
    target: GLenum,
    level: GLint,
    internalformat: GLenum,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
) {
    info!(
        "glCopyTexImage2D(target = {:?}, level = {:?}, internalformat = {:?}, x = {:?}, y = {:?}, width = {:?}, height = {:?}, border = {:?})",
        target, level, internalformat, x, y, width, height, border
    );
}

#[no_mangle]
pub extern "C" fn glCopyTexSubImage2D(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    x: GLint,
    y: GLint,
    width: GLsizei,
    height: GLsizei,
) {
    info!(
        "glCopyTexSubImage2D(target = {:?}, level = {:?}, xoffset = {:?}, yoffset = {:?}, x = {:?}, y = {:?}, width = {:?}, height = {:?})", target, level, xoffset, yoffset, x, y, width, height
    );
}

#[no_mangle]
pub extern "C" fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) {
    info!(
        "glGetTexParameterfv(target = {:?}, pname = {:?}, params = {:p})",
        target, pname, params
    );

    if !validate_texture_parameters(target, pname, params) {
        return;
    }
}

#[no_mangle]
pub extern "C" fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    info!(
        "glGetTexParameteriv(target = {:?}, pname = {:?}, params = {:p})",
        target, pname, params
    );

    if !validate_texture_parameters(target, pname, params) {
        return;
    }
}
