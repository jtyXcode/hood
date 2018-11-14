/// ref: https://github.com/gfx-rs/wgpu/tree/master/wgpu-native/src/registry

#[cfg(not(feature = "remote"))]
mod local;
#[cfg(feature = "remote")]
mod remote;

#[cfg(not(feature = "remote"))]
pub use self::local::{Id, ItemsGuard, Registry as ConcreteRegistry};
#[cfg(feature = "remote")]
pub use self::remote::{Id, ItemsGuard, Registry as ConcreteRegistry};

//use buffer::Buffer;
//use buffer::BufferManager;
//use buffer::BufferType;
use gl_sys::{GLenum, GLuint, GL_NO_ERROR};
use utilities::{Error, ErrorPtr, CURRENT_ERROR_PTR};
use {ErrorHandle, FrameBufferHandle};

type Item<'a, T> = &'a T;
type ItemMut<'a, T> = &'a mut T;

pub trait Registry<T>: Default {
    fn lock(&self) -> ItemsGuard<T>;
}

pub trait Items<T> {
    fn register(&mut self, handle: T) -> Id;
    fn get(&self, id: Id) -> Item<T>;
    fn get_mut(&mut self, id: Id) -> ItemMut<T>;
    fn take(&mut self, id: Id) -> T;
}

pub type ContextPtr = Id;





//    pub current_buffer: [ActiveObject<Buffer>; BufferType::Total as usize],
//    pub buffer_pool: ObjectPool<Buffer>,
//
//    pub shader_manager: ShaderManager,
//    pub program_manager: ProgramManager,
//    pub texture_manager: TextureManager,
//    pub render_buffer_manager: RenderBufferManager,
//    pub frame_buffer_manager: FrameBufferManager,
//    pub fragment_manager: FragmentManager,
//    pub rasterization_manager: RasterizationManager,
//    pub input_assembly_manager: InputAssemblyManager,
//    pub pixel_storage_manager: PixelStorageManager,
//    pub viewport_manager: ViewportManager,

#[derive(Debug)]
pub struct ActiveObjectEx<'a, T> {
    pub name: GLuint,
    pub ptr: &'a mut T,
}

impl<'a, T> Default for ActiveObjectEx<'a, T> {
    fn default() -> Self {
        unimplemented!()
    }
}

#[derive(Debug, Default)]
pub struct ShaderManager {}

#[derive(Debug, Default)]
pub struct ProgramManager {}

#[derive(Debug, Default)]
pub struct RenderBufferManager {}

#[derive(Debug, Default)]
pub struct FrameBufferManager {}

#[derive(Debug, Default)]
pub struct FragmentManager {}

#[derive(Debug, Default)]
pub struct RasterizationManager {}

#[derive(Debug, Default)]
pub struct PixelStorageManager {}

#[derive(Debug, Default)]
pub struct InputAssemblyManager {}

#[derive(Debug, Default)]
pub struct ViewportManager {}
