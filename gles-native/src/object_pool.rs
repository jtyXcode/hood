use gl_sys::GLuint;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub struct ObjectPool<T> {
    index: GLuint,
    map: BTreeMap<GLuint, Box<T>>,
}

impl<T> ObjectPool<T>
where
    T: Default,
{
    pub fn allocate(&mut self) -> GLuint {
        self.index += 1;
        self.index
    }

    pub fn deallocate(&mut self, index: GLuint) -> bool {
        self.map.remove(&index).is_some()
    }

    pub fn get_object_mut(&mut self, index: GLuint) -> &mut T {
        // NOTE: special case but it is legal. GL spec says that `glBind*` lets you create or use a named * object.
        if self.index < index {
            self.index = index;
        }

        let map = &mut self.map;
        if map.contains_key(&index) {
            map.get_mut(&index).unwrap()
        } else {
            let instance = Box::new(T::default());
            map.insert(index, instance);
            map.get_mut(&index).unwrap()
        }
    }

    #[inline]
    pub fn is_object_exists(&self, index: GLuint) -> bool {
        self.map.contains_key(&index)
    }

    pub fn get_objects_mut(&mut self) -> &mut BTreeMap<GLuint, Box<T>> {
        &mut self.map
    }
}
