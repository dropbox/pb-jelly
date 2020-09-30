use std::any::Any;
use std::io::{
    Error,
    ErrorKind,
    Result,
    Write,
};
use std::rc::Rc;

use bytes::Buf;

use crate::varint;
use crate::wire_format;

use super::{
    cast_buffer,
    ensure_split,
    ensure_wire_format,
    skip,
    Lazy,
    Message,
    PbBuffer,
    PbBufferReader,
    PbBufferWriter,
};

/// A wrapper around a `Vec` which owns its contents.
#[derive(Clone, PartialEq, Debug, Default)]
struct VecReader {
    contents: Rc<Vec<u8>>,
    position: usize,
    end: usize,
}

impl PbBuffer for VecReader {
    type Reader = VecReader;
    fn len(&self) -> usize {
        self.remaining()
    }
    fn into_reader(self) -> VecReader {
        self
    }
}

impl From<Vec<u8>> for VecReader {
    fn from(v: Vec<u8>) -> Self {
        VecReader {
            position: 0,
            end: v.len(),
            contents: Rc::new(v),
        }
    }
}

impl Buf for VecReader {
    fn remaining(&self) -> usize {
        debug_assert!(self.position <= self.end);
        self.end - self.position
    }

    fn bytes(&self) -> &[u8] {
        &self.contents[self.position..self.end]
    }

    fn advance(&mut self, cnt: usize) {
        debug_assert!(self.position + cnt <= self.end);
        self.position += cnt;
    }
}

impl PbBufferReader for VecReader {
    fn as_buffer<B: PbBuffer>(&self) -> Result<B> {
        cast_buffer(self.clone())
    }

    fn split(&mut self, at: usize) -> Self {
        debug_assert!(self.position + at <= self.end);

        let pos = self.position;
        self.position += at;

        Self {
            contents: Rc::clone(&self.contents),
            position: pos,
            end: self.position,
        }
    }
}

/// A kind of dumb zero-copy implementation in memory.
#[derive(Default, Debug)]
struct VecWriter {
    contents: Vec<VecReader>,
}

impl VecWriter {
    fn serialized(&self) -> Vec<u8> {
        let mut out = vec![];
        for content in &self.contents {
            for b in content.bytes() {
                out.push(*b);
            }
        }
        out
    }
}

impl Write for VecWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.contents.push(Vec::from(buf).into());
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl PbBufferWriter for VecWriter {
    /// Attempt to write non-deserialized buf into [Self]
    fn write_buffer<B: PbBuffer>(&mut self, buf: &B) -> Result<()> {
        if let Some(buf) = (buf as &dyn Any).downcast_ref::<VecReader>() {
            self.contents.push(buf.clone());
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Can't zero-copy non-VecReader"))
        }
    }
}

#[derive(Debug, PartialEq, Default, Clone)]
struct TestMessage {
    normal_data: Vec<u8>,
    payload: Lazy<VecReader>,
}

impl Message for TestMessage {
    fn compute_size(&self) -> usize {
        let mut size = wire_format::serialized_length(1);
        let l = self.normal_data.compute_size();
        size += varint::serialized_length(l as u64);
        size += l;

        size += wire_format::serialized_length(2);
        let l = self.payload.compute_size();
        size += varint::serialized_length(l as u64);
        size += l;

        size
    }

    fn compute_grpc_slices_size(&self) -> usize {
        self.normal_data.compute_grpc_slices_size() + self.payload.compute_grpc_slices_size()
    }

    fn serialize<W: PbBufferWriter>(&self, w: &mut W) -> Result<()> {
        wire_format::write(1, wire_format::Type::LengthDelimited, w)?;
        let l = self.normal_data.compute_size();
        varint::write(l as u64, w)?;
        self.normal_data.serialize(w)?;

        wire_format::write(2, wire_format::Type::LengthDelimited, w)?;
        let l = self.payload.compute_size();
        varint::write(l as u64, w)?;
        self.payload.serialize(w)?;

        Ok(())
    }

    fn deserialize<B: PbBufferReader>(&mut self, mut buf: &mut B) -> Result<()> {
        while let Some((field_number, typ)) = wire_format::read(&mut buf)? {
            match field_number {
                1 => {
                    ensure_wire_format(typ, wire_format::Type::LengthDelimited, "normal_data", 1)?;
                    let len = varint::ensure_read(&mut buf)?;
                    let mut next = ensure_split(buf, len as usize)?;
                    let mut val: Vec<u8> = Default::default();
                    val.deserialize(&mut next)?;
                    self.normal_data = val;
                },
                2 => {
                    ensure_wire_format(typ, wire_format::Type::LengthDelimited, "payload", 2)?;
                    let len = varint::ensure_read(&mut buf)?;
                    let mut next = ensure_split(buf, len as usize)?;
                    let mut val: Lazy<VecReader> = Default::default();
                    val.deserialize(&mut next)?;
                    self.payload = val;
                },
                _ => {
                    skip(typ, &mut buf)?;
                },
            }
        }

        Ok(())
    }
}

#[test]
fn test_serialize_message_into_slice() {
    let mut message = TestMessage::default();
    message.normal_data = vec![1, 2, 3];
    message.payload = Lazy::new(vec![4, 5, 6].into());

    let serialized = message.serialize_to_vec();
    assert_eq!(
        TestMessage::deserialize_from_slice(&serialized).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );
    assert_eq!(serialized.len(), message.compute_size());

    let mut deserialized = TestMessage::default();
    deserialized
        .deserialize(&mut VecReader::from(serialized.clone()))
        .unwrap();

    assert_eq!(deserialized.normal_data, message.normal_data);
    assert_eq!(
        deserialized.payload.into_buffer().bytes(),
        message.payload.into_buffer().bytes()
    );

    assert_eq!(serialized, vec![10, 3, 1, 2, 3, 18, 3, 4, 5, 6]);
}

#[test]
fn test_serialize_message_into_vec_writer() {
    let mut message = TestMessage::default();
    message.normal_data = vec![1, 2, 3];
    message.payload = Lazy::new(vec![4, 5, 6].into());

    let mut out = VecWriter::default();
    message.serialize(&mut out).unwrap();

    let serialized = out.serialized();

    assert_eq!(
        TestMessage::deserialize_from_slice(&serialized).unwrap_err().kind(),
        ErrorKind::InvalidInput
    );

    let mut deserialized = TestMessage::default();
    deserialized
        .deserialize(&mut VecReader::from(serialized.clone()))
        .unwrap();
    assert_eq!(serialized.len(), message.compute_size());

    assert_eq!(deserialized.normal_data, message.normal_data);
    assert_eq!(
        deserialized.payload.into_buffer().bytes(),
        message.payload.into_buffer().bytes()
    );

    assert_eq!(serialized, vec![10, 3, 1, 2, 3, 18, 3, 4, 5, 6]);
}
