use std::ptr;

use back;
use hal;

use gl_sys::{
    GLboolean, GLenum, GLfloat, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint, GLvoid, GL_ALPHA, GL_CLAMP_TO_EDGE,
    GL_COLOR_ATTACHMENT0, GL_FALSE, GL_GENERATE_MIPMAP_HINT, GL_INVALID_ENUM, GL_INVALID_OPERATION, GL_INVALID_VALUE, GL_LINEAR,
    GL_LINEAR_MIPMAP_LINEAR, GL_LINEAR_MIPMAP_NEAREST, GL_LUMINANCE, GL_LUMINANCE_ALPHA, GL_MAX_CUBE_MAP_TEXTURE_SIZE,
    GL_MAX_TEXTURE_SIZE, GL_MIRRORED_REPEAT, GL_NEAREST, GL_NEAREST_MIPMAP_LINEAR, GL_NEAREST_MIPMAP_NEAREST, GL_REPEAT, GL_RGB,
    GL_RGBA, GL_TEXTURE_CUBE_MAP_NEGATIVE_Z, GL_TEXTURE_CUBE_MAP_POSITIVE_X, GL_TEXTURE_MAG_FILTER, GL_TEXTURE_MIN_FILTER,
    GL_TEXTURE_WRAP_S, GL_TEXTURE_WRAP_T,
};

use active_object;
use context::{self, HUB};

use {HalImage, HalImageView, HalMemory, HalSampler};

use hal_registry;

// todo: auto increase value like C enum
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum TextureType {
    TwoD = 0,
    CubeMap = 1,
    Total = 2,
}

#[derive(Debug)]
pub struct TextureUnit {
    pub raw: GLenum,
}

impl Default for TextureUnit {
    fn default() -> Self {
        use gl_sys::GL_TEXTURE0;
        Self { raw: GL_TEXTURE0 }
    }
}

#[derive(Debug)]
pub struct Texture {
    pub format: GLenum,
    pub target: GLenum,
    pub type_: GLenum,
    pub internal_format: GLenum,

    pub mip_level_count: GLint,
    pub layers_count: GLint,

    //    states: Vec<StateMap>,
    pub explicit_type: GLenum,
    pub explicit_internal_format: GLenum,

    //    pub dimensions: Rect,
    //    pub sampling_params: Sampler,
    pub data_updated: bool,
    pub data_no_inversion: bool,

    pub memory: Option<HalMemory>,
    pub image: Option<HalImage>,
    pub image_view: Option<HalImageView>,
    pub sampler: Option<HalSampler>,
}

impl Texture {
    pub const DEFAULT_INTERNAL_ALIGNMENT: GLint = 1;
    pub const TEXTURE_2D_LAYERS: GLint = 1;
    pub const TEXTURE_CUBE_MAP_LAYERS: GLint = 6;

    pub fn new(
        format: GLenum,
        target: GLenum,
        type_: GLenum,
        internal_format: GLenum,
        explicit_type: GLenum,
        explicit_internal_format: GLenum,
        mip_level_count: GLint,
        layers_count: GLint,
        data_updated: bool,
        data_no_inversion: bool,
    ) -> Self {
        Self {
            format,
            target,
            type_,
            internal_format,
            mip_level_count,
            layers_count,
            //            states: Vec::new(),
            explicit_type,
            explicit_internal_format,
            //            dimensions: Rect::default(),
            //            sampling_params: Sampler::default(),
            memory: None,
            image: None,
            image_view: None,
            sampler: None,
            data_updated,
            data_no_inversion,
        }
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::new(
            GL_INVALID_VALUE,
            GL_INVALID_VALUE,
            GL_INVALID_VALUE,
            GL_INVALID_VALUE,
            GL_INVALID_VALUE,
            GL_INVALID_VALUE,
            1,
            1,
            false,
            false,
        )
    }
}

fn texture_allocate(texture: &mut Texture) -> bool {
    trace!("texture_allocate(texture: &mut Texture)");
    if !hal_registry::hal_texture_create() {
        return false;
    }
    true
}



pub fn generate_mip_maps(target: GLenum) {}

/// Return true if (target, pname, params) meet all the rules, otherwise false and record GL error flag
fn validate_texture_parameters<T>(target: GLenum, pname: GLenum, params: *mut T) -> bool {
    !validate_texture_target_parameter(target, pname) || !validate_texture_parameter_ptr(params)
}

fn validate_texture_target_parameter(target: GLenum, pname: GLenum) -> bool {
    validate_texture_target(target) && validate_texture_parameter(pname)
}

fn validate_texture_target(target: GLenum) -> bool {
    use gl_sys::{GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP};
    context::validate_target(target, &[GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP])
}

pub(crate) fn validate_texture_format(format: GLenum) -> bool {
    use gl_sys::{GL_ALPHA, GL_LUMINANCE, GL_LUMINANCE_ALPHA, GL_RGB, GL_RGBA};
    context::validate_invalid_enum(
        format,
        &[GL_ALPHA, GL_RGB, GL_RGBA, GL_LUMINANCE, GL_LUMINANCE_ALPHA],
        "invalid texture format",
    )
}

pub(crate) fn validate_texture_type(r#type: GLenum) -> bool {
    use gl_sys::{GL_UNSIGNED_BYTE, GL_UNSIGNED_SHORT_4_4_4_4, GL_UNSIGNED_SHORT_5_5_5_1, GL_UNSIGNED_SHORT_5_6_5};
    context::validate_invalid_enum(
        r#type,
        &[
            GL_UNSIGNED_BYTE,
            GL_UNSIGNED_SHORT_5_6_5,
            GL_UNSIGNED_SHORT_4_4_4_4,
            GL_UNSIGNED_SHORT_5_5_5_1,
        ],
        "invalid texture type",
    )
}

#[inline]
pub(crate) fn validate_pixel_internal_format(internal_format: GLenum, required_values: &[GLenum], message: &str) -> bool {
    context::validate_invalid_value(
        internal_format,
        |&internal_format| context::find_value_in_slice(internal_format, required_values, |_| {}),
        "",
    )
}

fn validate_texture_parameter(pname: GLenum) -> bool {
    if pname != GL_TEXTURE_WRAP_S
        && pname != GL_TEXTURE_WRAP_T
        && pname != GL_TEXTURE_MIN_FILTER
        && pname != GL_TEXTURE_MAG_FILTER
    {
        error!("invalid pname: {}", pname);
        //        record_error(GL_INVALID_ENUM);
        false
    } else {
        true
    }
}

fn validate_texture_parameter_ptr<T>(params: *mut T) -> bool {
    context::is_nullptr(params, "params is nullptr")
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glActiveTexture(texture: GLenum) {
    info!("glActiveTexture(texture = {:?})", texture);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glBindTexture(target: GLenum, texture: GLuint) {
    info!("glBindTexture(target = {:?}, texture = {})", target, texture);

    context::bind_target_object_name(
        target,
        texture,
        validate_texture_target,
        context::get_default_texture(target),
        |object_name| {
            debug!("enter modify");
            let mut pool_guard = HUB.texture_pool.lock();
            let object = pool_guard.get_object_mut(object_name);
            debug!("{:?}", object);
            if object.target != GL_INVALID_VALUE && object.target != target {
                debug!("object.target != target");
                ptr::null_mut()
            } else {
                object.target = target;
                //                init_state(object);
                object as *mut Texture
            }
        },
        |object_ptr| {
            //            let frame_buffer_pool = guard.frame_pool.get_objects();
            //            for (index, frame_buffer) in frame_buffer_pool {
            //                if frame_buffer.get_color_attachment_type() == GL_COLOR_ATTACHMENT0
            //                    && texture == frame_buffer.get_color_attachment_name()
            //                    {
            //                        frame_buffer.is_updated = true;
            //                    } else if frame_buffer.attachment_depth.r#type == GL_COLOR_ATTACHMENT0
            //                    && texture == frame_buffer.attachment_depth.name
            //                    {
            //                        frame_buffer.is_updated = true;
            //                    } else if frame_buffer.attachment_stencil.r#type == GL_COLOR_ATTACHMENT0
            //                    && texture == frame_buffer.attachment_stencil.name
            //                    {
            //                        frame_buffer.is_updated = true;
            //                    }
            //            }

            debug!("object_ptr = {:?}", object_ptr);
            context::set_active_texture(target, texture, object_ptr);
            let mut guard = HUB.active_program.lock();
            if guard.name != 0 {
                active_object::get_object_mut(&mut guard).unwrap().update_descriptor_sets = true;
            }
        },
    );
}

#[no_mangle]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
pub extern "C" fn glDeleteTextures(n: GLsizei, textures: *const GLuint) {
    info!("glDeleteTextures(n = {}, textures = {:p})", n, textures);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGenerateMipmap(target: GLenum) {
    info!("glGenerateMipmap(target = {:?})", target);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glGenTextures(n: GLsizei, textures: *mut GLuint) {
    info!("glGenTextures(n = {}, textures = {:p})", n, textures);

    context::generate_objects(n, textures, &HUB.texture_pool);
}

#[no_mangle]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
pub extern "C" fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    info!(
        "glGetTexParameteriv(target = {:?}, pname = {:?}, params = {:p})",
        target, pname, params
    );

    if !validate_texture_parameters(target, pname, params) {
        return;
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glIsTexture(texture: GLuint) -> GLboolean {
    info!("glIsTexture:(texture = {})", texture);

    context::is_valid_object(texture, &HUB.texture_pool)
}

#[no_mangle]
#[allow(non_snake_case)]
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

    //todo:refactor param check
    context::object_upload_data(
        || true,
        context::get_active_texture(target),
        |object| {
            debug!("enter operation block with texture object = {:?}", object);
            true
        },
        |object| {
            debug!("enter allocate guard closure");
            texture_allocate(object)
        },
        || debug!("set new state"),
    );
}

#[no_mangle]
#[allow(non_snake_case)]
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
#[allow(non_snake_case)]
pub extern "C" fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    info!(
        "glTexParameterf(target = {:?}, pname = {:?}, param = {:?})",
        target, pname, param
    );

    glTexParameteri(target, pname, param as _);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) {
    info!(
        "glTexParameterfv(target = {:?}, pname = {:?}, param = {:p})",
        target, pname, params
    );

    glTexParameteri(target, pname, unsafe { *params.offset(0) } as _);
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint) {
    info!(
        "glTexParameteri(target = {:?}, pname = {:?}, param = {:?})",
        target, pname, param
    );

    // todo: refactor
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) {
    info!(
        "glTexParameteriv(target = {:?}, pname = {:?}, param = {:p})",
        target, pname, params
    );

    glTexParameteri(target, pname, unsafe { *params.offset(0) });
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use gl_sys::*;
    use utilities::record_error;

    #[test]
    fn test_all_in_one() {
        &*HUB;

        let count = 1;
        let mut buffers = Vec::<GLuint>::with_capacity(count);
        glGenTextures(count as _, buffers.as_mut_ptr());
        unsafe {
            buffers.set_len(count as usize);
        };
        assert_eq!(1, buffers.len());

        glBindTexture(GL_TEXTURE_2D, buffers[0]);

        use utilities::glGetError;

        assert_eq!(GL_NO_ERROR, glGetError());

        record_error(GL_INVALID_FRAMEBUFFER_OPERATION);
        warn!("GL_INVALID_FRAMEBUFFER_OPERATION = {}", GL_INVALID_FRAMEBUFFER_OPERATION);
        warn!("glGetError = {}", glGetError());

        glBindTexture(GL_TEXTURE, buffers[0]);
        assert_ne!(GL_NO_ERROR, glGetError());

        glBindTexture(GL_TEXTURE_2D, buffers[0]);
        {
            warn!("test block 1->");
            let texture = buffers[0];
            let mut guard = HUB.texture_pool.lock();
            let texture_object = guard.get_object_mut(texture);
            warn!("test block 1-> {:?}", texture_object);
        }

        error!("current active_texture = {:?}", context::get_active_texture(GL_TEXTURE_2D));

        glTexImage2D(
            GL_TEXTURE_2D,
            0,
            GL_RGBA,
            16,
            16,
            0,
            GL_RGBA,
            GL_UNSIGNED_BYTE,
            std::ptr::null(),
        );
    }
}
