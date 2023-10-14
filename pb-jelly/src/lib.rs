#![warn(rust_2018_idioms)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_possible_wrap)]

#[macro_use]
extern crate serde;

use std::any::Any;
use std::collections::BTreeMap;
use std::default::Default;
use std::fmt::Debug;
use std::io::{
    Cursor,
    Error,
    ErrorKind,
    Result,
    Write,
};

use bytes::buf::{
    Buf,
    BufMut,
};

pub mod erased;
pub mod varint;
pub mod wire_format;

mod buffer;
pub use crate::buffer::{
    type_is,
    CopyWriter,
    Lazy,
    PbBuffer,
    PbBufferReader,
    PbBufferWriter,
};

mod base_types;
pub use crate::base_types::{
    ClosedProtoEnum,
    Fixed32,
    Fixed64,
    OpenProtoEnum,
    ProtoEnum,
    Sfixed32,
    Sfixed64,
    Signed32,
    Signed64,
};

mod descriptor;
pub use crate::descriptor::{
    FieldDescriptor,
    Label,
    MessageDescriptor,
    OneofDescriptor,
};

pub mod reflection;
pub use crate::reflection::Reflection;

#[cfg(test)]
mod tests;

/// Trait implemented by all the messages defined in proto files and base datatypes
/// like string, bytes, etc. The exact details of this trait is implemented for messages
/// and base types can be found at - <https://developers.google.com/protocol-buffers/docs/encoding>
pub trait Message: PartialEq + Default + Debug + Any {
    /// The `MessageDescriptor` for this message, if this is not a primitive type.
    const DESCRIPTOR: Option<MessageDescriptor> = None;

    /// Computes the number of bytes a message will take when serialized. This does not
    /// include number of bytes required for tag+wire_format or the bytes used to represent
    /// length of the message in case of LengthDelimited messages/types.
    fn compute_size(&self) -> usize;

    /// Computes the number of bytes in all grpc slices.
    /// This information is used to optimize memory allocations in zero-copy encoding.
    fn compute_grpc_slices_size(&self) -> usize {
        0
    }

    /// Serializes the message to the writer.
    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()>;

    /// Reads the message from the blob reader, copying as necessary.
    fn deserialize<B: PbBufferReader>(&mut self, r: &mut B) -> Result<()>;

    /// Helper method for serializing a message to a [Vec<u8>].
    #[inline]
    fn serialize_to_vec(&self) -> Vec<u8> {
        let size = self.compute_size() as usize;
        let mut out = Vec::with_capacity(size);
        // We know that a Cursor<Vec<u8>> only fails on u32 overflow
        // https://doc.rust-lang.org/src/std/io/cursor.rs.html#295
        self.serialize(&mut Cursor::new(&mut out)).expect("Vec u32 overflow");
        debug_assert_eq!(out.len(), size);
        out
    }

    /// Helper method for serializing a message to an arbitrary [Write].
    ///
    /// If there are [Lazy] fields in the message, their contents will be copied out.
    #[inline]
    fn serialize_to_writer<W: Write>(&self, writer: &mut W) -> Result<()> {
        let mut copy_writer = CopyWriter { writer };
        self.serialize(&mut copy_writer)?;
        Ok(())
    }

    /// Helper method for deserializing a message from a u8 slice.
    ///
    /// This will error if there are any [Lazy] fields in the message.
    #[inline]
    fn deserialize_from_slice(slice: &[u8]) -> Result<Self> {
        let mut buf = Cursor::new(slice);
        let mut m = Self::default();
        m.deserialize(&mut buf)?;
        Ok(m)
    }
}

pub fn ensure_wire_format(
    format: wire_format::Type,
    expected: wire_format::Type,
    msg_name: &str,
    field_number: usize,
) -> Result<()> {
    if format != expected {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "expected wire_format {:?}, found {:?}, at {:?}:{:?}",
                expected, format, msg_name, field_number
            ),
        ));
    }

    Ok(())
}

pub fn unexpected_eof() -> Error {
    Error::new(ErrorKind::UnexpectedEof, "unexpected EOF")
}

#[derive(Default)]
pub struct Unrecognized {
    by_field_number: BTreeMap<u32, Vec<u8>>,
}

impl Unrecognized {
    pub fn serialize(self, unrecognized_buf: &mut Vec<u8>) -> Result<()> {
        // Write out sorted by field number
        unrecognized_buf.reserve(self.by_field_number.values().map(|v| v.len()).sum());
        for serialized_field in self.by_field_number.values() {
            unrecognized_buf.write_all(&serialized_field)?;
        }
        Ok(())
    }

    pub fn gather<B: Buf>(&mut self, field_number: u32, typ: wire_format::Type, buf: &mut B) -> Result<()> {
        let mut unrecognized_buf = vec![];

        wire_format::write(field_number, typ, &mut unrecognized_buf)?;
        let advance = match typ {
            wire_format::Type::Varint => {
                if let Some(num) = varint::read(buf)? {
                    varint::write(num, &mut unrecognized_buf)?;
                } else {
                    return Err(unexpected_eof());
                };

                0
            },
            wire_format::Type::Fixed64 => 8,
            wire_format::Type::Fixed32 => 4,
            wire_format::Type::LengthDelimited => match varint::read(buf)? {
                Some(n) => {
                    varint::write(n, &mut unrecognized_buf)?;
                    n as usize
                },
                None => return Err(unexpected_eof()),
            },
        };

        if buf.remaining() < advance {
            return Err(unexpected_eof());
        }

        unrecognized_buf.put(buf.take(advance));

        self.by_field_number.insert(field_number, unrecognized_buf);

        Ok(())
    }
}

pub fn skip<B: Buf>(typ: wire_format::Type, buf: &mut B) -> Result<()> {
    let advance = match typ {
        wire_format::Type::Varint => {
            if varint::read(buf)?.is_none() {
                return Err(unexpected_eof());
            };

            0
        },
        wire_format::Type::Fixed64 => 8,
        wire_format::Type::Fixed32 => 4,
        wire_format::Type::LengthDelimited => match varint::read(buf)? {
            Some(n) => n as usize,
            None => return Err(unexpected_eof()),
        },
    };

    if buf.remaining() < advance {
        return Err(unexpected_eof());
    }

    buf.advance(advance);
    Ok(())
}

pub fn ensure_split<B: PbBufferReader>(buf: &mut B, len: usize) -> Result<B> {
    if buf.remaining() < len {
        Err(unexpected_eof())
    } else {
        Ok(buf.split(len))
    }
}
