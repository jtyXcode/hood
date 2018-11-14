use std::ffi::{CStr, CString};
use std::ops::Range;
use std::{ptr, slice};

use gl_sys::{
    GLboolean, GLchar, GLenum, GLint, GLsizei, GLuint, GLvoid, GL_FALSE, GL_FRAGMENT_SHADER, GL_INVALID_ENUM,
    GL_INVALID_OPERATION, GL_INVALID_VALUE, GL_TRUE, GL_VERTEX_SHADER,
};

use context::{generate_objects, is_nullptr, is_valid_object, HUB};
use utilities::record_error;

#[derive(Debug, Default)]
pub struct Shader {
    pub type_: GLenum,
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glCompileShader(shader: GLuint) {
    info!("glCompileShader(shader = {})", shader);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glCreateShader(r#type: GLenum) -> GLuint {
    info!("glCreateShader(type = {:?})", r#type);

    if r#type != GL_VERTEX_SHADER && r#type != GL_FRAGMENT_SHADER {
        error!("invalid type = {:?}", r#type);
        record_error(GL_INVALID_ENUM);
        return 0;
    }

    let mut shader_object_name = 0;
    generate_objects(1, &mut shader_object_name, &HUB.shader_pool);
    let mut shader_pool_guard = HUB.shader_pool.lock();
    let shader_object = shader_pool_guard.get_object_mut(shader_object_name);
    shader_object.type_ = r#type;
    shader_object_name
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glDeleteShader(shader: GLuint) {
    info!("glDeleteShader(shader = {})", shader);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glShaderBinary(
    n: GLsizei,
    shaders: *const GLuint,
    binaryformat: GLenum,
    binary: *const GLvoid,
    length: GLsizei,
) {
    info!(
        "glShaderBinary(n = {:?}, shaders = {:?}, binaryformat = {:?}, binary = {:?}, length = {:?} )",
        n, shaders, binaryformat, binary, length
    );

    unimplemented!()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glShaderSource(shader: GLuint, count: GLsizei, string: *const *const GLchar, length: *const GLint) {
    info!(
        "glShaderSource(shader = {:?}, count = {:?}, string = {:?}, length = {:?})",
        shader, count, string, length
    );

    if count < 0 {
        record_error(GL_INVALID_VALUE);
        error!("glShaderSource: The param: count is less than zero");
        return;
    }

    if !is_nullptr(string as *mut GLchar, "string is not allowed to set nullptr") {
        record_error(GL_INVALID_VALUE);
        return;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint) {
    info!(
        "glGetShaderiv(shader = {:?}, pname = {:?}, params = {:?})",
        shader, pname, params
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetShaderInfoLog(shader: GLuint, bufsize: GLsizei, length: *mut GLsizei, infolog: *mut GLchar) {
    info!(
        "glGetShaderInfoLog(shader = {:?}, bufsize = {:?}, length = {:?}, infolog = {:?}",
        shader, bufsize, length, infolog
    );

    if bufsize < 0 {
        error!("glGetShaderInfoLog: Invalid Value For Buffer Size= {:?}", bufsize);
        record_error(GL_INVALID_VALUE);
    }

    if !is_nullptr(infolog, "infolog is not allowed to set nullptr") {
        record_error(GL_INVALID_VALUE);
        return;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetShaderPrecisionFormat(
    shadertype: GLenum,
    precisiontype: GLenum,
    range: *mut GLint,
    precision: *mut GLint,
) {
    info!(
        "glGetShaderPrecisionFormat(shadertype = {:?}, precisiontype = {:?}, range = {:?}, precision = {:?})",
        shadertype, precisiontype, range, precision
    );
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGetShaderSource(shader: GLuint, bufsize: GLsizei, length: *mut GLsizei, source: *mut GLchar) {
    info!(
        "glGetShaderSource(shader = {:?}, bufsize = {:?}, length = {:?}, source = {:?})",
        shader, bufsize, length, source
    );

    if bufsize < 0 {
        error!("glGetShaderSource: Invalid Value For Buffer Size= {:?}", bufsize);
        record_error(GL_INVALID_VALUE);
        return;
    }

    if !is_nullptr(source, "source is not allowed to set nullptr") {
        record_error(GL_INVALID_OPERATION);
        return;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glIsShader(shader: GLuint) -> GLboolean {
    info!("glIsShader(shader = {})", shader);

    is_valid_object(shader, &HUB.shader_pool)
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glReleaseShaderCompiler() {
    info!("glReleaseShaderCompiler()");

    unimplemented!()
}
