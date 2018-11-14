#![allow(non_snake_case)]

use context::HUB;
use gl_sys::{GLsizei, GLuint};

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum BufferType {
    Array = 0,
    Element = 1,
    Total = 2,
}

#[derive(Debug, Default)]
pub struct Buffer {}

#[no_mangle]
pub extern "C" fn glGenBuffers(n: GLsizei, buffers: *mut GLuint) {}
