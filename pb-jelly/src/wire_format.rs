use std::io::{
    self,
    Write,
};

use bytes::Buf;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Type {
    Varint = 0,
    Fixed64 = 1,
    LengthDelimited = 2,
    Fixed32 = 5,
}

impl Type {
    const fn as_u32(self) -> u32 {
        match self {
            Type::Varint => 0,
            Type::Fixed64 => 1,
            Type::LengthDelimited => 2,
            Type::Fixed32 => 5,
        }
    }

    fn from_u32(val: u32) -> io::Result<Self> {
        match val {
            0 => Ok(Type::Varint),
            1 => Ok(Type::Fixed64),
            2 => Ok(Type::LengthDelimited),
            5 => Ok(Type::Fixed32),
            v => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Unexpected tag type: {}", v),
            )),
        }
    }
}

#[inline]
pub const fn serialized_length(field_number: u32) -> usize {
    super::varint::serialized_length((field_number << 3) as u64)
}

#[inline]
pub fn write<W: Write>(field_number: u32, typ: Type, w: &mut W) -> io::Result<()> {
    let encoded = (field_number << 3) | typ.as_u32();
    super::varint::write(u64::from(encoded), w)
}

#[inline]
pub fn read<B: Buf>(buf: &mut B) -> io::Result<Option<(u32, Type)>> {
    let encoded = match super::varint::read(buf)? {
        Some(v) => v,
        None => return Ok(None),
    };

    let field_number = (encoded >> 3) as u32;
    let typ = Type::from_u32((encoded & 0x7) as u32)?;
    Ok(Some((field_number, typ)))
}
