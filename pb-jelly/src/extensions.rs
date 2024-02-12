use std::io;
use std::marker::PhantomData;

use crate::{
    ensure_wire_format,
    varint,
    wire_format,
    Message,
    PbBufferReader,
    Unrecognized,
};

/// Indicates that a message type has extension ranges defined.
/// See <https://protobuf.dev/programming-guides/proto2/#extensions> for details.
pub trait Extensible: Message {
    /// Attempts to read the given extension field from `self`.
    ///
    /// Returns `Err(_)` if the field was found but could not be deserialized as the declared field type.
    fn get_extension<E: Extension<Extendee = Self>>(&self, extension: E) -> io::Result<E::Value> {
        extension.get(self)
    }

    /// Returns a reference to the `_extensions` field.
    /// This is intended to be implemented by generated code and isn't very useful for users of pb-jelly,
    /// so it's doc(hidden).
    #[doc(hidden)]
    fn _extensions(&self) -> &Unrecognized;
}

/// Abstracts over [SingularExtension]/[RepeatedExtension].
pub trait Extension {
    type Extendee: Extensible;
    type Value;
    fn get(&self, m: &Self::Extendee) -> io::Result<Self::Value>;
}

/// An extension field. See <https://protobuf.dev/programming-guides/proto2/#extensions> for details.
pub struct SingularExtension<T, U> {
    pub field_number: u32,
    pub wire_format: wire_format::Type,
    pub name: &'static str,
    _phantom: PhantomData<fn(&T) -> U>,
}

impl<T, U> SingularExtension<T, U> {
    pub const fn new(field_number: u32, wire_format: wire_format::Type, name: &'static str) -> Self {
        Self {
            field_number,
            wire_format,
            name,
            _phantom: PhantomData,
        }
    }
}

impl<T, U> Copy for SingularExtension<T, U> {}
impl<T, U> Clone for SingularExtension<T, U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Extensible, U: Message> Extension for SingularExtension<T, U> {
    type Extendee = T;
    type Value = Option<U>;

    fn get(&self, m: &Self::Extendee) -> io::Result<Option<U>> {
        Ok(match dbg!(m._extensions().get_singular_field(self.field_number)) {
            Some((field, wire_format)) => {
                let mut buf = io::Cursor::new(field);
                ensure_wire_format(wire_format, self.wire_format, self.name, self.field_number)?;
                if wire_format == wire_format::Type::LengthDelimited {
                    // we don't actually need this since we already have the length of `field`
                    varint::read(&mut buf)?;
                }
                let mut msg = U::default();
                msg.deserialize(&mut buf)?;
                Some(msg)
            },
            None => None,
        })
    }
}

/// A `repeated` extension field. See <https://protobuf.dev/programming-guides/proto2/#extensions> for details.
pub struct RepeatedExtension<T, U> {
    pub field_number: u32,
    pub wire_format: wire_format::Type,
    pub name: &'static str,
    _phantom: PhantomData<fn(&T) -> U>,
}

impl<T, U> RepeatedExtension<T, U> {
    pub const fn new(field_number: u32, wire_format: wire_format::Type, name: &'static str) -> Self {
        Self {
            field_number,
            wire_format,
            name,
            _phantom: PhantomData,
        }
    }
}

impl<T, U> Copy for RepeatedExtension<T, U> {}
impl<T, U> Clone for RepeatedExtension<T, U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Extensible, U: Message> Extension for RepeatedExtension<T, U> {
    type Extendee = T;
    type Value = Vec<U>;

    fn get(&self, m: &Self::Extendee) -> io::Result<Vec<U>> {
        let mut result = vec![];
        let mut buf = io::Cursor::new(m._extensions().get_fields(self.field_number));
        while let Some((_field_number, wire_format)) = wire_format::read(&mut buf)? {
            ensure_wire_format(wire_format, self.wire_format, self.name, self.field_number)?;
            let mut msg = U::default();
            if wire_format == wire_format::Type::LengthDelimited {
                let length = varint::read(&mut buf)?.expect("corrupted Unrecognized");
                msg.deserialize(&mut buf.split(length as usize))?;
            } else {
                // we rely on the fact that the appropriate `Message` impls for i32/Fixed32/etc. only read the prefix of
                // `buf`. this is a little dirty
                msg.deserialize(&mut buf)?;
            }
            result.push(msg);
        }
        Ok(result)
    }
}
