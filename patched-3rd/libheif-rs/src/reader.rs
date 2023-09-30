use std::io;
use std::os::raw::{c_int, c_void};
use std::slice;

use libheif_sys as lh;

use crate::enums::ReaderGrowStatus;

pub trait Reader {
    /// Current position, in bytes, inside of a source.
    fn position(&mut self) -> u64;

    /// Pull some bytes from a source into the specified buffer, returning
    /// how many bytes were read.
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>;

    /// Seek to an position, in bytes, from start of a source.
    fn seek(&mut self, position: u64) -> io::Result<u64>;

    /// Wait until a source will be ready to read bytes to
    /// the specified position.
    fn wait_for_file_size(&mut self, target_size: u64) -> ReaderGrowStatus;
}

#[derive(Debug)]
pub struct StreamReader<T>
where
    T: io::Read + io::Seek,
{
    stream: T,
    total_size: u64,
}

impl<T> StreamReader<T>
where
    T: io::Read + io::Seek,
{
    pub fn new(stream: T, total_size: u64) -> StreamReader<T> {
        StreamReader { stream, total_size }
    }
}

impl<T> Reader for StreamReader<T>
where
    T: io::Read + io::Seek,
{
    fn position(&mut self) -> u64 {
        self.stream.stream_position().unwrap_or(self.total_size)
    }

    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }

    fn seek(&mut self, position: u64) -> io::Result<u64> {
        self.stream.seek(io::SeekFrom::Start(position as _))
    }

    fn wait_for_file_size(&mut self, target_size: u64) -> ReaderGrowStatus {
        if self.stream.stream_position().is_err() {
            ReaderGrowStatus::Timeout
        } else if target_size > self.total_size {
            ReaderGrowStatus::SizeBeyondEof
        } else {
            ReaderGrowStatus::SizeReached
        }
    }
}

unsafe extern "C" fn get_position(user_data: *mut c_void) -> i64 {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    reader.position() as _
}

unsafe extern "C" fn read(data: *mut c_void, size: usize, user_data: *mut c_void) -> c_int {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    let buf = slice::from_raw_parts_mut(data as *mut u8, size);
    match reader.read(buf) {
        Ok(real_size) if real_size == buf.len() => 0,
        _ => 1,
    }
}

unsafe extern "C" fn seek(position: i64, user_data: *mut c_void) -> c_int {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    match reader.seek(position as _) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

unsafe extern "C" fn wait_for_file_size(
    target_size: i64,
    user_data: *mut c_void,
) -> lh::heif_reader_grow_status {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    let target_size = target_size as u64;
    reader.wait_for_file_size(target_size) as _
}

pub(crate) static HEIF_READER: lh::heif_reader = lh::heif_reader {
    reader_api_version: 1,
    get_position: Some(get_position),
    read: Some(read),
    seek: Some(seek),
    wait_for_file_size: Some(wait_for_file_size),
};
