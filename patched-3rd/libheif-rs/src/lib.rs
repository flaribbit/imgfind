#![doc = include_str!("../README.md")]

pub use color_profile::*;
pub use context::HeifContext;
pub use decoder::*;
pub use encoder::*;
pub use enums::*;
pub use errors::{HeifError, HeifErrorCode, HeifErrorSubCode, Result};
pub use heif::*;
pub use image::*;
pub use image_handle::{ImageHandle, ItemId};
pub use reader::{Reader, StreamReader};
pub use utils::check_file_type;

mod color_profile;
mod context;
mod decoder;
mod encoder;
mod enums;
mod errors;
mod heif;
mod image;
mod image_handle;
mod reader;
mod utils;
