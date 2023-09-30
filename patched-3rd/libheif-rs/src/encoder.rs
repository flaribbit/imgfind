use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::sync::Mutex;

use libheif_sys as lh;

use crate::utils::cstr_to_str;
use crate::{
    ChromaDownsamplingAlgorithm, ChromaUpsamplingAlgorithm, ColorConversionOptions, HeifError,
    HeifErrorCode, HeifErrorSubCode, ImageOrientation, Result,
};

static ENCODER_MUTEX: Mutex<()> = Mutex::new(());

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, enumn::N)]
#[non_exhaustive]
#[repr(C)]
pub enum CompressionFormat {
    Undefined = lh::heif_compression_format_heif_compression_undefined as _,
    Hevc = lh::heif_compression_format_heif_compression_HEVC as _,
    Avc = lh::heif_compression_format_heif_compression_AVC as _,
    Jpeg = lh::heif_compression_format_heif_compression_JPEG as _,
    Av1 = lh::heif_compression_format_heif_compression_AV1 as _,
    Vvc = lh::heif_compression_format_heif_compression_VVC as _,
    Evc = lh::heif_compression_format_heif_compression_EVC as _,
    Jpeg2000 = lh::heif_compression_format_heif_compression_JPEG2000 as _,
    Uncompressed = lh::heif_compression_format_heif_compression_uncompressed as _,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, enumn::N)]
#[repr(C)]
pub enum EncoderParameterType {
    Int = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_integer as _,
    Bool = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_boolean as _,
    String = lh::heif_encoder_parameter_type_heif_encoder_parameter_type_string as _,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderParameterValue {
    Int(i32),
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EncoderQuality {
    LossLess,
    /// Value inside variant is a 'quality' factor (0-100).
    /// How this is mapped to actual encoding parameters is encoder dependent.
    Lossy(u8),
}

pub type EncoderParametersTypes = HashMap<String, EncoderParameterType>;

pub struct Encoder<'a> {
    pub(crate) inner: *mut lh::heif_encoder,
    pub(crate) parameters_types: EncoderParametersTypes,
    phantom: PhantomData<&'a mut lh::heif_encoder>,
}

impl<'a> Encoder<'a> {
    pub(crate) fn new(c_encoder: &'a mut lh::heif_encoder) -> Result<Self> {
        let parameters_types = parameters_types(c_encoder)?;
        Ok(Self {
            inner: c_encoder,
            parameters_types,
            phantom: PhantomData::default(),
        })
    }
}

impl<'a> Drop for Encoder<'a> {
    fn drop(&mut self) {
        unsafe { lh::heif_encoder_release(self.inner) };
    }
}

impl<'a> Encoder<'a> {
    /// Name of encoder.
    pub fn name(&self) -> String {
        // Name of encoder in `libheif` is mutable static array of chars.
        // So we must use mutex to get access this array.
        let _lock = ENCODER_MUTEX.lock();
        let res = unsafe { lh::heif_encoder_get_name(self.inner) };
        cstr_to_str(res).unwrap_or("").to_owned()
    }

    pub fn set_quality(&mut self, quality: EncoderQuality) -> Result<()> {
        let err = match quality {
            EncoderQuality::LossLess => unsafe { lh::heif_encoder_set_lossless(self.inner, 1) },
            EncoderQuality::Lossy(value) => unsafe {
                let middle_err = lh::heif_encoder_set_lossless(self.inner, 0);
                HeifError::from_heif_error(middle_err)?;
                lh::heif_encoder_set_lossy_quality(self.inner, i32::from(value))
            },
        };
        HeifError::from_heif_error(err)
    }

    fn parameter_value(
        &self,
        name: &str,
        parameter_type: EncoderParameterType,
    ) -> Result<EncoderParameterValue> {
        let c_param_name = CString::new(name).unwrap();
        let param_value = match parameter_type {
            EncoderParameterType::Int => {
                let mut value = 0;
                let err = unsafe {
                    lh::heif_encoder_get_parameter_integer(
                        self.inner,
                        c_param_name.as_ptr(),
                        &mut value as _,
                    )
                };
                HeifError::from_heif_error(err)?;
                EncoderParameterValue::Int(value)
            }
            EncoderParameterType::Bool => {
                let mut value = 0;
                let err = unsafe {
                    lh::heif_encoder_get_parameter_boolean(
                        self.inner,
                        c_param_name.as_ptr(),
                        &mut value as _,
                    )
                };
                HeifError::from_heif_error(err)?;
                EncoderParameterValue::Bool(value > 0)
            }
            EncoderParameterType::String => {
                let value: Vec<u8> = vec![0; 51];
                let err = unsafe {
                    lh::heif_encoder_get_parameter_string(
                        self.inner,
                        c_param_name.as_ptr(),
                        value.as_ptr() as _,
                        50,
                    )
                };
                HeifError::from_heif_error(err)?;
                EncoderParameterValue::String(
                    cstr_to_str(value.as_ptr() as _).unwrap_or("").to_string(),
                )
            }
        };

        Ok(param_value)
    }

    pub fn parameters_names(&self) -> Vec<String> {
        self.parameters_types.keys().cloned().collect()
    }

    /// Get value of encoder's parameter.
    pub fn parameter(&self, name: &str) -> Result<Option<EncoderParameterValue>> {
        match self.parameters_types.get(name) {
            Some(param_type) => {
                let value = self.parameter_value(name, *param_type)?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    /// Set value of encoder's parameter.
    pub fn set_parameter_value(&self, name: &str, value: EncoderParameterValue) -> Result<()> {
        let c_param_name = CString::new(name).unwrap();
        let err = match value {
            EncoderParameterValue::Bool(v) => unsafe {
                lh::heif_encoder_set_parameter_boolean(self.inner, c_param_name.as_ptr(), v.into())
            },
            EncoderParameterValue::Int(v) => unsafe {
                lh::heif_encoder_set_parameter_integer(self.inner, c_param_name.as_ptr(), v)
            },
            EncoderParameterValue::String(v) => unsafe {
                let c_param_value = CString::new(v).unwrap();
                lh::heif_encoder_set_parameter_string(
                    self.inner,
                    c_param_name.as_ptr(),
                    c_param_value.as_ptr(),
                )
            },
        };
        HeifError::from_heif_error(err)?;
        Ok(())
    }
}

fn parameters_types(c_encoder: &mut lh::heif_encoder) -> Result<EncoderParametersTypes> {
    let mut res = EncoderParametersTypes::new();
    unsafe {
        let mut param_pointers = lh::heif_encoder_list_parameters(c_encoder);
        if !param_pointers.is_null() {
            while let Some(raw_param) = (*param_pointers).as_ref() {
                let c_param_type = lh::heif_encoder_parameter_get_type(raw_param);
                let param_type = match EncoderParameterType::n(c_param_type) {
                    Some(res) => res,
                    None => {
                        return Err(HeifError {
                            code: HeifErrorCode::EncoderPluginError,
                            sub_code: HeifErrorSubCode::UnsupportedParameter,
                            message: format!("{} is unknown type of parameter", c_param_type),
                        });
                    }
                };
                let c_param_name = lh::heif_encoder_parameter_get_name(raw_param);
                let name = cstr_to_str(c_param_name).unwrap_or("").to_string();
                res.insert(name, param_type);
                param_pointers = param_pointers.offset(1);
            }
        }
    }
    Ok(res)
}

#[derive(Debug)]
pub struct EncodingOptions {
    pub(crate) inner: *mut lh::heif_encoding_options,
}

impl EncodingOptions {
    pub fn new() -> Result<Self> {
        let inner = unsafe { lh::heif_encoding_options_alloc() };
        if inner.is_null() {
            return Err(HeifError {
                code: HeifErrorCode::MemoryAllocationError,
                sub_code: HeifErrorSubCode::Unspecified,
                message: Default::default(),
            });
        }
        Ok(Self { inner })
    }
}

impl Default for EncodingOptions {
    fn default() -> Self {
        Self::new().expect("heif_encoding_options_alloc() returns a null pointer")
    }
}

impl Drop for EncodingOptions {
    fn drop(&mut self) {
        unsafe {
            lh::heif_encoding_options_free(self.inner);
        }
    }
}

impl EncodingOptions {
    #[inline(always)]
    fn inner_ref(&self) -> &lh::heif_encoding_options {
        unsafe { &(*self.inner) }
    }

    #[inline(always)]
    fn inner_mut(&mut self) -> &mut lh::heif_encoding_options {
        unsafe { &mut (*self.inner) }
    }

    #[inline]
    pub fn version(&self) -> u8 {
        self.inner_ref().version
    }

    #[inline]
    pub fn save_alpha_channel(&self) -> bool {
        self.inner_ref().save_alpha_channel != 0
    }

    #[inline]
    pub fn set_save_alpha_channel(&mut self, enable: bool) {
        self.inner_mut().save_alpha_channel = if enable { 1 } else { 0 };
    }

    #[inline]
    pub fn mac_os_compatibility_workaround(&self) -> bool {
        self.inner_ref().macOS_compatibility_workaround != 0
    }

    #[inline]
    pub fn set_mac_os_compatibility_workaround(&mut self, enable: bool) {
        self.inner_mut().macOS_compatibility_workaround = if enable { 1 } else { 0 };
    }

    #[inline]
    pub fn save_two_colr_boxes_when_icc_and_nclx_available(&self) -> bool {
        self.inner_ref()
            .save_two_colr_boxes_when_ICC_and_nclx_available
            != 0
    }

    #[inline]
    pub fn set_save_two_colr_boxes_when_icc_and_nclx_available(&mut self, enable: bool) {
        self.inner_mut()
            .save_two_colr_boxes_when_ICC_and_nclx_available = if enable { 1 } else { 0 };
    }

    #[inline]
    pub fn mac_os_compatibility_workaround_no_nclx_profile(&self) -> bool {
        self.inner_ref()
            .macOS_compatibility_workaround_no_nclx_profile
            != 0
    }

    #[inline]
    pub fn set_mac_os_compatibility_workaround_no_nclx_profile(&mut self, enable: bool) {
        self.inner_mut()
            .macOS_compatibility_workaround_no_nclx_profile = if enable { 1 } else { 0 };
    }

    #[inline]
    pub fn image_orientation(&self) -> ImageOrientation {
        let orientation = self.inner_ref().image_orientation;
        ImageOrientation::n(orientation).unwrap_or(ImageOrientation::Normal)
    }

    #[inline]
    pub fn set_image_orientation(&mut self, orientation: ImageOrientation) {
        self.inner_mut().image_orientation = orientation as _;
    }

    pub fn color_conversion_options(&self) -> ColorConversionOptions {
        let lh_options = self.inner_ref().color_conversion_options;
        ColorConversionOptions {
            preferred_chroma_downsampling_algorithm: ChromaDownsamplingAlgorithm::n(
                lh_options.preferred_chroma_downsampling_algorithm,
            )
            .unwrap_or(ChromaDownsamplingAlgorithm::Average),
            preferred_chroma_upsampling_algorithm: ChromaUpsamplingAlgorithm::n(
                lh_options.preferred_chroma_upsampling_algorithm,
            )
            .unwrap_or(ChromaUpsamplingAlgorithm::Bilinear),
            only_use_preferred_chroma_algorithm: lh_options.only_use_preferred_chroma_algorithm
                != 0,
        }
    }

    pub fn set_color_conversion_options(&mut self, options: ColorConversionOptions) {
        let lh_options = &mut self.inner_mut().color_conversion_options;
        lh_options.preferred_chroma_downsampling_algorithm =
            options.preferred_chroma_downsampling_algorithm as _;
        lh_options.preferred_chroma_upsampling_algorithm =
            options.preferred_chroma_upsampling_algorithm as _;
        lh_options.only_use_preferred_chroma_algorithm =
            options.only_use_preferred_chroma_algorithm as _;
    }
}

#[derive(Copy, Clone)]
pub struct EncoderDescriptor<'a> {
    pub(crate) inner: &'a lh::heif_encoder_descriptor,
}

impl<'a> EncoderDescriptor<'a> {
    pub(crate) fn new(inner: &'a lh::heif_encoder_descriptor) -> Self {
        Self { inner }
    }

    /// A short, symbolic name for identifying the encoder.
    /// This name should stay constant over different encoder versions.
    pub fn id(&self) -> &str {
        let name = unsafe { lh::heif_encoder_descriptor_get_id_name(self.inner) };
        cstr_to_str(name).unwrap_or_default()
    }

    /// A long, descriptive name of the encoder
    /// (including version information).
    pub fn name(&self) -> String {
        // Name of encoder in `libheif` is mutable static array of chars.
        // So we must use mutex to get access this array.
        let _lock = ENCODER_MUTEX.lock();
        let name = unsafe { lh::heif_encoder_descriptor_get_name(self.inner) };
        cstr_to_str(name).unwrap_or_default().to_owned()
    }

    pub fn compression_format(&self) -> CompressionFormat {
        let c_format = unsafe { lh::heif_encoder_descriptor_get_compression_format(self.inner) };
        match CompressionFormat::n(c_format) {
            Some(res) => res,
            None => CompressionFormat::Undefined,
        }
    }

    pub fn supports_lossy_compression(&self) -> bool {
        unsafe { lh::heif_encoder_descriptor_supports_lossy_compression(self.inner) != 0 }
    }

    pub fn supports_lossless_compression(&self) -> bool {
        unsafe { lh::heif_encoder_descriptor_supports_lossless_compression(self.inner) != 0 }
    }
}

impl<'a> Debug for EncoderDescriptor<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EncoderDescriptor")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}
