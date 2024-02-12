use std::io;

use crate::{
    ensure_split,
    ensure_wire_format,
    varint,
    wire_format,
    Message,
    PbBufferReader,
    PbBufferWriter,
};

pub fn deserialize_packed<B: PbBufferReader, T: Message>(
    buf: &mut B,
    typ: wire_format::Type,
    expected_wire_format: wire_format::Type,
    msg_name: &'static str,
    field_number: u32,
    out: &mut Vec<T>,
) -> io::Result<()> {
    match typ {
        wire_format::Type::LengthDelimited => {
            let len = varint::ensure_read(buf)?;
            let mut vals = ensure_split(buf, len as usize)?;
            while vals.has_remaining() {
                let mut val: T = Default::default();
                val.deserialize(&mut vals)?;
                out.push(val);
            }
        },
        _ => {
            ensure_wire_format(typ, expected_wire_format, msg_name, field_number)?;
            let mut val: T = Default::default();
            val.deserialize(buf)?;
            out.push(val);
        },
    }
    Ok(())
}

pub fn deserialize_length_delimited<B: PbBufferReader, T: Message>(
    buf: &mut B,
    typ: wire_format::Type,
    msg_name: &'static str,
    field_number: u32,
) -> io::Result<T> {
    ensure_wire_format(typ, wire_format::Type::LengthDelimited, msg_name, field_number)?;
    let len = varint::ensure_read(buf)?;
    let mut next = ensure_split(buf, len as usize)?;
    let mut val: T = Default::default();
    val.deserialize(&mut next)?;
    Ok(val)
}

pub fn deserialize_known_length<B: PbBufferReader, T: Message>(
    buf: &mut B,
    typ: wire_format::Type,
    expected_wire_format: wire_format::Type,
    msg_name: &'static str,
    field_number: u32,
) -> io::Result<T> {
    ensure_wire_format(typ, expected_wire_format, msg_name, field_number)?;
    let mut val: T = Default::default();
    val.deserialize(buf)?;
    Ok(val)
}

pub fn serialize_scalar<W: PbBufferWriter, T: Message>(
    w: &mut W,
    val: &T,
    field_number: u32,
    wire_format: wire_format::Type,
) -> io::Result<()> {
    if *val != T::default() {
        wire_format::write(field_number, wire_format, w)?;
        if let wire_format::Type::LengthDelimited = wire_format {
            let l = val.compute_size();
            varint::write(l as u64, w)?;
        }
        val.serialize(w)?;
    }
    Ok(())
}

pub fn compute_size_scalar<T: Message>(val: &T, field_number: u32, wire_format: wire_format::Type) -> usize {
    let mut size = 0;
    if *val != T::default() {
        size += wire_format::serialized_length(field_number);
        let l = val.compute_size();
        if let wire_format::Type::LengthDelimited = wire_format {
            size += varint::serialized_length(l as u64);
        }
        size += l;
    }
    size
}
