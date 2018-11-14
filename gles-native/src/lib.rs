#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate libc;

#[cfg(not(any(feature = "vulkan", feature = "metal")))]
extern crate gfx_backend_empty as back;
#[cfg(feature = "metal")]
extern crate gfx_backend_metal as back;
#[cfg(feature = "vulkan")]
extern crate gfx_backend_vulkan as back;

extern crate gfx_hal as hal;
extern crate parking_lot;

use back::Backend as B;

mod gl_sys;

//mod registry;

mod texture;
mod buffer;
//mod render_buffer;
mod frame_buffer;
mod context;
mod utilities;

mod object_pool;
mod active_object;

//use registry::Id;
//
//type ErrorHandle = utilities::Error;
//type FrameBufferHandle = frame_buffer::FrameBuffer<B>;
