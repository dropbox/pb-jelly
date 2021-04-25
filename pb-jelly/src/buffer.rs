//! This module exposes three fundamental concepts:
//!
//! [PbBufferReader]
//! A [PbBufferReader] is something which a [Message] can be deserialized from. In the common case,
//! this means that the relevant bytes are copied out of the underlying store and copied into an
//! appropriate struct which implements [Message]. The underlying data store is referred to as a
//! [PbBuffer].
//!
//! [PbBufferWriter]
//! A [PbBufferWriter] is something which a [Message] can be serialized to. In the common case, this
//! means that the relevant bytes are copied out of the concrete struct and into the underlying
//! data store. The underlying data store is referred to as a [PbBuffer].
//!
//! [Lazy]
//! The interesting trick of the [PbBuffer]/[PbBufferReader]/[PbBufferWriter] abstraction is that there are
//! cases where we want to minimize the number of times we copy the data contained within a message
//! (or field of a message). Especially on resource-constrained hardware (mostly MP OSDs), we want
//! to be able to avoid the cost of serialization and deserialization.
//!
//! To support this, a [PbBufferReader] and a [PbBufferWriter] may use *compatible* backing stores, and
//! expose a [Lazy] message type which represents the not-yet-deserialized data. The flow is
//! roughly as follows:
//!
//! Request (bytes on the wire)
//!     |
//!     v
//! RPC Framework (with an underlying allocator, e.g. blob::Blob or grpc::Slice)
//!     |
//!     v
//! BR: PbBufferReader deserializes the struct using RPC framework's allocator.
//!     |
//!     v
//! Request (concrete struct containing a [Lazy] field)
//!     |
//!     v
//! RPC handler (doesn't modify the [Lazy] field)
//!     |
//!     v
//! Response (concrete struct containing a [Lazy] field)
//!     |
//!     v
//! BW: PbBufferWriter serializes the struct using RPC framework's allocator.
//!     |
//!     v
//! RPC Framework (with an underlying allocator, e.g. blob::Blob or grpc::Slice)
//!     |
//!     v
//! Response (bytes on the wire).
//!
//! In order for [Lazy] to work (i.e. minimize copies), we must have the [PbBufferReader] and the
//! [PbBufferWriter] use compatible underlying allocators. Compatibility is defined by the
//! both of the following:
//! - the ability for the [PbBufferWriter] to successfully call `write_buffer` for the appropriate
//! [PbBuffer] type, which writes non-deserialized data back into the data store.
//! - the ability to use [PbBuffer::into_reader] and [PbBufferReader::as_buffer] to *convert* between
//! underlying data stores.
//!
//! Roughly speaking, [Lazy] converts to the appropriate [PbBuffer] type when `deserialize` is
//! called, and then lazily writes itself when `serialize` is called.
//!
//! In the status quo, the behavior is as follows:
//!
//! `blob_pb::WrappedBlob` and `blob_pb::VecSlice` allow zero-copy deserialization -> serialization,
//! provided that their respective [PbBufferWriter]s are used.
//! Converting from `blob_pb::WrappedBlob` to a `blob_pb::VecSlice` is zero-copy.
//! Converting from a `blob_pb::VecSlice` to a `blob_pb::Blob` requires a single copy.

use std::any::{
    type_name,
    Any,
};
use std::default::Default;
use std::fmt::{
    self,
    Debug,
};
use std::io::{
    Cursor,
    Error,
    ErrorKind,
    Result,
    Write,
};

use bytes::{
    Buf,
    Bytes,
};

use super::Message;

/// A stand-in trait for any backing buffer store. Required to be object-safe for lazy evaluation.
/// PbBuffers are expected to own references to the data they reference, and should be cheap
/// (constant-time) to clone.
#[allow(clippy::len_without_is_empty)]
pub trait PbBuffer: Any + Clone + Default {
    type Reader: PbBufferReader;
    fn len(&self) -> usize;
    fn into_reader(self) -> Self::Reader;

    /// Deserialize this buffer from a reader. Unless overridden, this will error
    /// if the reader does not support casting to [Self].
    fn from_reader<R: PbBufferReader>(reader: &mut R) -> Result<Self> {
        reader.as_buffer::<Self>()
    }
}

/// If `B1` and `B2` are the same type, returns the passed-in buffer;
/// otherwise, returns an error.
pub fn cast_buffer<B1: PbBuffer, B2: PbBuffer>(buf: B1) -> Result<B2> {
    let mut buf = Some(buf);
    if let Some(buf) = (&mut buf as &mut dyn Any).downcast_mut::<Option<B2>>() {
        Ok(buf.take().unwrap())
    } else {
        Err(Error::new(
            ErrorKind::InvalidInput,
            format!(
                "This reader produces buffers of type {}, not {}",
                type_name::<B1>(),
                type_name::<B2>()
            ),
        ))
    }
}

/// All concrete types which are used for deserialization should implement
/// [PbBufferReader], which includes functions to convert to and from [PbBuffer].
pub trait PbBufferReader: Buf + Sized {
    /// Get a reference to the underlying [PbBuffer]. This is expected to be cheap,
    /// if supported, or return error.
    /// The implementation should dispatch on the type `B` and return an error
    /// if the requested buffer type is unknown.
    fn as_buffer<B: PbBuffer>(&self) -> Result<B> {
        Err(Error::new(
            ErrorKind::InvalidInput,
            "Taking ownership of the PbBuffer from this reader is not supported",
        ))
    }

    /// Advance the interal cursor by `at`, and return a [PbBufferReader] corresponding to the
    /// traversed indices (i.e. self.position..self.position + at).
    fn split(&mut self, at: usize) -> Self;
}

/// All concrete types used for serialization should implement [PbBufferWriter] in order to support
/// serializing lazily-evaluated types without copies.
pub trait PbBufferWriter: Write {
    /// Attempt to write a lazily-evaluated buffer into [Self]. If the underlying [B] is not
    /// zero-copy-supported by the [PbBufferWriter], this should read/copy the bytes out from [B].
    fn write_buffer<B: PbBuffer>(&mut self, buf: &B) -> Result<()>;
}

/// A wrapper around a lazily-evaluted [PbBufferReader] which implements [Message].
#[derive(Clone, Default, PartialEq)]
pub struct Lazy<B: PbBuffer> {
    contents: Option<B>,
}

impl<B: PbBuffer> Lazy<B> {
    pub fn new(r: B) -> Self {
        Self { contents: Some(r) }
    }

    pub fn into_buffer(self) -> B {
        self.contents.unwrap_or_default()
    }
}

impl<B: PbBuffer> Debug for Lazy<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lazy")
            .field("contents", &self.contents.as_ref().map(|_| "_"))
            .finish()
    }
}

impl<B: PbBuffer + PartialEq> Message for Lazy<B> {
    fn compute_size(&self) -> usize {
        self.contents.as_ref().map(PbBuffer::len).unwrap_or(0)
    }

    fn compute_grpc_slices_size(&self) -> usize {
        self.contents.as_ref().map(PbBuffer::len).unwrap_or(0)
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        if let Some(ref contents) = self.contents {
            w.write_buffer(contents)?;
        }
        Ok(())
    }

    fn deserialize<R: PbBufferReader>(&mut self, r: &mut R) -> Result<()> {
        self.contents = Some(B::from_reader(r)?);
        Ok(())
    }
}

impl<'a> PbBufferReader for Cursor<&'a [u8]> {
    fn split(&mut self, at: usize) -> Self {
        let pos = self.position() as usize;
        self.advance(at);
        let new_slice = &self.get_ref()[pos..pos + at];
        Self::new(new_slice)
    }
}

impl PbBuffer for Bytes {
    type Reader = Cursor<Bytes>;
    fn len(&self) -> usize {
        self.len()
    }
    fn into_reader(self) -> Cursor<Bytes> {
        Cursor::new(self)
    }
}

impl PbBufferReader for Cursor<Bytes> {
    fn as_buffer<B: PbBuffer>(&self) -> Result<B> {
        let bytes = self.get_ref().slice((self.position() as usize)..);
        cast_buffer(bytes)
    }

    fn split(&mut self, at: usize) -> Self {
        let pos = self.position() as usize;
        self.advance(at);
        let new_slice = self.get_ref().slice(pos..(pos + at));
        Self::new(new_slice)
    }
}

impl<'a> PbBufferWriter for Cursor<&'a mut Vec<u8>> {
    /// Note: this implementation freely copies the data out of `buf`.
    #[inline]
    fn write_buffer<B: PbBuffer>(&mut self, buf: &B) -> Result<()> {
        let mut reader = buf.clone().into_reader();
        while reader.has_remaining() {
            let n = {
                let bytes = reader.chunk();
                self.write_all(bytes)?;
                bytes.len()
            };
            reader.advance(n);
        }
        Ok(())
    }
}

impl<'a> PbBufferWriter for Cursor<&'a mut [u8]> {
    /// Note: this implementation freely copies the data out of `buf`.
    #[inline]
    fn write_buffer<B: PbBuffer>(&mut self, buf: &B) -> Result<()> {
        let mut reader = buf.clone().into_reader();
        while reader.has_remaining() {
            let n = {
                let bytes = reader.chunk();
                self.write_all(bytes)?;
                bytes.len()
            };
            reader.advance(n);
        }
        Ok(())
    }
}

/// A wrapper around a [Write] which copies all the data into the underlying [Write]r.
pub struct CopyWriter<'a, W: Write> {
    pub writer: &'a mut W,
}

impl<'a, W: Write + 'a> Write for CopyWriter<'a, W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writer.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}

impl<'a, W: Write + 'a> PbBufferWriter for CopyWriter<'a, W> {
    /// Note: this implementation freely copies the data out of `buf`.
    #[inline]
    fn write_buffer<B: PbBuffer>(&mut self, buf: &B) -> Result<()> {
        let mut reader = buf.clone().into_reader();
        while reader.has_remaining() {
            let n = {
                let bytes = reader.chunk();
                self.write_all(bytes)?;
                bytes.len()
            };
            reader.advance(n);
        }
        Ok(())
    }
}
