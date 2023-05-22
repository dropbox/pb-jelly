//! Implementations of [Message] for the base types in the protobuf library.

use std::convert::TryFrom;
use std::fmt::Debug;
use std::io::{
    Error,
    ErrorKind,
    Result,
};
use std::mem;
use std::ops::{
    Deref,
    DerefMut,
};

use byteorder::{
    LittleEndian,
    ReadBytesExt,
    WriteBytesExt,
};
use bytes::{
    Buf,
    BufMut,
};
use compact_str::CompactString;

use super::{
    unexpected_eof,
    Message,
};
use crate::buffer::{
    PbBufferReader,
    PbBufferWriter,
};
use crate::reflection::Reflection;
use crate::varint;

/// Trait implemented by enums which are generated with the `err_if_default` option. Note that
/// these enums are *not* forward compatible, since they do not handle unrecognized enum variants
/// (and will fail to deserialize instead).
///
/// Note that the `Default` variant of the `ClosedProtoEnum` is not safe and should only be used
/// for deserialization.
pub trait ClosedProtoEnum: ProtoEnum + Debug {
    /// Get the name of this variant.
    fn name(self) -> &'static str;
}

/// Trait implemented by enums to help with serialization and deserialization.
///
/// Note that this is *not* a closed enum.
pub trait OpenProtoEnum: ProtoEnum {
    /// Get the name of this variant, if it is known.
    fn name(self) -> Option<&'static str>;
    /// Whether or not this enum variant is "known" (i.e. there is an associate constant with it).
    fn is_known(self) -> bool;
}

/// Marker trait for proto enums.
pub trait ProtoEnum: Copy + Default + Eq + Debug + 'static {}

impl<T, E> Message for T
where
    T: TryFrom<i32, Error = E> + Into<i32> + ProtoEnum,
    E: Debug,
{
    fn compute_size(&self) -> usize {
        let v: i32 = (*self).into();
        v.compute_size()
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        let v: i32 = (*self).into();
        v.serialize(w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let mut v: i32 = 0;
        v.deserialize(buf)?;

        *self = T::try_from(v).map_err(|u| Error::new(ErrorKind::Other, format!("invalid value for enum: {:?}", u)))?;
        Ok(())
    }
}

impl<T, E> Reflection for T
where
    T: TryFrom<i32, Error = E> + Into<i32> + ProtoEnum,
    E: Debug,
{
}

impl<T: Message> Message for Option<T> {
    fn compute_size(&self) -> usize {
        if let Some(ref inner) = *self {
            inner.compute_size()
        } else {
            0
        }
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        if let Some(ref inner) = *self {
            inner.serialize(w)
        } else {
            Ok(())
        }
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        if buf.has_remaining() {
            if self.is_none() {
                *self = Some(T::default());
            }
            self.as_mut().unwrap().deserialize(buf)
        } else {
            *self = None;
            Ok(())
        }
    }
}

impl<T: Message> Reflection for Option<T> {}

impl Message for u32 {
    fn compute_size(&self) -> usize {
        varint::serialized_length(u64::from(*self))
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        varint::write(u64::from(*self), w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        *self = match varint::read(buf)? {
            Some(n) => n as u32,
            None => return Err(unexpected_eof()),
        };

        Ok(())
    }
}

impl Reflection for u32 {}

impl Message for i32 {
    fn compute_size(&self) -> usize {
        varint::serialized_length(*self as u64)
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        varint::write(*self as u64, w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        *self = match varint::read(buf)? {
            Some(n) => n as i32,
            None => return Err(unexpected_eof()),
        };

        Ok(())
    }
}

impl Reflection for i32 {}

impl Message for u64 {
    fn compute_size(&self) -> usize {
        varint::serialized_length(*self)
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        varint::write(*self, w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        *self = match varint::read(buf)? {
            Some(n) => n as u64,
            None => return Err(unexpected_eof()),
        };

        Ok(())
    }
}

impl Reflection for u64 {}

impl Message for i64 {
    fn compute_size(&self) -> usize {
        varint::serialized_length(*self as u64)
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        varint::write(*self as u64, w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        *self = match varint::read(buf)? {
            Some(n) => n as i64,
            None => return Err(unexpected_eof()),
        };

        Ok(())
    }
}

impl Reflection for i64 {}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Signed64(pub i64);

impl Signed64 {
    fn encode(n: i64) -> u64 {
        ((n << 1) ^ (n >> 63)) as u64
    }

    fn decode(n: u64) -> i64 {
        ((n >> 1) as i64) ^ (-((n & 1) as i64))
    }
}

impl Message for Signed64 {
    fn compute_size(&self) -> usize {
        varint::serialized_length(Self::encode(self.0))
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        varint::write(Self::encode(self.0), w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        self.0 = match varint::read(buf)? {
            Some(n) => Self::decode(n as u64),
            None => return Err(unexpected_eof()),
        };
        Ok(())
    }
}

impl Reflection for Signed64 {}

impl Deref for Signed64 {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl DerefMut for Signed64 {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Signed32(pub i32);

impl Signed32 {
    fn encode(n: i32) -> u32 {
        ((n << 1) ^ (n >> 31)) as u32
    }

    fn decode(n: u32) -> i32 {
        ((n >> 1) as i32) ^ (-((n & 1) as i32))
    }
}

impl Message for Signed32 {
    fn compute_size(&self) -> usize {
        varint::serialized_length(u64::from(Self::encode(self.0)))
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        varint::write(u64::from(Self::encode(self.0)), w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        self.0 = match varint::read(buf)? {
            Some(n) => Self::decode(n as u32),
            None => return Err(unexpected_eof()),
        };
        Ok(())
    }
}

impl Reflection for Signed32 {}

impl Deref for Signed32 {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl DerefMut for Signed32 {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Fixed64(pub u64);

impl Message for Fixed64 {
    fn compute_size(&self) -> usize {
        8
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_u64::<LittleEndian>(self.0)?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let val = buf.reader().read_u64::<LittleEndian>()?;
        self.0 = val;
        Ok(())
    }
}

impl Reflection for Fixed64 {}

impl Deref for Fixed64 {
    type Target = u64;

    fn deref(&self) -> &u64 {
        &self.0
    }
}

impl DerefMut for Fixed64 {
    fn deref_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Fixed32(pub u32);

impl Message for Fixed32 {
    fn compute_size(&self) -> usize {
        4
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_u32::<LittleEndian>(self.0)?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let val = buf.reader().read_u32::<LittleEndian>()?;
        self.0 = val;
        Ok(())
    }
}

impl Reflection for Fixed32 {}

impl Deref for Fixed32 {
    type Target = u32;

    fn deref(&self) -> &u32 {
        &self.0
    }
}

impl DerefMut for Fixed32 {
    fn deref_mut(&mut self) -> &mut u32 {
        &mut self.0
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Sfixed64(pub i64);

impl Message for Sfixed64 {
    fn compute_size(&self) -> usize {
        8
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_i64::<LittleEndian>(self.0)?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let val = buf.reader().read_i64::<LittleEndian>()?;
        self.0 = val;
        Ok(())
    }
}

impl Reflection for Sfixed64 {}

impl Deref for Sfixed64 {
    type Target = i64;

    fn deref(&self) -> &i64 {
        &self.0
    }
}

impl DerefMut for Sfixed64 {
    fn deref_mut(&mut self) -> &mut i64 {
        &mut self.0
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct Sfixed32(pub i32);

impl Message for Sfixed32 {
    fn compute_size(&self) -> usize {
        4
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_i32::<LittleEndian>(self.0)?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let val = buf.reader().read_i32::<LittleEndian>()?;
        self.0 = val;
        Ok(())
    }
}

impl Reflection for Sfixed32 {}

impl Deref for Sfixed32 {
    type Target = i32;

    fn deref(&self) -> &i32 {
        &self.0
    }
}

impl DerefMut for Sfixed32 {
    fn deref_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

impl Message for f32 {
    fn compute_size(&self) -> usize {
        4
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_f32::<LittleEndian>(*self)?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let val = buf.reader().read_f32::<LittleEndian>()?;
        *self = val;
        Ok(())
    }
}

impl Reflection for f32 {}

impl Message for f64 {
    fn compute_size(&self) -> usize {
        8
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_f64::<LittleEndian>(*self)?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let val = buf.reader().read_f64::<LittleEndian>()?;
        *self = val;
        Ok(())
    }
}

impl Reflection for f64 {}

impl Message for bool {
    fn compute_size(&self) -> usize {
        1
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        varint::write(*self as u64, w)
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        *self = match varint::read(buf)? {
            Some(n) => n > 0,
            None => return Err(unexpected_eof()),
        };

        Ok(())
    }
}

impl Reflection for bool {}

impl Message for Vec<u8> {
    fn compute_size(&self) -> usize {
        self.len()
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_all(&self[..])?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        let cnt = buf.remaining();
        if !self.is_empty() {
            self.clear();
        }
        self.reserve(cnt);
        self.put(buf);
        Ok(())
    }
}

impl Reflection for Vec<u8> {}

impl Message for String {
    fn compute_size(&self) -> usize {
        self.len()
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_all(self.as_bytes())?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        // Reuse any existing allocation held by `self`.
        let mut bytes = mem::take(self).into_bytes();
        bytes.clear();

        // Write all of our message into `bytes`.
        let cnt = buf.remaining();
        bytes.reserve(cnt);
        bytes.put(buf);

        match String::from_utf8(bytes) {
            Ok(s) => {
                // Valid UTF-8
                *self = s;
                Ok(())
            }
            Err(e) => {
                // Invalid UTF-8. Grab the decoding error, then restore the allocation
                let error = e.utf8_error();
                bytes = e.into_bytes();
                bytes.clear();
                *self = String::from_utf8(bytes).unwrap();
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
            }
        }
    }
}

impl Reflection for String {}

impl Message for compact_str::CompactString {
    fn compute_size(&self) -> usize {
        self.len()
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        w.write_all(self.as_bytes())?;
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
        match CompactString::from_utf8_buf(buf) {
            // success! set ourself equal to the CompactString we just created
            Ok(compact) => {
                *self = compact;
                Ok(())
            },
            // error! there was invalid UTF-8, return an error
            Err(_) => Err(std::io::ErrorKind::InvalidData.into()),
        }
    }
}

impl Reflection for compact_str::CompactString {}

impl Message for () {
    fn compute_size(&self) -> usize {
        0
    }

    fn serialize<W: PbBufferWriter>(&self, _w: &mut W) -> Result<()> {
        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, _buf: &mut B) -> Result<()> {
        Ok(())
    }
}

impl Reflection for () {}

macro_rules! fixed_length_impls {
    ($($len: tt)*) => ($(
        impl Message for [u8; $len] {
            fn compute_size(&self) -> usize {
                $len
            }
            fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
                w.write_all(self)?;
                Ok(())
            }

            fn deserialize<B: PbBufferReader>(&mut self, buf: &mut B) -> Result<()> {
                if buf.remaining() != $len {
                    return Err(Error::new(ErrorKind::InvalidData, concat!("not of length ", stringify!($len))));
                }
                buf.copy_to_slice(self);
                Ok(())
            }
        }

        impl Reflection for [u8; $len] {}
    )*);
}

fixed_length_impls!(0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);

#[test]
fn test_sint32_encoding() {
    let pairs: Vec<(i32, u32)> = vec![
        (0, 0),
        (-1, 1),
        (1, 2),
        (-2, 3),
        (2147483647, 4294967294),
        (-2147483648, 4294967295),
    ];
    for (signed, unsigned) in pairs {
        assert_eq!(Signed32::encode(signed), unsigned);
        assert_eq!(Signed32::decode(unsigned), signed);
    }
}

#[test]
fn test_sint64_encoding() {
    let pairs: Vec<(i64, u64)> = vec![
        (0, 0),
        (-1, 1),
        (1, 2),
        (-2, 3),
        (2147483647, 4294967294),
        (-2147483648, 4294967295),
        (9223372036854775807, 18446744073709551614),
        (-9223372036854775808, 18446744073709551615),
    ];
    for (signed, unsigned) in pairs {
        assert_eq!(Signed64::encode(signed), unsigned);
        assert_eq!(Signed64::decode(unsigned), signed);
    }
}
