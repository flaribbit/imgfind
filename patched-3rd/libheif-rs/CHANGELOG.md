# Change Log

## [0.22.0] - 2023-09-12

### Added

- Added method `assign_thumbnail` for `HeifContext` structure.
- **BREAKING**: Added default feature `use-bindgen` to control that type of 
  binding will be used by `libheif-sys` crate (pre-generated or
  generated on the fly by `bindgen`)

### Fixed

- Fixed creating instance of `ColorProfileType` from instance of 
  `heif_color_profile_type` on Windows.

## [0.21.0] - 2023-09-08

### Added

- Added `ColorConversionOptions` structure.
- Added `DecodingOptions` methods: 
  - `decoder_id` 
  - `set_decoder_id`
  - `color_conversion_options` 
  - `set_color_conversion_options`
- Added `EncodingOptions` methods:
  - `color_conversion_options`
  - `set_color_conversion_options`
- Added enums: 
  - `ChromaDownsamplingAlgorithm`
  - `ChromaUpsamplingAlgorithm`
- Added `DecoderDescriptor` structure.
- Added method `decoder_descriptors` for `LibHeif` structure.
- Added method `item_id` for `ImageHandle` structure.
- Added `HeifContext` methods:
  - `top_level_image_handles`
  - `encode_thumbnail`
- Added `Image` methods: 
  - `pixel_aspect_ratio`
  - `set_pixel_aspect_ratio`
- **BREAKING**: Added new values of `CompressionFormat` enum:
  - `Vvc`
  - `Evc`
  - `Jpeg2000`
  - `Uncompressed`
- **BREAKING**: Added new values of `HeifErrorSubCode` enum:
  - `InvalidRegionData`
  - `InvalidProperty`
  - `ItemReferenceCycle`
  - `EncoderInitialization`
  - `EncoderEncoding`
  - `EncoderCleanup`
  - `TooManyRegions`
- Added support of [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg)
  to install `libheif` with help of `cargo`.

### Changed

- `libheif-sys` updated to version 1.16.
- **BREAKING**: Enums `HeifErrorCode`, `HeifErrorSubCode`, `CompressionFormat`
  marked as `non_exhaustive`

### Fixed

- Fixed conversion of `profile` argument of `Image.set_color_profile_raw`
  method into C-version of this argument.

## [0.20.0] - 2023-06-05

### Fixed

- **BREAKING**: Added lifetime for `HeifContext` structure to
  exclude "use after free" in case it is created from `&[u8]`
  ([#13](https://github.com/Cykooz/libheif-rs/issues/13)).

## [0.19.2] - 2023-03-22

### Added

- Added method `HeifContext::set_max_decoding_threads()`.

## [0.19.1] - 2023-03-19

### Fixed

- Fixed `path_to_cstring()` function for building on Windows
  ([#11](https://github.com/Cykooz/libheif-rs/issues/11)).

## [0.19.0] - 2023-03-12

### Added

- Added `LibHeif` structure to initialise internals of `libheif`, load
  plugins, get instances of encoders, decode `ImageHandle` into `Image`,
  get version of linked `libheif`.
- Added `EncoderDescriptor` structure that describe some properties of
  encoder and  may be used to get `Encoder` instance from `LibHeif`.
- Added `EncodingOptions::new()` method.

### Changed

- **BREAKING**: Removed method `HeifContext::encoder_for_format()`. Use
  `LibHeif::encoder_for_format()` method instead.
- **BREAKING**: Removed method `ImageHandle::decode()`. Use
  `LibHeif::decode()` method instead.
- **BREAKING**: Lifetime of `Encoder` structure now bounds by lifetime of
  `LibHeif` instance. 
- **BREAKING**: Method `Encoder::name()` now returns `String` instead `&str`.
- Structure `DecodingOptions` was exposed in public interface of the crate.

## [0.18.0] - 2023-03-02

### Added

- Added methods `add_generic_metadata()`, `add_exif_metadata()` 
  and `add_xmp_metadata()` for `HeifContext`.

### Changed

- **BREAKING**: Changed type of argument `type_filer` of methods
  ``ImageHandle::number_of_metadata_blocks()`` and 
  ``ImageHandle::metadata_block_ids()``. Now it must be something that 
  implements `Into<FourCC>`. For example - `b"Exif"`. 
- **BREAKING**: Changed order of arguments of method 
  ``ImageHandle::metadata_block_ids()``.

## [0.17.0] - 2023-02-25

### Added

- Added methods `Image::width()` and `Image::height()` for  receiving
  width and height of the main image channel (Y in YCbCr, or any in RGB).
- Added method `Image::storage_bits_per_pixel()`. This method returns the
  number of bits used for storage of each pixel.
- Added methods `Image::set_color_profile_raw()` and
  `Image::set_color_profile_nclx()`.
- Added field `storage_bits_per_pixel` into structure `Plane`.
- Added method `ColorProfileRaw::new()`.
- Methods `new` and `set_color_primaries` were added into structure `ColorProfileNCLX`.

### Changed

- **BREAKING**: Old methods `Image::width(channel)` and `Image::height(channel)`
  were renamed into `Image::channel_width(channel)`
  and `Image::channel_height(channel)`. The return type of these methods
  was changed from `Result<u32>` to `Option<u32>`.
- **BREAKING**: The return type of method `Image::bits_per_pixel()` was
  changed from `Result<u8>` to `Option<u8>`.
- **BREAKING**: Method `Image::bits_per_pixel()` now returns the number of bits
  used for representing the pixel value, which might be smaller than
  the number of bits used in memory.
- **BREAKING**: Fixed typo in name of field `Plane::bits_pre_pixel`. This field
  was renamed into `Plane::bits_per_pixel`.
- Structures `Plane` and `Planes` were exposed in public interface of the crate.

## [0.16.0] - 2023-02-16

- `libheif-sys` updated to version 1.14.2.
- Added new value of `HeifErrorCode` enum - `PluginLoadingError`. 
- Added new value of `HeifErrorSubCode` enum:
  `UnknownNclxColorPrimaries`, `UnknownNclxTransferCharacteristics`,
  `UnknownNclxMatrixCoefficients`, `UnsupportedHeaderCompressionMethod`,
  `PluginLoadingError`, `PluginIsNotLoaded`, `CannotReadPluginDirectory`,
  `Unknown`.
- Struct `Encoder` exposed in public interface of the crate.
- Added enum `ImageOrientation`.
- Added methods `Encoder::image_orientation()` and `Encoder::set_image_orientation()`.
- Added struct `DecodingOptions`.
- Added type `ColorProfleType`.
- Added structs `ColorProfileRaw` and `ColorProfileNCLX`.
- Added methods `Image::color_profile_raw()` and `Image::color_profile_nclx()`.
- Added methods `ImageHandle::color_profile_raw()` and `ImageHandle::color_profile_nclx()`.

### Breaking changes

- Argument `ignore_transformations` of `ImageHandle::decode()` method was
  replaced on argument `decoding_options`.
- Removed deprecated methods: `ImageHandle::list_of_depth_image_ids()`,
  `ImageHandle::list_of_thumbnail_ids()`, 
  `ImageHandle::list_of_metadata_block_ids()`.

## [0.15.1] - 2022-10-21

- Added method `Encoder::set_parameter_value()`.

## [0.15.0] - 2021-05-12

- `libheif-sys` updated to version 1.12.
- Added new value of `HeifErrorSubCode` enum -
  `WrongTileImagePixelDepth`.  
- Added methods:
  `Image::set_premultiplied_alpha()`, `Image::is_premultiplied_alpha()`,
  `ImageHandle::is_premultiplied_alpha()`.

# [0.14.0] - 2021-03-17

- Added new methods:
  `ImageHandle::depth_image_ids()`, `ImageHandle::thumbnail_ids()`,
  `ImageHandle::metadata_block_ids()`.
- Deprecated some methods:
  `ImageHandle::list_of_depth_image_ids()`, `ImageHandle::list_of_thumbnail_ids()`,
  `ImageHandle::list_of_metadata_block_ids()`.
- Added new methods for getting top level images from `HeifContext`:
  `HeifContext::top_level_image_ids()`, `HeifContext::image_handle()`.

## [0.13.1] - 2021-02-03

- `libheif-sys` updated to version 1.11.
- Added methods: 
  `EncodingOptions::mac_os_compatibility_workaround_no_nclx_profile()`,
  `EncodingOptions::set_mac_os_compatibility_workaround_no_nclx_profile()`.

## [0.13.0] - 2021-01-15

### Breaking changes

- Added new value of `HeifErrorCode` enum -
  `ColorProfileDoesNotExist`.

## [0.12.0] - 2021-01-14

- `libheif-sys` updated to version 1.10.

### Breaking changes

- All fields of `EncodingOptions` struct are made private. Added 
  corresponding methods for access to these fields.
- Method `HeifContext::encode_image()` now returns `Result<ImageHandle>`.

## [0.11.0] - 2020-09-26

- `Image` has marked as `Send`.

## [0.10.0] - 2020-08-29

- `libheif-sys` updated to version 1.8.
- Added new compression format - `CompressionFormat::Av1`.
- Added new values of `HeifErrorSubCode` enum:
  `InvalidFractionalNumber`, `InvalidImageSize`,
  `InvalidPixiBox`,  `NoAv1cBox`.

## [0.9.2] - 2020-08-15

- Implemented `std::error::Error` for `HeifError` (paolobarbolini).

## [0.9.1] - 2020-06-16

- Removed `num`, `num-traits` and `num-derive` from dependencies.
- Added `enumn` as dependency.

## [0.9.0] - 2020-02-24

- Updated versions of dependencies.

### Breaking changes

- Added argument `ignore_transformations` into method `ImageHandle::decode()`.

## [0.8.0] - 2019-10-03

- Added method `ImageHandle::set_primary()`.

### Breaking changes

- Removed dependency from `failure` crate.
- Added type `Result<T>` as alias for `std::result::Result<T, HeifError>`.
- `ImageHandle::is_primary_image` method renamed to `ImageHandle::is_primary`.

## [0.7.0] - 2019-08-28

- Separate enums `ColorSpace` and `Chroma` replaced by one
  complex enum `ColorSpace`.
- `libheif-sys` updated to version 1.5.

## [0.6.0] - 2019-07-17

- Added function `check_file_type` that checks file type by it first bytes.

## [0.5.0] - 2019-07-16

- Renamed some values of `HeifErrorCode` and `HeifErrorSubCode` enums.

## [0.4.0] - 2019-06-28

- Added method `HeifContext::read_from_reader()` to create context 
  form any object that implements the `Reader` trait.

## [0.3.0] - 2019-06-25

- Specified lifetime of `ImageHandle`. Now it depends on `HeifContext`.
- `HeifContext` implements the `Send` trait now.

## [0.2.1] - 2019-06-24

- Fixed filtering of metadata blocks by type.

## [0.2.0] - 2019-06-18

- Changed URL of the crate documentation.
- Added small example of usage into README.md.
- Changed some enum values and name of methods to comply with the Rust
  naming conventions.
- Methods `Encoder::set_lossless()` and `Encoder::set_lossy_quality()`
  replaced by `Encoder::set_quality()`.
- Added methods `Image::planes()` and `Image::planes_mut()`.

## [0.1.0]

- Initial version.
