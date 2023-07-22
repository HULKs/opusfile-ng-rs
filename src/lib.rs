mod bindings;

use std::{ffi::CString, path::Path, ptr::null_mut};

use bindings::{
    op_bitrate, op_bitrate_instant, op_channel_count, op_current_link, op_free, op_head,
    op_link_count, op_open_file, op_open_memory, op_open_url, op_pcm_seek, op_pcm_tell,
    op_pcm_total, op_raw_seek, op_raw_tell, op_raw_total, op_read, op_read_float,
    op_read_float_stereo, op_read_stereo, op_seekable, op_serialno, op_tags, op_test_file,
    op_test_memory, op_test_open, op_test_url,
};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::FromPrimitive;

#[derive(Debug, thiserror::Error, FromPrimitive, ToPrimitive)]
#[repr(i32)]
pub enum OpusFileError {
    #[error("Unknown error.")]
    Unknown = i32::MIN,
    #[error("A request did not succeed.")]
    False = bindings::OP_FALSE,
    #[error("End of file.")]
    Eof = bindings::OP_EOF,
    #[error(
        "There was a hole in the page sequence numbers (e.g., a page was corrupt or missing)."
    )]
    Hole = bindings::OP_HOLE,
    #[error("An underlying read, seek, or tell operation failed when it should have succeeded.")]
    Read = bindings::OP_EREAD,
    #[error("A NULL pointer was passed where one was unexpected, or an internal memory allocation failed, or an internal library error was encountered.")]
    Fault = bindings::OP_EFAULT,
    #[error(
        "The stream used a feature that is not implemented, such as an unsupported channel family."
    )]
    NotImplemented = bindings::OP_EIMPL,
    #[error("One or more parameters to a function were invalid.")]
    InvalidParameters = bindings::OP_EINVAL,
    #[error("A purported Ogg Opus stream did not begin with an Ogg page, a purported header packet did not start with one of the required strings, \"OpusHead\" or \"OpusTags\", or a link in a chained file was encountered that did not contain any logical Opus streams.")]
    InvalidFormat = bindings::OP_ENOTFORMAT,
    #[error("A required header packet was not properly formatted, contained illegal values, or was missing altogether.")]
    BadHeader = bindings::OP_EBADHEADER,
    #[error("The ID header contained an unrecognized version number.")]
    UnrecognizedVersion = bindings::OP_EVERSION,
    #[error("Not audio.")]
    NotAudio = bindings::OP_ENOTAUDIO,
    #[error("An audio packet failed to decode properly.")]
    BadPacket = bindings::OP_EBADPACKET,
    #[error("We failed to find data we had seen before, or the bitstream structure was sufficiently malformed that seeking to the target destination was impossible.")]
    BadLink = bindings::OP_EBADLINK,
    #[error("An operation that requires seeking was requested on an unseekable stream.")]
    UnableToSeek = bindings::OP_ENOSEEK,
    #[error("The first or last granule position of a link failed basic validity checks.")]
    BadTimestamp = bindings::OP_EBADTIMESTAMP,
}

#[derive(Clone, Debug)]
pub struct OggOpusFile(*mut bindings::OggOpusFile);

impl OggOpusFile {
    pub fn open_file(path: impl AsRef<Path>) -> Result<Self, OpusFileError> {
        let path = CString::new(path.as_ref().to_str().unwrap()).expect("CString::new() failed");
        let mut error = 0;
        let handle = unsafe { op_open_file(path.as_ptr(), &mut error) };
        if handle.is_null() || error < 0 {
            Err(OpusFileError::from_i32(error).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(Self(handle))
        }
    }

    pub fn open_memory(data: &[u8]) -> Result<Self, OpusFileError> {
        let mut error = 0;
        let handle = unsafe { op_open_memory(data.as_ptr(), data.len(), &mut error) };
        if handle.is_null() || error < 0 {
            Err(OpusFileError::from_i32(error).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(Self(handle))
        }
    }

    pub fn open_url(url: impl AsRef<Path>) -> Result<Self, OpusFileError> {
        let url = CString::new(url.as_ref().to_str().unwrap()).expect("CString::new() failed");
        let mut error = 0;
        let handle = unsafe { op_open_url(url.as_ptr(), &mut error) };
        if handle.is_null() || error < 0 {
            Err(OpusFileError::from_i32(error).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(Self(handle))
        }
    }

    pub fn test_file(path: impl AsRef<Path>) -> Result<Self, OpusFileError> {
        let path = CString::new(path.as_ref().to_str().unwrap()).expect("CString::new() failed");
        let mut error = 0;
        let handle = unsafe { op_test_file(path.as_ptr(), &mut error) };
        if handle.is_null() || error < 0 {
            Err(OpusFileError::from_i32(error).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(Self(handle))
        }
    }

    pub fn test_memory(data: &[u8]) -> Result<Self, OpusFileError> {
        let mut error = 0;
        let handle = unsafe { op_test_memory(data.as_ptr(), data.len(), &mut error) };
        if handle.is_null() || error < 0 {
            Err(OpusFileError::from_i32(error).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(Self(handle))
        }
    }

    pub fn test_url(url: impl AsRef<Path>) -> Result<Self, OpusFileError> {
        let url = CString::new(url.as_ref().to_str().unwrap()).expect("CString::new() failed");
        let mut error = 0;
        let handle = unsafe { op_test_url(url.as_ptr(), &mut error) };
        if handle.is_null() || error < 0 {
            Err(OpusFileError::from_i32(error).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(Self(handle))
        }
    }

    pub fn test_open(self) -> Result<Self, OpusFileError> {
        let result = unsafe { op_test_open(self.0) };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(self)
        }
    }

    pub fn seekable(&self) -> bool {
        unsafe { op_seekable(self.0) != 0 }
    }

    pub fn link_count(&self) -> usize {
        unsafe { op_link_count(self.0) as usize }
    }

    pub fn serial_number_of_link(&self, link_index: i32) -> u32 {
        unsafe { op_serialno(self.0, link_index) }
    }

    pub fn channel_count(&self, link_index: i32) -> usize {
        unsafe { op_channel_count(self.0, link_index) as usize }
    }

    pub fn raw_total(&self, link_index: i32) -> Result<usize, OpusFileError> {
        let result = unsafe { op_raw_total(self.0, link_index) };
        if result < 0 {
            Err(OpusFileError::from_i64(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result.try_into().unwrap())
        }
    }

    pub fn pcm_total(&self, link_index: i32) -> Result<usize, OpusFileError> {
        let result = unsafe { op_pcm_total(self.0, link_index) };
        if result < 0 {
            Err(OpusFileError::from_i64(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result.try_into().unwrap())
        }
    }

    pub fn head(&self, link_index: i32) -> Result<OpusHead, OpusFileError> {
        let result = unsafe { op_head(self.0, link_index) };
        if result.is_null() {
            Err(OpusFileError::Unknown)
        } else {
            Ok(OpusHead(result))
        }
    }

    pub fn tags(&self, link_index: i32) -> Result<OpusTags, OpusFileError> {
        let result = unsafe { op_tags(self.0, link_index) };
        if result.is_null() {
            Err(OpusFileError::Unknown)
        } else {
            Ok(OpusTags(result))
        }
    }

    pub fn current_link(&self) -> Result<i32, OpusFileError> {
        let result = unsafe { op_current_link(self.0) };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result)
        }
    }

    pub fn bitrate(&self, link_index: i32) -> Result<i32, OpusFileError> {
        let result = unsafe { op_bitrate(self.0, link_index) };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result)
        }
    }

    pub fn bitrate_instant(&self) -> Result<i32, OpusFileError> {
        let result = unsafe { op_bitrate_instant(self.0) };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result)
        }
    }

    pub fn raw_tell(&self) -> Result<i64, OpusFileError> {
        let result = unsafe { op_raw_tell(self.0) };
        if result < 0 {
            Err(OpusFileError::from_i64(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result)
        }
    }

    pub fn pcm_tell(&self) -> Result<i64, OpusFileError> {
        let result = unsafe { op_pcm_tell(self.0) };
        if result < 0 {
            Err(OpusFileError::from_i64(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result)
        }
    }

    pub fn read(
        &self,
        buffer: &mut [i16],
        link_index: Option<&mut i32>,
    ) -> Result<usize, OpusFileError> {
        let link_index = link_index
            .map(|link_index| link_index as *mut _)
            .unwrap_or(null_mut());
        let result = unsafe {
            op_read(
                self.0,
                buffer.as_mut_ptr(),
                buffer.len().try_into().unwrap(),
                link_index,
            )
        };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result.try_into().unwrap())
        }
    }

    pub fn read_float(
        &self,
        buffer: &mut [f32],
        link_index: Option<&mut i32>,
    ) -> Result<usize, OpusFileError> {
        let link_index = link_index
            .map(|link_index| link_index as *mut _)
            .unwrap_or(null_mut());
        let result = unsafe {
            op_read_float(
                self.0,
                buffer.as_mut_ptr(),
                buffer.len().try_into().unwrap(),
                link_index,
            )
        };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result.try_into().unwrap())
        }
    }

    pub fn read_stereo(&self, buffer: &mut [i16]) -> Result<usize, OpusFileError> {
        let result = unsafe {
            op_read_stereo(
                self.0,
                buffer.as_mut_ptr(),
                buffer.len().try_into().unwrap(),
            )
        };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result.try_into().unwrap())
        }
    }

    pub fn read_float_stereo(&self, buffer: &mut [f32]) -> Result<usize, OpusFileError> {
        let result = unsafe {
            op_read_float_stereo(
                self.0,
                buffer.as_mut_ptr(),
                buffer.len().try_into().unwrap(),
            )
        };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(result.try_into().unwrap())
        }
    }

    pub fn raw_seek(&self, byte_offset: i64) -> Result<(), OpusFileError> {
        let result = unsafe { op_raw_seek(self.0, byte_offset) };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(())
        }
    }

    pub fn pcm_seek(&self, sample_offset: i64) -> Result<(), OpusFileError> {
        let result = unsafe { op_pcm_seek(self.0, sample_offset) };
        if result < 0 {
            Err(OpusFileError::from_i32(result).unwrap_or(OpusFileError::Unknown))
        } else {
            Ok(())
        }
    }
}

impl Drop for OggOpusFile {
    fn drop(&mut self) {
        unsafe { op_free(self.0) }
    }
}

pub struct OpusHead(pub *const bindings::OpusHead);

impl OpusHead {
    // TODO
}

pub struct OpusTags(pub *const bindings::OpusTags);

impl OpusTags {
    // TODO
}
