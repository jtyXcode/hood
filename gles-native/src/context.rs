use active_object::ActiveObject;
use object_pool::ObjectPool;
use parking_lot::Mutex;
use texture::Texture;
use buffer::Buffer;
use utilities;

#[derive(Debug, Default)]
pub struct Context {
    pub(crate) error: Mutex<utilities::Error>,

    pub(crate) texture_pool: Mutex<ObjectPool<Texture>>,
    pub(crate) current_texture_object: Mutex<ActiveObject<Texture>>,

    pub(crate) buffer_pool: Mutex<ObjectPool<Buffer>>,
    pub(crate) current_buffer_object: Mutex<ActiveObject<Buffer>>,
}

lazy_static! {
    pub(crate) static ref HUB: Context = Context::default();
}
