use libheif_rs::{
    Channel, ChromaDownsamplingAlgorithm, ChromaUpsamplingAlgorithm, ColorSpace, CompressionFormat,
    EncoderParameterValue, EncoderQuality, EncodingOptions, HeifContext, Image, ImageOrientation,
    LibHeif, Result, RgbChroma,
};

fn create_image(width: u32, height: u32) -> Result<Image> {
    let mut image = Image::new(width, height, ColorSpace::Rgb(RgbChroma::Rgb))?;
    image.create_plane(Channel::Interleaved, width, height, 24)?;

    let planes = image.planes_mut();
    let plane = planes.interleaved.unwrap();
    let stride = plane.stride;
    let data = plane.data;

    for y in 0..height {
        let mut row_start = stride * y as usize;
        for x in 0..width {
            let color = x * y;
            data[row_start] = ((color & 0x00_ff_00_00) >> 16) as u8;
            data[row_start + 1] = ((color & 0x00_00_ff_00) >> 8) as u8;
            data[row_start + 2] = (color & 0x00_00_00_ff) as u8;
            row_start += 3;
        }
    }
    Ok(image)
}

#[test]
fn create_and_encode_image() -> Result<()> {
    let width = 640;
    let height = 480;
    let image = create_image(width, height)?;
    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;
    encoder.set_quality(EncoderQuality::LossLess)?;
    let encoding_options: EncodingOptions = Default::default();
    context.encode_image(&image, &mut encoder, Some(encoding_options))?;

    let buf = context.write_to_bytes()?;

    // Check result of encoding by decode it
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;
    assert_eq!(handle.width(), width);
    assert_eq!(handle.height(), height);

    // Decode the image
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::Rgb)));
    let planes = image.planes();
    let plan = planes.interleaved.unwrap();
    assert_eq!(plan.width, width);
    assert_eq!(plan.height, height);

    Ok(())
}

#[test]
fn create_and_encode_monochrome_image() -> Result<()> {
    let width = 640;
    let height = 480;

    let mut image = Image::new(width, height, ColorSpace::Monochrome)?;

    image.create_plane(Channel::Y, width, height, 8)?;

    let planes = image.planes_mut();
    let plane_a = planes.y.unwrap();
    let stride = plane_a.stride;
    let data_a = plane_a.data;

    for y in 0..height {
        let mut row_start = stride * y as usize;
        for x in 0..width {
            let color = ((x + y) % 255) as u8;
            data_a[row_start] = color;
            row_start += 1;
        }
    }

    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;

    encoder.set_quality(EncoderQuality::LossLess)?;
    let encoding_options = EncodingOptions::new()?;

    context.encode_image(&image, &mut encoder, Some(encoding_options))?;
    let _buf = context.write_to_bytes()?;

    Ok(())
}

#[test]
fn set_encoder_param() -> Result<()> {
    let width = 640;
    let height = 480;
    let image = create_image(width, height)?;

    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;
    encoder.set_parameter_value("speed", EncoderParameterValue::Int(5))?;
    let encoding_options = EncodingOptions::new()?;
    context.encode_image(&image, &mut encoder, Some(encoding_options))?;

    let buf = context.write_to_bytes()?;

    // Check result of encoding by decode it
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;
    assert_eq!(handle.width(), width);
    assert_eq!(handle.height(), height);

    // Decode the image
    let image = lib_heif.decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)?;
    assert_eq!(image.color_space(), Some(ColorSpace::Rgb(RgbChroma::Rgb)));
    let planes = image.planes();
    let plan = planes.interleaved.unwrap();
    assert_eq!(plan.width, width);
    assert_eq!(plan.height, height);

    Ok(())
}

#[test]
fn add_metadata() -> Result<()> {
    let width = 640;
    let height = 480;
    let image = create_image(width, height)?;

    let lib_heif = LibHeif::new();
    let mut context = HeifContext::new()?;
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;
    let handle = context.encode_image(&image, &mut encoder, None)?;

    let item_type = b"MyDt";
    let item_data = b"custom data";
    let exif_data = b"MM\0*FakeExif";
    let content_type = Some("text/plain");
    context.add_generic_metadata(&handle, item_data, item_type, content_type)?;
    context.add_exif_metadata(&handle, exif_data)?;
    context.add_xmp_metadata(&handle, item_data)?;

    // Write result HEIF file into vector
    let buf = context.write_to_bytes()?;

    // Check stored meta data in the encoded result
    let context = HeifContext::read_from_bytes(&buf)?;
    let handle = context.primary_image_handle()?;

    // Custom meta data block "MyDt"
    let mut item_ids = vec![0; 1];
    let count = handle.metadata_block_ids(&mut item_ids, item_type);
    assert_eq!(count, 1);
    let md_data = handle.metadata(item_ids[0])?;
    assert_eq!(&md_data, item_data);
    let md_content_type = handle.metadata_content_type(item_ids[0]);
    // content_type is stored in HEIF only for "mime" type of meta data.
    assert_eq!(md_content_type, Some(""));

    // Exif
    let count = handle.metadata_block_ids(&mut item_ids, b"Exif");
    assert_eq!(count, 1);
    let md_data = handle.metadata(item_ids[0])?;
    assert_eq!(&md_data, b"\0\0\0\0MM\0*FakeExif");

    // Xmp
    let count = handle.metadata_block_ids(&mut item_ids, b"mime");
    assert_eq!(count, 1);
    let md_data = handle.metadata(item_ids[0])?;
    assert_eq!(&md_data, item_data);
    let md_content_type = handle.metadata_content_type(item_ids[0]);
    assert_eq!(md_content_type, Some("application/rdf+xml"));

    Ok(())
}

#[test]
fn test_encoder_hevc() -> Result<()> {
    let lib_heif = LibHeif::new();
    let mut encoder = match lib_heif.encoder_for_format(CompressionFormat::Hevc) {
        Ok(e) => e,
        Err(_) => {
            println!(
                "WARNING: Hevc encoder is absent. The test that check encoding of heic file has skipped."
            );
            return Ok(());
        }
    };
    assert!(encoder.name().starts_with("x265 HEVC encoder"));

    let mut params = encoder.parameters_names();
    params.sort();
    assert_eq!(params.len(), 7);
    let expect = vec![
        "chroma".to_string(),
        "complexity".to_string(),
        "lossless".to_string(),
        "preset".to_string(),
        "quality".to_string(),
        "tu-intra-depth".to_string(),
        "tune".to_string(),
    ];
    assert_eq!(params, expect);

    assert_eq!(
        encoder.parameter("lossless")?,
        Some(EncoderParameterValue::Bool(false))
    );

    encoder.set_quality(EncoderQuality::LossLess)?;
    assert_eq!(
        encoder.parameter("lossless")?,
        Some(EncoderParameterValue::Bool(true))
    );
    Ok(())
}

#[test]
fn test_encoder_av1() -> Result<()> {
    let lib_heif = LibHeif::new();
    let mut encoder = lib_heif.encoder_for_format(CompressionFormat::Av1)?;
    assert!(encoder.name().starts_with("AOMedia Project AV1 Encoder"));

    let params = encoder.parameters_names();
    assert!(params.len() >= 13);

    assert_eq!(
        encoder.parameter("lossless")?,
        Some(EncoderParameterValue::Bool(false))
    );

    encoder.set_quality(EncoderQuality::LossLess)?;
    assert_eq!(
        encoder.parameter("lossless")?,
        Some(EncoderParameterValue::Bool(true))
    );
    Ok(())
}

#[test]
fn test_encoding_options() -> Result<()> {
    let enc_options = EncodingOptions::new().unwrap();
    assert!(enc_options.version() >= 5);
    // Test defaults
    assert!(enc_options.save_alpha_channel());
    assert!(!enc_options.mac_os_compatibility_workaround());
    assert!(enc_options.mac_os_compatibility_workaround_no_nclx_profile());
    assert!(!enc_options.save_two_colr_boxes_when_icc_and_nclx_available());
    assert_eq!(enc_options.image_orientation(), ImageOrientation::Normal);
    let color_options = enc_options.color_conversion_options();
    assert_eq!(
        color_options.preferred_chroma_downsampling_algorithm,
        ChromaDownsamplingAlgorithm::Average
    );
    assert_eq!(
        color_options.preferred_chroma_upsampling_algorithm,
        ChromaUpsamplingAlgorithm::Bilinear
    );
    assert!(!color_options.only_use_preferred_chroma_algorithm);

    Ok(())
}
