use gl_sys::GLuint;
use std::ptr;

#[derive(Debug)]
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
