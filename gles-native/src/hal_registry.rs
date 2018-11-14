pub fn hal_buffer_allocate() -> bool {
    debug!("hal_buffer_allocate: fake allocate failed");
    true
}



pub(crate) fn hal_sampler_release() {}
pub(crate) fn hal_image_view_release() {}
pub(crate) fn hal_image_release() {}
pub(crate) fn hal_memory_release() {}

pub fn hal_texture_create() -> bool {
    trace!("hal_texture_create()");

    hal_sampler_release();
    hal_image_view_release();
    hal_image_release();
    hal_memory_release();

    if !hal_image_create() {
        return false;
    }

    //        if mip_level_count > 1 {
    //
    //        }

    if !hal_memory_allocate() {
        hal_image_release();
        return false;
    }

    if !hal_image_view_create() {
//        image_release();
//        memory_release();
        return false;
    }
    true
}





pub fn hal_memory_allocate() -> bool {
    true
}

pub fn hal_image_create() -> bool {
    true
}

pub fn hal_image_view_create() -> bool {
    true
}

pub fn hal_memory_create() -> bool {
    true
}