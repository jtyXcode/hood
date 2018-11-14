#![allow(dead_code, unused_imports, unused_variables)]

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
pub(crate) type HalMemory = <B as hal::Backend>::Memory;
pub(crate) type HalImage = <B as hal::Backend>::Image;
pub(crate) type HalImageView = <B as hal::Backend>::ImageView;
pub(crate) type HalSampler = <B as hal::Backend>::Sampler;
pub(crate) type HalBuffer = <back::Backend as hal::Backend>::Buffer;
pub(crate) type HalFrameBuffer = <back::Backend as hal::Backend>::Framebuffer;
pub(crate) type HalFence = <back::Backend as hal::Backend>::Fence;
pub(crate) type HalRenderPass = <back::Backend as hal::Backend>::RenderPass;
pub(crate) type HalPipelineLayout = <back::Backend as hal::Backend>::PipelineLayout;
pub(crate) type HalPipelineCache = <back::Backend as hal::Backend>::PipelineCache;

/// OpenGL (ES) defined types, constants
mod gl_sys;

/// OpenGL (ES) defined functions implementation split into multiple modules
mod buffer;
mod fragment_state;
mod frame_buffer;
mod pixel_operations;
mod program;
mod rasterization_state;
mod render_buffer;
mod rendering;
mod shader;
mod texture;
mod utilities;
mod viewport_transformation;

/// Export OpenGL (ES) defined functions
pub use buffer::*;
pub use fragment_state::*;
pub use frame_buffer::*;
pub use pixel_operations::*;
//pub use program::*;
//pub use rasterization_state::*;
pub use render_buffer::*;
pub use rendering::*;
pub use shader::*;
pub use texture::*;
/// NOTE: only OpenGL (ES) defined functions
pub use utilities::*;
pub use viewport_transformation::*;

/// Entry point to access states and resources
mod context;

/// Infrastructure
mod active_object;
mod object_pool;

mod hal_registry;
