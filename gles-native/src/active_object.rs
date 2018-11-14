use std::ptr;

use gl_sys::GLuint;

#[derive(Debug, Copy)]
pub(crate) struct ActiveObject<T> {
    pub name: GLuint,
    pub ptr: *mut T,
}

impl<T> Default for ActiveObject<T> {
    fn default() -> Self {
        Self {
            name: 0,
            ptr: ptr::null_mut(),
        }
    }
}

unsafe impl<T> std::marker::Send for ActiveObject<T> {}

impl<T> std::clone::Clone for ActiveObject<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            ptr: self.ptr,
        }
    }
}

/// Call it once within a function for the same `T`
pub(crate) fn get_object_mut<'a, T>(object: &mut ActiveObject<T>) -> Option<&'a mut T> {
    let ptr = object.ptr;
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { &mut *ptr })
    }
}
