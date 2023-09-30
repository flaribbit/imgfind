# Change Log

## [2.0.0] - 2023-09-12

- Fixed link to crate documentation.
- Updated version of `bindgen` to 0.68.
- Added bindgen settings to copy comments from `heif.h` into generated
  rust file.
- **BREAKING**: Feature `use-binding` added into list of default features.

## [1.16.2] - 2023-09-08

- Fixed minimal required version of `libeif` in `build.rs` script.
- Don't link with `libheif` in case of building documentation for `docs.rs`.

## [1.16.1] - 2023-06-21

- Fixed minimal required version of `libheif` in `README.md`.

## [1.16.0] - 2023-06-21

- Updated version of `bindgen` to 0.66.
- Updated "bindings.rs" to correspond `libheif 1.16.2`:
  - added new values into `heif_suberror_code` "enum":
    - `heif_suberror_code_heif_suberror_Invalid_region_data`
    - `heif_suberror_code_heif_suberror_Invalid_property` 
    - `heif_suberror_code_heif_suberror_Item_reference_cycle`
    - `heif_suberror_code_heif_suberror_Encoder_initialization`
    - `heif_suberror_code_heif_suberror_Encoder_encoding`
    - `heif_suberror_code_heif_suberror_Encoder_cleanup`
    - `heif_suberror_code_heif_suberror_Too_many_regions`
  - added new values into `heif_brand` "enum":
    - `heif_brand_heif_vvic` 
    - `heif_brand_heif_vvis`
    - `heif_brand_heif_evbi` 
    - `heif_brand_heif_evbs`
  - added new values into `heif_compression_format` "enum":
    - `heif_compression_format_heif_compression_VVC` 
    - `heif_compression_format_heif_compression_EVC`
    - `heif_compression_format_heif_compression_JPEG2000` 
    - `heif_compression_format_heif_compression_uncompressed`
  - added new "enum" `heif_item_property_type` with follow values:
    - `heif_item_property_type_heif_item_property_type_invalid`
    - `heif_item_property_type_heif_item_property_type_user_description`
    - `heif_item_property_type_heif_item_property_type_transform_mirror`
    - `heif_item_property_type_heif_item_property_type_transform_rotation`
    - `heif_item_property_type_heif_item_property_type_transform_crop`
    - `heif_item_property_type_heif_item_property_type_image_size`
  - added new "enum" `heif_transform_mirror_direction` with follow values:
    - `heif_transform_mirror_direction_heif_transform_mirror_direction_vertical`
    - `heif_transform_mirror_direction_heif_transform_mirror_direction_horizontal`
  - added new "enum" `heif_chroma_downsampling_algorithm` with follow values:
    - `heif_chroma_downsampling_algorithm_heif_chroma_downsampling_nearest_neighbor`
    - `heif_chroma_downsampling_algorithm_heif_chroma_downsampling_average`
    - `heif_chroma_downsampling_algorithm_heif_chroma_downsampling_sharp_yuv`
  - added new "enum" `heif_chroma_upsampling_algorithm` with follow values:
    - `heif_chroma_upsampling_algorithm_heif_chroma_upsampling_nearest_neighbor`
    - `heif_chroma_upsampling_algorithm_heif_chroma_upsampling_bilinear`
  - added new "enum" `heif_region_type` with follow values:
    - `heif_region_type_heif_region_type_point`
    - `heif_region_type_heif_region_type_rectangle`
    - `heif_region_type_heif_region_type_ellipse`
    - `heif_region_type_heif_region_type_polygon`
    - `heif_region_type_heif_region_type_referenced_mask`
    - `heif_region_type_heif_region_type_inline_mask`
    - `heif_region_type_heif_region_type_polyline`
  - added structs: 
    - `heif_property_user_description` 
    - `heif_plugin_info`
    - `heif_color_conversion_options` 
    - `heif_content_light_level`
    - `heif_mastering_display_colour_volume`
    - `heif_decoded_mastering_display_colour_volume`
    - `heif_decoder_descriptor` 
    - `heif_region_item` 
    - `heif_region`
  - added new fields into `heif_decoding_options` struct:
    - `decoder_id`
    - `color_conversion_options`
  - added field `color_conversion_options` into structure `heif_encoding_options`;
  - added functions: 
    - `heif_image_handle_get_item_id` 
    - `heif_image_handle_release_auxiliary_type` 
    - `heif_item_get_properties_of_type` 
    - `heif_item_get_transformation_properties`
    - `heif_item_get_property_type`
    - `heif_item_get_property_user_description`
    - `heif_item_add_property_user_description` 
    - `heif_property_user_description_release`
    - `heif_item_get_property_transform_mirror`
    - `heif_item_get_property_transform_rotation_ccw`
    - `heif_item_get_property_transform_crop_borders`
    - `heif_image_has_content_light_level`
    - `heif_image_get_content_light_level`
    - `heif_image_set_content_light_level`
    - `heif_image_has_mastering_display_colour_volume`
    - `heif_image_get_mastering_display_colour_volume`
    - `heif_image_set_mastering_display_colour_volume`
    - `heif_mastering_display_colour_volume_decode`
    - `heif_image_get_pixel_aspect_ratio`
    - `heif_image_set_pixel_aspect_ratio`
    - `heif_get_decoder_descriptors`
    - `heif_decoder_descriptor_get_name`
    - `heif_decoder_descriptor_get_id_name`
    - `heif_get_encoder_descriptors`
    - `heif_image_extend_padding_to_size`
    - `heif_image_handle_get_number_of_region_items`
    - `heif_image_handle_get_list_of_region_item_ids`
    - `heif_context_get_region_item`
    - `heif_region_item_get_id`
    - `heif_region_item_release`
    - `heif_region_item_get_reference_size`
    - `heif_region_item_get_number_of_regions`
    - `heif_region_item_get_list_of_regions`
    - `heif_region_release`
    - `heif_region_release_many`
    - `heif_region_get_type`
    - `heif_region_get_point`
    - `heif_region_get_point_transformed`
    - `heif_region_get_rectangle`
    - `heif_region_get_rectangle_transformed`
    - `heif_region_get_ellipse`
    - `heif_region_get_ellipse_transformed`
    - `heif_region_get_polygon_num_points`
    - `heif_region_get_polygon_points`
    - `heif_region_get_polygon_points_transformed`
    - `heif_region_get_polyline_num_points`
    - `heif_region_get_polyline_points`
    - `heif_region_get_polyline_points_transformed`
    - `heif_image_handle_add_region_item`
    - `heif_region_item_add_region_point`
    - `heif_region_item_add_region_rectangle`
    - `heif_region_item_add_region_ellipse`
    - `heif_region_item_add_region_polygon`
    - `heif_region_item_add_region_polyline`

## [1.14.4] - 2023-06-21

- For Windows target [vcpkg crate](https://crates.io/crates/vcpkg) is used 
  to find `libheif` installed with help of `vcpkg`.
- Added support of [cargo-vcpkg](https://crates.io/crates/cargo-vcpkg)
  to install `libheif` with help of `cargo`.

## [1.14.3] - 2023-06-05

- Updated version of `bindgen` to 0.65.
- Use `pkg-config` in `build.rs` ([#1](https://github.com/Cykooz/libheif-sys/pull/1)).

## [1.14.2] - 2023-01-31

- Updated version of `bindgen` to 0.63.0.
- Updated "bindings.rs" to correspond `libheif 1.14.2`:
  - added new values into `heif_error_code` "enum":
    `heif_error_code_heif_error_Plugin_loading_error`.
  - added new values into `heif_suberror_code` "enum":
    `heif_suberror_code_heif_suberror_Unknown_NCLX_color_primaries`,
    `heif_suberror_code_heif_suberror_Unknown_NCLX_transfer_characteristics`,
    `heif_suberror_code_heif_suberror_Unknown_NCLX_matrix_coefficients`,
    `heif_suberror_code_heif_suberror_Unsupported_header_compression_method`,
    `heif_suberror_code_heif_suberror_Plugin_loading_error`,
    `heif_suberror_code_heif_suberror_Plugin_is_not_loaded`,
    `heif_suberror_code_heif_suberror_Cannot_read_plugin_directory`.
  - added new "enum" `heif_plugin_type` with follow values:
    `heif_plugin_type_heif_plugin_type_encoder`, 
    `heif_plugin_type_heif_plugin_type_decoder`.
  - added new "enum" `heif_orientation` with follow values:
    `heif_orientation_heif_orientation_normal`,
    `heif_orientation_heif_orientation_flip_horizontally`,
    `heif_orientation_heif_orientation_rotate_180`,
    `heif_orientation_heif_orientation_flip_vertically`,
    `heif_orientation_heif_orientation_rotate_90_cw_then_flip_horizontally`,
    `heif_orientation_heif_orientation_rotate_90_cw`,
    `heif_orientation_heif_orientation_rotate_90_cw_then_flip_vertically`,
    `heif_orientation_heif_orientation_rotate_270_cw`.
  - added new "enum" `heif_metadata_compression` with follow values:
    `heif_metadata_compression_heif_metadata_compression_off`,
    `heif_metadata_compression_heif_metadata_compression_auto`,
    `heif_metadata_compression_heif_metadata_compression_deflate`.
  - added structs: `heif_init_params`, `heif_plugin_info`.
  - added field `strict_decoding` into structure `heif_decoding_options`.
  - added field `image_orientation` into structure `heif_encoding_options`.
  - added functions: `heif_init`, `heif_deinit`, `heif_load_plugin`,
    `heif_load_plugins`, `heif_unload_plugin`, `heif_check_jpeg_filetype`,
    `heif_context_set_max_decoding_threads`,
    `heif_nclx_color_profile_set_color_primaries`,
    `heif_nclx_color_profile_set_transfer_characteristics`,
    `heif_nclx_color_profile_set_matrix_coefficients`,
    `heif_image_get_decoding_warnings`,
    `heif_image_add_decoding_warning`,
    `heif_context_add_XMP_metadata2`.

## [1.12.0] - 2021-05-12

- Updated "bindings.rs" to correspond `libheif 1.12`:
  - added new values into `heif_suberror_code` "enum":
    `heif_suberror_code_heif_suberror_Wrong_tile_image_pixel_depth`.
  - added new functions: `heif_image_handle_is_premultiplied_alpha`,
    `heif_image_set_premultiplied_alpha`,  `heif_image_is_premultiplied_alpha`.
    
## [1.11.0] - 2021-02-03

- Updated "bindings.rs" to correspond `libheif 1.11`:
  - added new type `heif_brand2`;
  - added new functions: `heif_read_main_brand`,
    `heif_fourcc_to_brand`,  `heif_brand_to_fourcc`,
    `heif_has_compatible_brand`, `heif_list_compatible_brands`,
    `heif_free_list_of_compatible_brands`,
    `heif_image_handle_free_auxiliary_types`,
  - added new fields into `heif_encoding_options` struct:
    `output_nclx_profile`, `macOS_compatibility_workaround_no_nclx_profile`.

## [1.10.0] - 2021-01-14

- Updated "bindings.rs" to correspond `libheif 1.10`:
  - added new values into `heif_error_code` "enum":
    `heif_error_code_heif_error_Color_profile_does_not_exist`.  
  - added new functions: `heif_image_handle_get_number_of_auxiliary_images`,
    `heif_image_handle_get_list_of_auxiliary_image_IDs`, 
    `heif_image_handle_get_auxiliary_type`,
    `heif_image_handle_get_auxiliary_image_handle`,
    `heif_encoder_parameter_get_valid_integer_values`,
    `heif_encoder_parameter_integer_valid_values`.
  - added new fields into `heif_encoding_options` struct:
    `macOS_compatibility_workaround`,
    `save_two_colr_boxes_when_ICC_and_nclx_available`.

## [1.9.0] - 2020-09-26

- Updated "bindings.rs" to correspond `libheif 1.9`:
  - added new functions: `heif_nclx_color_profile_alloc`, 
    `heif_image_get_primary_width`, `heif_image_get_primary_height`,
    `heif_image_crop`.
    
## [1.8.1] - 2020-08-28

- Fixed `README.md`.

## [1.8.0] - 2020-08-28

- Updated version of `bindgen` to 0.55.1.
- Updated "bindings.rs" to correspond `libheif 1.8`:
  - added new functions: `heif_nclx_color_profile_free`, 
    `heif_encoder_descriptor_supports_lossy_compression`,
    `heif_encoder_descriptor_supports_lossless_compression`.
  - added new values into `heif_suberror_code` "enum":
    `heif_suberror_code_heif_suberror_Invalid_pixi_box`,
    `heif_suberror_code_heif_suberror_No_av1C_box`.
  - added new values into `heif_brand` "enum":
    `heif_brand_heif_avif`, `heif_brand_heif_avis`.
  - added new values into `heif_color_primaries` "enum":
    `heif_color_primaries_heif_color_primaries_generic_film`, 
    `heif_color_primaries_heif_color_primaries_ITU_R_BT_2020_2_and_2100_0`,
    `heif_color_primaries_heif_color_primaries_SMPTE_ST_428_1`,
    `heif_color_primaries_heif_color_primaries_SMPTE_RP_431_2`,
    `heif_color_primaries_heif_color_primaries_SMPTE_EG_432_1`,
    `heif_color_primaries_heif_color_primaries_EBU_Tech_3213_E`.
  - added new values into `heif_transfer_characteristics` "enum":
    `heif_transfer_characteristics_heif_transfer_characteristic_logarithmic_100`,
    `heif_transfer_characteristics_heif_transfer_characteristic_logarithmic_100_sqrt10`,
    `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2020_2_10bit`,
    `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2020_2_12bit`,
    `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2100_0_PQ`,
    `heif_transfer_characteristics_heif_transfer_characteristic_SMPTE_ST_428_1`,
    `heif_transfer_characteristics_heif_transfer_characteristic_ITU_R_BT_2100_0_HLG`.   
  - added new values into `heif_matrix_coefficients` "enum":
    `heif_matrix_coefficients_heif_matrix_coefficients_ITU_R_BT_2020_2_non_constant_luminance`,
    `heif_matrix_coefficients_heif_matrix_coefficients_ITU_R_BT_2020_2_constant_luminance`,
    `heif_matrix_coefficients_heif_matrix_coefficients_SMPTE_ST_2085`,
    `heif_matrix_coefficients_heif_matrix_coefficients_chromaticity_derived_non_constant_luminance`,
    `heif_matrix_coefficients_heif_matrix_coefficients_chromaticity_derived_constant_luminance`,
    `heif_matrix_coefficients_heif_matrix_coefficients_ICtCp`.
  - added new values into `heif_compression_format` "enum":
    `heif_compression_format_heif_compression_AV1`.
  - added field `convert_hdr_to_8bit` into `heif_decoding_options` struct.

## [1.6.0] - 2019-11-13

- Updated "bindings.rs" to correspond `libheif 1.6`:
  - added new functions: `heif_context_set_maximum_image_size_limit`, 
    `heif_context_add_generic_metadata`.

## [1.5.0] - 2019-08-28

- Updated "bindings.rs" to correspond `libheif 1.5`:
  - added new value of `heif_brand` - `heif_brand_heif_msf1 = 10`;
  - added new functions: `heif_get_file_mime_type`, `heif_image_get_color_profile_type`,
    `heif_image_get_raw_color_profile_size`, `heif_image_get_raw_color_profile`,
    `heif_image_get_nclx_color_profile`, `heif_image_get_bits_per_pixel_range`.

## [1.4.2] - 2019-07-16

- Added "libc" as dependency.
- "bindings.rs" has been rebuild, removed not needed definitions. 

## [1.4.1] - 2019-05-24

- Added the feature "use-bindgen" to enable generate bindings
  during building time.

## [1.4.0]

- Initial version.
