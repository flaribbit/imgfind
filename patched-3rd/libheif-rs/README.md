# libheif-rs

Safe wrapper around the libheif-sys crate for parsing heif/heic files.

[CHANGELOG](https://github.com/Cykooz/libheif-rs/blob/master/CHANGELOG.md)

## System dependencies

- `libheif-dev` >= 1.16.0
- `clang` - to generate rust bindings for `libheif` in `libheif-sys` crate.
  [See bindgen requirements.](https://rust-lang.github.io/rust-bindgen/requirements.html)

`clang` wouldn't be needed if you disable `use-bindgen` feature.
In this case the pre-generated file `bindings.rs` will be used  
instead of generating it on the fly with help of `binden` crate.

Warning: `bindings.rs` file was generated under x64 linux and may
not work as expected under x32 architectures or other operating systems.

### Linux

Crate `libheif-sys` uses `pkg-confing` to find installed `libheif`.

### Windows

Crate `libheif-sys` uses [vcpkg crate](https://crates.io/crates/vcpkg)
to find `libheif` installed with help of `vcpkg`.

You can use [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg)
to install `libheif` with help of `cargo` command:

```shell
cargo vcpkg -v build
```

`cargo-vcpkg` can fetch and build a `vcpkg` installation of required
packages from scratch. It merges package requirements specified in
the `Cargo.toml` of crates in the dependency tree.

PS: I tried build `libheif` for Windows, but `libheif-rs` tests
filed because `libheif` does not have any encoder plugins available.

## Examples

### Read HEIF file

```rust
use libheif_rs::{
    Channel, RgbChroma, ColorSpace, HeifContext, Result, ItemId, LibHeif
};

fn main() -> Result<()> {
    let lib_heif = LibHeif::new();
    let ctx = HeifContext::read_from_file("./data/test.heif")?;
    let handle = ctx.primary_image_handle()?;
    assert_eq!(handle.width(), 1652);
    assert_eq!(handle.height(), 1791);

    // Get Exif
    let mut meta_ids: Vec<ItemId> = vec![0; 1];
    let count = handle.metadata_block_ids(&mut meta_ids, b"Exif");
    assert_eq!(count, 1);
    let exif: Vec<u8> = handle.metadata(meta_ids[0])?;

    // Decode the image
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::Rgb)));
    assert_eq!(image.width(), 1652);
    assert_eq!(image.height(), 1791);

    // Scale the image
    let small_img = image.scale(1024, 800, None)?;
    assert_eq!(small_img.width(), 1024);
    assert_eq!(small_img.height(), 800);

    // Get "pixels"
    let planes = small_img.planes();
    let interleaved_plane = planes.interleaved.unwrap();
    assert_eq!(interleaved_plane.width, 1024);
    assert_eq!(interleaved_plane.height, 800);
    assert!(!interleaved_plane.data.is_empty());
    assert!(interleaved_plane.stride > 0);

    Ok(())
}
```

### Write HEIF file

```rust
use tempfile::NamedTempFile;
use libheif_rs::{
    Channel, RgbChroma, ColorSpace, CompressionFormat, EncoderQuality, 
    HeifContext, Image, Result, LibHeif
};

fn main() -> Result<()> {
    let width = 640;
    let height = 480;

    let mut image = Image::new(width, height, ColorSpace::Rgb(RgbChroma::C444))?;

    image.create_plane(Channel::R, width, height, 8)?;
    image.create_plane(Channel::G, width, height, 8)?;
    image.create_plane(Channel::B, width, height, 8)?;

    let planes = image.planes_mut();
    let plane_r = planes.r.unwrap();
    let stride = plane_r.stride;

    let data_r = plane_r.data;
    let data_g = planes.g.unwrap().data;
    let data_b = planes.b.unwrap().data;

    // Fill data of planes by some "pixels"
    for y in 0..height {
        let mut row_start = stride * y as usize;
        for x in 0..width {
            let color = (x * y) as u32;
            data_r[row_start] = ((color & 0x00_ff_00_00) >> 16) as u8;
            data_g[row_start] = ((color & 0x00_00_ff_00) >> 8) as u8;
            data_b[row_start] = (color & 0x00_00_00_ff) as u8;
            row_start += 1;
        }
    }

    // Encode image and save it into file.
    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;
    encoder.set_quality(EncoderQuality::LossLess)?;
    context.encode_image(&image, &mut encoder, None)?;

    let tmp_file = NamedTempFile::new().unwrap();
    context.write_to_file(tmp_file.path().to_str().unwrap())?;

    Ok(())
}
```
