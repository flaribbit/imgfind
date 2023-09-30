# libheif-sys is bindings to libheif

[CHANGELOG](https://github.com/Cykooz/libheif-sys/blob/master/CHANGELOG.md)

## System dependencies

- `libheif-dev` >= 1.16.0
- `clang` - to generate rust bindings for `libheif`.
  [See bindgen requirements.](https://rust-lang.github.io/rust-bindgen/requirements.html)

`clang` wouldn't be needed if you disable `use-bindgen` feature.
In this case the pre-generated file `bindings.rs` will be used
instead of generating it on the fly with help of `binden` crate.

Warning: `bindings.rs` file was generated under x64 linux and may
not work as expected under x32 architectures or other operating systems.

### Linux

The crate uses `pkg-confing` to find installed `libheif`.

### Windows

The crate uses [vcpkg crate](https://crates.io/crates/vcpkg)
to find `libheif` installed with help of `vcpkg`.

You can use [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg)
to install `libheif` with help of `cargo` command:

```shell
cargo vcpkg -v build
```

`cargo-vcpkg` can fetch and build a `vcpkg` installation of required
packages from scratch. It merges package requirements specified in
the `Cargo.toml` of crates in the dependency tree.

## Example of reading and decoding of HEIF-image

```rust
use std::ffi;
use std::ptr;

use libheif_sys as lh;

#[test]
fn read_and_decode_heic_file() {
    unsafe {
        lh::heif_init(ptr::null_mut());

        let ctx = lh::heif_context_alloc();
        assert_ne!(ctx, ptr::null_mut());

        let c_name = ffi::CString::new("data/test.heif").unwrap();
        let err = lh::heif_context_read_from_file(ctx, c_name.as_ptr(), ptr::null());
        assert_eq!(err.code, 0);

        let mut handle = ptr::null_mut();
        let err = lh::heif_context_get_primary_image_handle(ctx, &mut handle);
        assert_eq!(err.code, 0);
        assert!(!handle.is_null());

        let width = lh::heif_image_handle_get_width(handle);
        assert_eq!(width, 4032);
        let height = lh::heif_image_handle_get_height(handle);
        assert_eq!(height, 3024);

        let options = lh::heif_decoding_options_alloc();

        let mut image = ptr::null_mut();
        let err = lh::heif_decode_image(
            handle,
            &mut image,
            lh::heif_colorspace_heif_colorspace_RGB,
            lh::heif_chroma_heif_chroma_444,
            options,
        );
        lh::heif_decoding_options_free(options);
        assert_eq!(err.code, 0);
        assert!(!image.is_null());

        let colorspace = lh::heif_image_get_colorspace(image);
        assert_eq!(colorspace, lh::heif_colorspace_heif_colorspace_RGB);
        let chroma_format = lh::heif_image_get_chroma_format(image);
        assert_eq!(chroma_format, lh::heif_chroma_heif_chroma_444);
        let width = lh::heif_image_get_width(image, lh::heif_channel_heif_channel_R);
        assert_eq!(width, 4032);
        let height = lh::heif_image_get_height(image, lh::heif_channel_heif_channel_R);
        assert_eq!(height, 3024);

        lh::heif_context_free(ctx);

        lh::heif_deinit();
    };
}
```
