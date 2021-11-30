//! This module exposes two fundamental concepts:
//!
//! [PbBufferReader]
//! A [PbBufferReader] is something which a [Message] can be deserialized from. In the common case,
//! this means that the relevant bytes are copied out of the underlying store and copied into an
//! appropriate struct which implements [Message].
//!
//! [PbBufferWriter]
//! A [PbBufferWriter] is something which a [Message] can be serialized to. In the common case, this
//! means that the relevant bytes are copied out of the concrete struct and into the underlying
//! data store.
//!
//! # Zerocopy serialization and deserialization and `Lazy`
//!
//! There are cases where we want to minimize the number of times we copy the data contained within
//! a message. Especially on resource-constrained hardware (mostly MP OSDs), we want to avoid the cost
//! of copying large buffers during serialization and deserialization.
//!
//! To support this, messages may contain zerocopy fields using the [`Lazy`] type.
//! `pb-jelly-gen` may generate these using the `CORD`, `grpc_slices`, or `zero_copy` options; they
//! use different underlying types, which must implement [PbBuffer], but they all behave similarly.
//!
//! [PbBufferReader] and a [PbBufferWriter] have the opportunity to recognize [Lazy] fields.
//! At deserialization time, if [PbBufferReader] is used with a compatible [Lazy] field, instead of
//! allocating, it may simply store a reference to its underlying input buffer in the [Lazy].
//! Similarly, at serialization time, a [PbBufferWriter] used with a compatible [Lazy] may copy a
//! reference to the [Lazy] field into its output buffer, rather than copying its content.
//!
//! Request (bytes on the wire)
//!     |
//!     v
//! RPC Framework (with an underlying allocator, e.g. blob::Blob or grpc::Slice)
//!     |
//!     v
//! BR: [PbBufferReader] deserializes the struct using RPC framework's allocator.
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
//! BW: [PbBufferWriter] serializes the struct using RPC framework's allocator.
//!     |
//!     v
//! RPC Framework (with an underlying allocator, e.g. blob::Blob or grpc::Slice)
//!     |
//!     v
//! Response (bytes on the wire).
//!
//!
//! In the status quo, the behavior is as follows:
//!
//! `blob_pb::WrappedBlob` and `blob_pb::VecSlice` allow zero-copy deserialization -> serialization,
//! provided that their respective [PbBufferWriter]s are used.
//! Converting from `blob_pb::WrappedBlob` to a `blob_pb::VecSlice` is zero-copy.
//! Converting from a `blob_pb::VecSlice` to a `blob_pb::Blob` requires a single copy.

use std::any::Any;
use std::fmt::{
    self,
    Debug,
};
use std::io::{
    Cursor,
    Result,
    Write,
};

use bytes::{
    Buf,
    Bytes,
};

use super::{
    Message,
    Reflection,
};

/// A stand-in trait for any backing buffer store.
/// `PbBuffer`s are expected to own references to the data they reference, and should be cheap
/// (constant-time) to clone.
#[allow(clippy::len_without_is_empty)]
pub trait PbBuffer: Any + Sized {
    /// Returns the length of the data contained in this buffer.
    fn len(&self) -> usize;
    /// Fallback method to read the contents of `self`. This method is expected to write exactly
    /// `self.len()` bytes into `writer`, or fail with an error.
    ///
    /// This method is used to write `Lazy` fields to incompatible [`PbBufferWriter`]s.
    fn copy_to_writer<W: Write + ?Sized>(&self, writer: &mut W) -> Result<()>;
    /// Fallback method to create an instance of this `PbBuffer`.
    ///
    /// This method is used to read `Lazy` fields from incompatible [`PbBufferReader`]s.
    fn copy_from_reader<B: Buf + ?Sized>(reader: &mut B) -> Result<Self>;
}

/// If `B1` and `B2` are the same type, returns a function to cast `B1 -> B2`; otherwise None.
/// Used to implement [PbBuffer] casting.
pub fn type_is<B1: 'static, B2: 'static>() -> Option<fn(B1) -> B2> {
    let f: fn(B1) -> B1 = |x| x;
    // If B1 = B2, then this cast should succeed!
    (&f as &dyn Any).downcast_ref::<fn(B1) -> B2>().copied()
}

/// All concrete types which are used for deserialization should implement
/// [PbBufferReader], which includes functions to convert to and from [PbBuffer].
pub trait PbBufferReader: Buf {
    /// Attempt to read into a compatible [PbBuffer], avoiding a copy if possible.
    /// The implementation should dispatch on the type `B`. If unsupported,
    /// the reader may fall back to [PbBuffer::copy_from_reader].
    fn read_buffer<B: PbBuffer>(&mut self) -> Result<B> {
        B::copy_from_reader(self)
    }

    /// Advance the interal cursor by `at`, and return a [PbBufferReader] corresponding to the
    /// traversed indices (i.e. self.position..self.position + at).
    fn split(&mut self, at: usize) -> Self;
}

/// All concrete types used for serialization should implement [PbBufferWriter] in order to support
/// serializing [Lazy] fields without copies.
pub trait PbBufferWriter: Write {
    /// Attempt to write a zerocopy buffer into `self`. If `B` is not zero-copy-supported
    /// by the [PbBufferWriter], this may read/copy the bytes out from `buf`.
    fn write_buffer<B: PbBuffer>(&mut self, buf: &B) -> Result<()>;
}

/// A wrapper around a [PbBuffer], which implements [Message].
#[derive(Clone, PartialEq)]
pub struct Lazy<B> {
    // TODO: Make this not an `Option` by giving `VecSlice` a cheap `Default` impl
    contents: Option<B>,
}

impl<B> Default for Lazy<B> {
    fn default() -> Self {
        Self { contents: None }
    }
}

impl<B> Lazy<B> {
    pub fn new(r: B) -> Self {
        Self { contents: Some(r) }
    }

    pub fn into_buffer(self) -> B
    where
        B: Default,
    {
        self.contents.unwrap_or_default()
    }
}

impl<B> Debug for Lazy<B> {
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
        self.contents = Some(r.read_buffer()?);
        Ok(())
    }
}

impl<B: PbBuffer + PartialEq> Reflection for Lazy<B> {}

impl<'a> PbBufferReader for Cursor<&'a [u8]> {
    fn split(&mut self, at: usize) -> Self {
        let pos = self.position() as usize;
        self.advance(at);
        let new_slice = &self.get_ref()[pos..pos + at];
        Self::new(new_slice)
    }
}

impl PbBuffer for Bytes {
    #[inline]
    fn len(&self) -> usize {
        self.len()
    }

    fn copy_to_writer<W: Write + ?Sized>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(&self)
    }

    fn copy_from_reader<B: Buf + ?Sized>(reader: &mut B) -> Result<Self> {
        let len = reader.remaining();
        Ok(reader.copy_to_bytes(len))
    }
}

impl PbBufferReader for Cursor<Bytes> {
    fn read_buffer<B: PbBuffer>(&mut self) -> Result<B> {
        if let Some(cast) = type_is::<Bytes, B>() {
            let bytes = self.get_ref().slice((self.position() as usize)..);
            Ok(cast(bytes))
        } else {
            B::copy_from_reader(self)
        }
    }

    #[inline]
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
        buf.copy_to_writer(self)
    }
}

impl<'a> PbBufferWriter for Cursor<&'a mut [u8]> {
    /// Note: this implementation freely copies the data out of `buf`.
    #[inline]
    fn write_buffer<B: PbBuffer>(&mut self, buf: &B) -> Result<()> {
        buf.copy_to_writer(self)
    }
}

/// A wrapper around a [Write] which copies all [Lazy] data into the underlying [Write]r.
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
        buf.copy_to_writer(self.writer)
    }
}

#[test]
fn test_lazy_bytes_deserialize() {
    let mut lazy = Lazy::<Bytes>::default();
    let bytes = Bytes::from_static(b"asdfasdf");
    lazy.deserialize(&mut Cursor::new(bytes.clone()))
        .expect("failed to deserialize");
    let deserialized = lazy.into_buffer();
    assert_eq!(deserialized, bytes, "The entire buffer should be copied");
    assert_eq!(
        deserialized.as_ptr(),
        bytes.as_ptr(),
        "The Bytes instance should be cloned"
    )
}
