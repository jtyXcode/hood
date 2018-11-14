use gl_sys::{GLboolean, GLenum, GLint, GLintptr, GLsizei, GLsizeiptr, GLuint, GLvoid};

use context::{self, HUB};

use hal_registry;
use {HalBuffer, HalMemory};

// todo: auto increase value like C enum
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum BufferType {
    Array = 0,
    Element = 1,
    Total = 2,
}

#[derive(Debug, Default)]
pub struct Buffer {
    pub usage: GLenum,
    pub target: GLenum,
    pub size: GLsizeiptr,
    // todo: remove data field?
    pub data: Option<Vec<u8>>,
    pub is_allocated: bool,

    pub memory: Option<HalMemory>,
    pub buffer: Option<HalBuffer>,
}

//IndexBufferObject
//TransferDstBufferObject
//TransferSrcBufferObject
//UniformBufferObject
//VertexBufferObject

pub fn hal_buffer_release(buffer: &mut Buffer) {}

pub fn has_hal_buffer(buffer: &Buffer) -> bool {
    buffer.buffer.is_some()
}

fn set_update_index_buffer(target: GLenum, object_ptr: *mut Buffer) {
    use active_object;
    use gl_sys::GL_ELEMENT_ARRAY_BUFFER;

    //    if target == GL_ELEMENT_ARRAY_BUFFER
    //        || (!context::is_nullptr(object_ptr, "object_ptr is nullptr")
    //            && is_index_buffer(active_object::get_object_mut(object_ptr)))
    //    {
    //        //        HUB.pipeline.lock();
    //    }
}

fn is_index_buffer(buffer: &Buffer) -> bool {
    true
}

fn validate_buffer_target(value: GLenum) -> bool {
    use gl_sys::{GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER};
    context::validate_target(value, &[GL_ARRAY_BUFFER, GL_ELEMENT_ARRAY_BUFFER])
}

fn validate_buffer_pname(value: GLenum) -> bool {
    use gl_sys::{GL_BUFFER_SIZE, GL_BUFFER_USAGE};
    context::validate_pname(value, &[GL_BUFFER_SIZE, GL_BUFFER_USAGE])
}

fn validate_buffer_usage(value: GLenum) -> bool {
    use gl_sys::{GL_DYNAMIC_DRAW, GL_STATIC_DRAW, GL_STREAM_DRAW};
    context::validate_pname(value, &[GL_STREAM_DRAW, GL_STATIC_DRAW, GL_DYNAMIC_DRAW])
}

fn validate_buffer_size(value: GLsizeiptr) -> bool {
    context::validate_invalid_value(value, |&size| size < 0, "invalid value:")
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn glBindBuffer(target: GLenum, buffer: GLuint) {
    info!("glBindBuffer(target = {:?}, buffer = {})", target, buffer);

    use std::ptr;

    context::bind_target_object_name(
        target,
        buffer,
        validate_buffer_target,
        ptr::null_mut(),
        |object_name| {
            let mut pool_guard = HUB.buffer_pool.lock();
            let object = pool_guard.get_object_mut(object_name);
            object.target = target;
            object as *mut Buffer
        },
        |object_ptr| {
            context::set_active_buffer(target, buffer, object_ptr);
            set_update_index_buffer(target, object_ptr);
        },
    );
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn glBufferData(target: GLenum, size: GLsizeiptr, data: *const GLvoid, usage: GLenum) {
    info!(
        "glBufferData(target = {:?}, size = {}, data = {:p}, usage = {:?})",
        target, size, data, usage
    );

    context::object_upload_data(
        || validate_buffer_target(target) && validate_buffer_usage(usage) && validate_buffer_size(size),
        context::get_active_buffer(target),
        |object| {
            object.usage = usage;
            let allocated_size = object.size;
            let data_is_nullptr = context::is_nullptr(data as *mut Buffer, "");
            if (!data_is_nullptr && has_hal_buffer(object)) || (data_is_nullptr && allocated_size > 0 && size != allocated_size) {
                hal_buffer_release(object);
            }
            true
        },
        |object| hal_registry::hal_buffer_allocate(), /* todo: missing process logic*/
        || {},
    );
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn glBufferSubData(target: GLenum, offset: GLintptr, size: GLsizeiptr, data: *const GLvoid) {
    info!(
        "glBufferSubData(target = {:?}, offset = {:?},size = {}, data = {:p})",
        target, offset, size, data
    );
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint) {
    info!("glDeleteBuffers(n = {}, buffers = {:p})", n, buffers);

//    context::delete_objects(n, buffers);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) {
    info!(
        "glGetBufferParameteriv(target: {:?}, pname: {:?}, params: {:p})",
        target, pname, params
    );

    if !validate_buffer_target(target) || !validate_buffer_pname(pname) {
        return;
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {
    info!("glGenBuffers(n = {}, buffers = {:p})", n, buffers);

    context::generate_objects(n, buffers, &HUB.buffer_pool);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn glIsBuffer(buffer: GLuint) -> GLboolean {
    info!("glIsBuffer(buffer = {})", buffer);

    context::is_valid_object(buffer, &HUB.buffer_pool)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use gl_sys::*;

    #[test]
    fn test_all_in_one() {
        &*HUB;

        let count = 1;
        let mut buffers = Vec::<GLuint>::with_capacity(count);
        glGenBuffers(count as GLsizei, buffers.as_mut_ptr());
        unsafe { buffers.set_len(count) }
        glBindBuffer(GL_ARRAY_BUFFER, buffers[0]);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindBuffer(GL_ARRAY_BUFFER, buffers[0]);
        glBindBuffer(GL_ARRAY_BUFFER, 0);

        let count_2 = 2;
        let mut buffers_2 = Vec::<GLuint>::with_capacity(count_2);
        glGenBuffers(count_2 as GLsizei, buffers_2.as_mut_ptr());
        unsafe { buffers_2.set_len(count_2) }
        glBindBuffer(GL_ARRAY_BUFFER, buffers_2[1]);

        glBindBuffer(GL_ARRAY_BUFFER, buffers[0]);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindBuffer(GL_ARRAY_BUFFER, 0);
        glBindBuffer(GL_ARRAY_BUFFER, 0);

        glBindBuffer(GL_ARRAY_BUFFER, buffers_2[0]);

        {
            let guard = HUB.active_buffer[0].lock();
            let texture_object = unsafe { &*guard.ptr };
            println!("test block 2-> {:?}", texture_object);
        }

        glBindBuffer(GL_ARRAY_BUFFER, buffers[0]);
        glBufferData(GL_ARRAY_BUFFER, 100, std::ptr::null(), GL_STATIC_DRAW);
    }
}
