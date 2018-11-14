use parking_lot::Mutex;

use std;
use std::ptr;

use gl_sys::{
    GLboolean, GLenum, GLsizei, GLuint, GL_FALSE, GL_INVALID_ENUM, GL_INVALID_OPERATION, GL_INVALID_VALUE, GL_OUT_OF_MEMORY,
    GL_TEXTURE_2D, GL_TEXTURE_CUBE_MAP, GL_TRUE,
};

use buffer::{self, Buffer};
use frame_buffer::FrameBuffer;
use pixel_operations::PixelStorageState;
use program::Program;
use rasterization_state::RasterizationState;
use render_buffer::RenderBuffer;
use shader::Shader;
use texture::{self, Texture};
use utilities::{self, record_error};
use viewport_transformation::ViewportTransformation;

use active_object::{get_object_mut, ActiveObject};
use object_pool::ObjectPool;

type MutexObjectPool<T> = Mutex<ObjectPool<T>>;
type MutexActiveObject<T> = Mutex<ActiveObject<T>>;

// todo: refactor to AnyMap for Mutex<T>
#[derive(Debug, Default)]
pub struct Context {
    pub(crate) error: Mutex<utilities::Error>,

    pub(crate) texture_pool: MutexObjectPool<Texture>,
    pub(crate) active_texture: [[MutexActiveObject<Texture>; 32]; texture::TextureType::Total as usize], // todo: split to 8(vertex) + 32(fragment)
    pub(crate) active_texture_unit: Mutex<texture::TextureUnit>,

    pub(crate) buffer_pool: MutexObjectPool<Buffer>,
    pub(crate) active_buffer: [MutexActiveObject<Buffer>; buffer::BufferType::Total as usize],

    pub(crate) render_buffer_pool: MutexObjectPool<RenderBuffer>,
    pub(crate) active_render_buffer: MutexActiveObject<RenderBuffer>,

    pub(crate) frame_buffer_pool: MutexObjectPool<FrameBuffer>,
    pub(crate) active_frame_buffer: MutexActiveObject<FrameBuffer>,

    pub(crate) shader_pool: MutexObjectPool<Shader>,

    pub(crate) program_pool: MutexObjectPool<Program>,
    pub(crate) active_program: MutexActiveObject<Program>,

    pub(crate) rasterization_state: Mutex<RasterizationState>,
    pub(crate) pixel_storage_state: Mutex<PixelStorageState>,
    pub(crate) viewport_state: Mutex<ViewportTransformation>,
    //    pub(crate) input_assembly_State: Mutex<InputAssemblyState>,
}

//#[derive(Debug, Default)]
//pub struct InputAssemblyState {}

lazy_static! {
    pub(crate) static ref HUB: Context = {
        init_log();
        Context::default()
    };
}

#[inline(always)]
fn init_log() {
    let env = env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "trace");
    env_logger::Builder::from_env(env).init();
    info!("logger initialized.");
}

pub fn generate_objects<T: Default>(count: GLsizei, objects_ptr: *mut GLuint, pool: &MutexObjectPool<T>) {
    error!("generate_objects");

    if !validate_objects_ptr(count, objects_ptr) {
        return;
    }

    let objects = unsafe { std::slice::from_raw_parts_mut(objects_ptr, count as usize) };
    for buffer in objects {
        *buffer = pool.lock().allocate();
    }
}

pub(crate) fn delete_objects<T, UPS>(
    count: GLsizei,
    objects_ptr: *const GLuint,
    pool: &MutexObjectPool<T>,
    active_object: &mut MutexActiveObject<T>,
    need_forward_finish_drawing: bool,
    update_state: UPS,
) where
    T: Default,
    UPS: Fn(&mut T),
{
    if !validate_objects_ptr(count, objects_ptr) {
        return;
    }

    if need_forward_finish_drawing {
        // todo: glFinish()
    }

    let pool_guard = &mut *pool.lock();
    let object_names: &[GLuint] = unsafe { std::slice::from_raw_parts(objects_ptr, count as usize) };
    for &object_name in object_names {
        if !pool_guard.has_object(object_name) {
            debug!("name {} that do not correspond to an existing object", object_name);
            continue;
        }
        {
            let object = pool_guard.get_object_mut(object_name);
            update_state(object);
            active_object.lock().name = 0;
            active_object.lock().ptr = ptr::null_mut();
        }
        pool_guard.deallocate(object_name);
    }
}

#[inline(always)]
fn validate_objects_ptr(count: GLsizei, objects_ptr: *const GLuint) -> bool {
    debug!("validate_objects_ptr");

    if count < 0 {
        error!("invalid value: {}", count);
        utilities::record_error(GL_INVALID_VALUE);
        return false;
    }

    if objects_ptr == ptr::null() {
        warn!("ptr is nullptr");
        return false;
    }

    true
}

pub fn is_valid_object<T: Default>(object_name: GLuint, pool: &MutexObjectPool<T>) -> GLboolean {
    if object_name == 0 {
        warn!("object name is 0, not a valid value");
        return GL_FALSE;
    }

    pool.lock().has_object(object_name).as_gl_bool_value()
}

pub(crate) fn get_default_frame_buffer(target: GLenum) -> *mut FrameBuffer {
    ptr::null_mut()
}

pub(crate) fn get_default_texture(target: GLenum) -> *mut Texture {
    //    match target {
    //        GL_TEXTURE_2D => self.default_texture_2d,
    //        GL_TEXTURE_CUBE_MAP => self.default_texture_cube_map,
    //        _ => {
    //            error!("invalid value");
    //            self.default_texture_2d
    //        }
    //    }
    //    HUB.active_texture.lock().ptr
    ptr::null_mut()
}

// todo: refactor
pub(crate) fn set_active_object<T: Default>(object_name: GLuint, object_ptr: *mut T, current_object: &MutexActiveObject<T>) {
    //    current_object

}

//pub(crate) fn get_texture_mut<'a>(texture: GLuint) -> &'a mut Texture {

//    get_object_mut(texture,&HUB.texture_pool)

//    let mut guard = HUB.texture_pool.lock();
//    guard.get_object_mut(texture)
//}

//pub(crate) fn get_object_mut<'a, T>(object_name: GLuint,pool: &'static MutexObjectPool<T>) -> &'a mut T where
//    T: Default,{
//    let mut guard = pool.lock();
//    guard.get_object_mut(object_name)
//}

/// Both buffer object and texture object supports multiple target
pub(crate) fn set_active_buffer(target: GLenum, name: GLuint, object_ptr: *mut Buffer) {
    let mut guard = HUB.active_buffer[buffer_target_to_index(target)].lock();
    guard.name = name;
    guard.ptr = object_ptr;
}

pub(crate) fn set_active_render_buffer(_target: GLenum, name: GLuint, object_ptr: *mut RenderBuffer) {
    let mut guard = HUB.active_render_buffer.lock();
    guard.name = name;
    guard.ptr = object_ptr;
}

pub(crate) fn set_active_texture(target: GLenum, name: GLuint, object_ptr: *mut Texture) {
    debug!("set_active_texture");
    let mut guard =
        HUB.active_texture[texture_target_to_index(target)][texture_unit_to_index(HUB.active_texture_unit.lock().raw)].lock();
    guard.name = name;
    guard.ptr = object_ptr;
}

pub(crate) fn get_active_texture(target: GLenum) -> ActiveObject<Texture> {
    debug!("get_active_texture");

    HUB.active_texture[texture_target_to_index(target)][texture_unit_to_index(HUB.active_texture_unit.lock().raw)]
        .lock()
        .clone()
}

#[inline]
pub(crate) fn get_active_buffer(target: GLenum) -> ActiveObject<Buffer> {
    HUB.active_buffer[buffer_target_to_index(target)].lock().clone()
}

#[inline]
pub(crate) fn bind_target_object_name<T, PRL, OPT, PLG>(
    target: GLenum,
    object_name: GLuint,
    params_check: PRL,
    default_object_ptr: *mut T,
    modify_object: OPT,
    set_active_state: PLG,
) where
    T: Default,
    PRL: Fn(GLenum) -> bool,
    OPT: Fn(GLuint) -> *mut T,
    PLG: Fn(*mut T),
{
    if !params_check(target) {
        return;
    }

    let object_ptr = if 0 == object_name {
        debug!("return default_object_ptr");
        default_object_ptr
    } else {
        let result = modify_object(object_name);
        debug!("result is {:?}", result);
        if is_nullptr(result, "") {
            /* Texture object only for now */
            error!("old target of object does not match new target");
            record_error(GL_INVALID_OPERATION);
            return;
        }
        result
    };

    set_active_state(object_ptr)
}

#[inline(always)]
pub(crate) fn is_nullptr<T>(ptr: *mut T, message: &str) -> bool {
    if ptr == ptr::null_mut() {
        warn!("{}", message);
        true
    } else {
        false
    }
}

pub fn validate_parameter(pname: GLenum, required_values: &[GLenum]) -> bool {
    //    validate_parameters(pname, &[GL_BUFFER_SIZE, GL_BUFFER_USAGE])
    //    if pname != GL_BUFFER_SIZE && pname != GL_BUFFER_USAGE {
    //        error!("invalid value: {:?}", pname);
    //        record_error(GL_INVALID_ENUM);
    //        false
    //    } else {
    true
    //    }
}

/// `record_error(GL_INVALID_ENUM)` when match failed and return `false`, otherwise `true`
pub(crate) fn validate_parameters(param: GLenum, required_values: &[GLenum]) -> bool {
    for value in required_values {
        if param != *value {
            error!("invalid pname: {}", param);
            record_error(GL_INVALID_ENUM);
            return false;
        }
    }
    true
}

#[inline]
pub(crate) fn validate_target(target: GLenum, required_values: &[GLenum]) -> bool {
    validate_invalid_enum(target, required_values, "invalid target")
}

#[inline]
pub(crate) fn validate_pname(target: GLenum, required_values: &[GLenum]) -> bool {
    validate_invalid_enum(target, required_values, "invalid pname")
}

#[inline]
pub(crate) fn validate_params(target: GLenum, required_values: &[GLenum]) -> bool {
    validate_invalid_enum(target, required_values, "invalid params")
}

/// `record_error(GL_INVALID_ENUM)` when match failed and return `false`, otherwise `true`
#[inline]
pub(crate) fn validate_invalid_enum(value: GLenum, required_values: &[GLenum], message: &str) -> bool {
    find_value_in_slice(value, required_values, |param| {
        error!("{}: {}", message, param);
        record_error(GL_INVALID_ENUM);
    })
}

/// `record_error(GL_INVALID_VALUE)` when `operation` failed and return `false`, otherwise `true`
#[inline]
pub(crate) fn validate_invalid_value<T, OPT>(value: T, operation: OPT, message: &str) -> bool
where
    T: PartialEq,
    T: std::fmt::Debug,
    OPT: Fn(&T) -> bool,
{
    if operation(&value) {
        error!("{}: {:?}", message, value);
        record_error(GL_INVALID_VALUE);
        false
    } else {
        true
    }
}

/// Return true if `value` is one of the `required_values`, otherwise false
#[inline(always)]
pub(crate) fn find_value_in_slice<F>(value: GLenum, required_values: &[GLenum], operation_when_false: F) -> bool
where
    F: Fn(GLenum),
{
    if !required_values.into_iter().any(|&value| value == value) {
        operation_when_false(value);
        false
    } else {
        true
    }
}

// todo: refactor `object_upload_data` to accept `cleanup()` or insert `allocate_guard()` before `epilogue()`
/// Allocate memory and copy data passed from user to it.
pub(crate) fn object_upload_data<'a, T, PRL, OPT, PLG, UPS>(
    params_check: PRL,
    mut active_object: ActiveObject<T>,
    operation: OPT,
    allocate_guard: PLG,
    update_state: UPS,
) where
    T: Default,
    T: 'a,
    PRL: Fn() -> bool,
    OPT: Fn(&'a mut T) -> bool,
    PLG: Fn(&'a mut T) -> bool,
    UPS: Fn(),
{
    if !params_check() {
        return;
    }

    if 0 == active_object.name {
        error!("current active object is default value");
        record_error(GL_INVALID_OPERATION);
        return;
    }

    if !operation(get_object_mut(&mut active_object).unwrap()) {
        debug!("operation failed, return now");
        return;
    }

    if !allocate_guard(get_object_mut(&mut active_object).unwrap()) {
        error!("out of memory when allocating underlying data structures");
        record_error(GL_OUT_OF_MEMORY);
        return;
    }

    update_state();
}

//
//fn get_active_object<T: Default>() ->ActiveObject<Any>{
//    use std::any::{Any, TypeId};
//
//    let obj = if TypeId::of::<T>() == TypeId::of::<Texture>() {
//        HUB.active_texture.lock().clone()
//    } else if TypeId::of::<T>() == TypeId::of::<Program>() {
//        HUB.active_program.lock().clone()
//    } else if TypeId::of::<T>() == TypeId::of::<Buffer>() {
//        HUB.active_buffer.lock().clone()
//    } else if TypeId::of::<T>() == TypeId::of::<RenderBuffer>() {
//        HUB.active_render_buffer.lock().clone()
//    } else if TypeId::of::<T>() == TypeId::of::<FrameBuffer>() {
//        HUB.active_frame_buffer.lock().clone()
//    } else {
//        error!("invalid T");
//        HUB.active_texture.lock().clone()
//    };
//    obj
//}

#[inline(always)]
fn buffer_target_to_index(target: GLenum) -> usize {
    use gl_sys::{GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER};

    match target {
        GL_ARRAY_BUFFER => 0,
        GL_ELEMENT_ARRAY_BUFFER => 1,
        _ => {
            error!("invalid buffer target {:?}", target);
            0
        }
    }
}

#[inline(always)]
fn texture_target_to_index(target: GLenum) -> usize {
    match target {
        GL_TEXTURE_2D => 0,
        GL_TEXTURE_CUBE_MAP => 1,
        _ => {
            error!("invalid texture target {:?}", target);
            0
        }
    }
}

#[inline(always)]
fn texture_unit_to_index(texture: GLenum) -> usize {
    use gl_sys::GL_TEXTURE0;
    (texture - GL_TEXTURE0) as usize
}

#[inline(always)]
pub(crate) fn is_default_frame_buffer(object_name: GLuint) -> bool {
    object_name == 0
}

/// Builds a `GLboolean`.
pub(crate) trait AsGlBoolValue {
    fn as_gl_bool_value(&self) -> GLboolean;
}

impl AsGlBoolValue for bool {
    /// Builds a `GLboolean`.
    #[inline]
    fn as_gl_bool_value(&self) -> GLboolean {
        match *self {
            true => GL_TRUE,
            false => GL_FALSE,
        }
    }
}
