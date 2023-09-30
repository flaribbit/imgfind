use std::ptr;

use libheif_sys as lh;

#[test]
fn create_heic_context() {
    unsafe {
        lh::heif_init(ptr::null_mut());

        let ctx = lh::heif_context_alloc();
        assert!(!ctx.is_null());
        lh::heif_context_free(ctx);

        lh::heif_deinit();
    }
}
