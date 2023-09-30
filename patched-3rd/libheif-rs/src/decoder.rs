use crate::utils::{cstr_to_str, str_to_cstring};
use crate::{ChromaDownsamplingAlgorithm, ChromaUpsamplingAlgorithm, HeifError};
use libheif_sys as lh;
use std::ffi::CString;
use std::fmt::{Debug, Formatter};
use std::ptr;
use std::sync::Mutex;

static DECODER_MUTEX: Mutex<()> = Mutex::new(());

#[derive(Debug)]
pub struct DecodingOptions {
    pub(crate) inner: *mut lh::heif_decoding_options,
    decoder_id: Option<CString>,
}

impl DecodingOptions {
    pub fn new() -> Option<Self> {
        let inner = unsafe { lh::heif_decoding_options_alloc() };
        if inner.is_null() {
            return None;
        }
        Some(Self {
            inner,
            decoder_id: None,
        })
    }
}

impl Drop for DecodingOptions {
    fn drop(&mut self) {
        unsafe {
            lh::heif_decoding_options_free(self.inner);
        }
    }
}

impl DecodingOptions {
    #[inline(always)]
    fn inner_ref(&self) -> &lh::heif_decoding_options {
        unsafe { &(*self.inner) }
    }

    #[inline(always)]
    fn inner_mut(&mut self) -> &mut lh::heif_decoding_options {
        unsafe { &mut (*self.inner) }
    }

    #[inline]
    pub fn version(&self) -> u8 {
        self.inner_ref().version
    }

    #[inline]
    pub fn ignore_transformations(&self) -> bool {
        self.inner_ref().ignore_transformations != 0
    }

    #[inline]
    pub fn set_ignore_transformations(&mut self, enable: bool) {
        self.inner_mut().ignore_transformations = if enable { 1 } else { 0 }
    }

    #[inline]
    pub fn convert_hdr_to_8bit(&self) -> bool {
        self.inner_ref().convert_hdr_to_8bit != 0
    }

    #[inline]
    pub fn set_convert_hdr_to_8bit(&mut self, enable: bool) {
        self.inner_mut().convert_hdr_to_8bit = if enable { 1 } else { 0 }
    }

    /// When strict decoding is enabled, an error is returned for invalid input.
    /// Otherwise, it will try its best and add decoding warnings to
    /// the decoded `Image`. Default is non-strict.
    pub fn strict_decoding(&self) -> bool {
        self.inner_ref().strict_decoding != 0
    }

    pub fn set_strict_decoding(&mut self, enable: bool) {
        self.inner_mut().strict_decoding = if enable { 1 } else { 0 }
    }

    /// ID of the decoder to use for the decoding.
    /// If set to `None` (default), the highest priority decoder is chosen.
    /// The priority is defined in the plugin.
    pub fn decoder_id(&self) -> Option<&str> {
        cstr_to_str(self.inner_ref().decoder_id)
    }

    pub fn set_decoder_id(&mut self, decoder_id: Option<&str>) -> Result<(), HeifError> {
        if let Some(decoder_id) = decoder_id {
            let c_decoder_id = str_to_cstring(decoder_id, "decoder_id")?;
            self.inner_mut().decoder_id = c_decoder_id.as_ptr();
            self.decoder_id = Some(c_decoder_id);
        } else {
            self.inner_mut().decoder_id = ptr::null() as _;
            self.decoder_id = None;
        }
        Ok(())
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

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ColorConversionOptions {
    pub preferred_chroma_downsampling_algorithm: ChromaDownsamplingAlgorithm,
    pub preferred_chroma_upsampling_algorithm: ChromaUpsamplingAlgorithm,
    /// When set to `false`, libheif may also use a different algorithm
    /// if the preferred one is not available.
    pub only_use_preferred_chroma_algorithm: bool,
}

#[derive(Copy, Clone)]
pub struct DecoderDescriptor<'a> {
    inner: &'a lh::heif_decoder_descriptor,
}

impl<'a> DecoderDescriptor<'a> {
    pub(crate) fn new(inner: &'a lh::heif_decoder_descriptor) -> Self {
        Self { inner }
    }

    /// A short, symbolic name for identifying the decoder.
    /// This name should stay constant over different decoder versions.
    pub fn id(&self) -> &str {
        let name = unsafe { lh::heif_decoder_descriptor_get_id_name(self.inner) };
        cstr_to_str(name).unwrap_or_default()
    }

    /// A long, descriptive name of the decoder
    /// (including version information).
    pub fn name(&self) -> String {
        // Name of decoder in `libheif` is mutable static array of chars.
        // So we must use mutex to get access this array.
        let _lock = DECODER_MUTEX.lock();
        let name = unsafe { lh::heif_decoder_descriptor_get_name(self.inner) };
        cstr_to_str(name).unwrap_or_default().to_owned()
    }
}

impl<'a> Debug for DecoderDescriptor<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DecoderDescriptor")
            .field("id", &self.id())
            .field("name", &self.name())
            .finish()
    }
}
