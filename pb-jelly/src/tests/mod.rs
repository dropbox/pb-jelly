use std::any::Any;
use std::io::{
    Error,
    ErrorKind,
    Result,
    Write,
};
use std::rc::Rc;

use bytes::{
    Buf,
    BufMut,
};

use crate::varint;
use crate::wire_format;

use super::{
    ensure_split,
    ensure_wire_format,
    skip,
    type_is,
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

impl AsRef<[u8]> for VecReader {
    fn as_ref(&self) -> &[u8] {
        &self.contents[self.position..self.end]
    }
}

impl PbBuffer for VecReader {
    fn len(&self) -> usize {
        self.remaining()
    }
    fn copy_to_writer<W: Write + ?Sized>(&self, writer: &mut W) -> Result<()> {
        writer.write_all(self.as_ref())
    }
    fn copy_from_reader<B: Buf + ?Sized>(reader: &mut B) -> Result<Self> {
        let mut v = vec![];
        v.put(reader);
        Ok(v.into())
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

    fn chunk(&self) -> &[u8] {
        self.as_ref()
    }

    fn advance(&mut self, cnt: usize) {
        debug_assert!(self.position + cnt <= self.end);
        self.position += cnt;
    }
}

impl PbBufferReader for VecReader {
    fn read_buffer<B: PbBuffer>(&mut self) -> Result<B> {
        if let Some(cast) = type_is::<VecReader, B>() {
            Ok(cast(self.clone()))
        } else {
            B::copy_from_reader(self)
        }
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
            out.extend_from_slice(content.as_ref());
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
fn test_deserialize_message_from_vec_reader() {
    let mut message = TestMessage::default();
    message.normal_data = vec![1, 2, 3];
    message.payload = Lazy::new(vec![4, 5, 6].into());

    let serialized = message.serialize_to_vec();
    assert_eq!(serialized.len(), message.compute_size());
    assert_eq!(serialized, [10, 3, 1, 2, 3, 18, 3, 4, 5, 6]);

    // Deserializing the message from a slice should succeed, even though it incurs a copy.
    assert_eq!(
        TestMessage::deserialize_from_slice(&serialized).expect("deserialization failed"),
        message
    );

    // Deserializing from a `VecReader` should succeed and capture a reference to the reader.
    let reader = VecReader::from(serialized);
    let mut deserialized = TestMessage::default();
    deserialized.deserialize(&mut reader.clone()).unwrap();

    assert_eq!(deserialized.normal_data, message.normal_data);
    let deserialized_payload = deserialized.payload.into_buffer();
    assert!(
        Rc::ptr_eq(&deserialized_payload.contents, &reader.contents),
        "Should reference the original buffer"
    );
    assert_eq!(deserialized_payload.as_ref(), message.payload.into_buffer().as_ref());
}

#[test]
fn test_serialize_message_into_vec_writer() {
    let payload: VecReader = vec![4, 5, 6].into();
    let mut message = TestMessage::default();
    message.normal_data = vec![1, 2, 3];
    message.payload = Lazy::new(payload.clone());

    // Serialize to a `VecWriter`.
    let mut out = VecWriter::default();
    message.serialize(&mut out).unwrap();

    // One of the segments of `out` should reference the same buffer as `payload`.
    assert_eq!(
        out.contents
            .iter()
            .filter(|buf| Rc::ptr_eq(&buf.contents, &payload.contents))
            .count(),
        1
    );

    let serialized = out.serialized();
    assert_eq!(
        TestMessage::deserialize_from_slice(&serialized).expect("deserialization failed"),
        message
    );
    assert_eq!(serialized, [10, 3, 1, 2, 3, 18, 3, 4, 5, 6]);
}
