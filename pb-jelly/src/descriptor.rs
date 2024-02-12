use crate::wire_format;

/// Trait implemented by all the messages defined in proto files.
/// Provides rudimentary support for message descriptor, mostly as a way to get
/// the type name for a given message rather than trying to implement the full
/// reflection API. For more info, see:
/// <https://developers.google.com/protocol-buffers/docs/techniques>
/// <https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto>
pub struct MessageDescriptor {
    /// The name of the message type, not including its scope.
    pub name: &'static str,
    /// The fully-qualified name of the message type, scope delimited by periods.
    /// For example, message type "Foo" which is declared in package "bar" has full name "bar.Foo".
    /// If a type "Baz" is nested within Foo, Baz's full_name is "bar.Foo_Baz". To get only the
    /// part that comes after the last '.', use `message_name`.
    pub full_name: &'static str,
    /// Information about the fields in this message.
    pub fields: &'static [FieldDescriptor],
    /// Information about the oneofs in this message.
    pub oneofs: &'static [OneofDescriptor],
}

/// Describes a field within a message.
/// Provides rudimentary support for the proto field reflection API, with the intention of being
/// able to serialize or introspect fields individually without having knowledge of the structure
/// of the specific message itself. For more info, see:
/// <https://developers.google.com/protocol-buffers/docs/techniques>
/// <https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto>
pub struct FieldDescriptor {
    /// The name of this field, not including its scope.
    pub name: &'static str,
    /// The fully-qualified name of this field, scope delimited by periods.
    pub full_name: &'static str,
    /// The index of this field, which has values from 0 inclusive to n exclusive, where n is the
    /// number of fields in this message.
    pub index: u32,
    /// The number assigned to the field in the proto declaration.
    pub number: u32,
    pub typ: wire_format::Type,
    pub label: Label,
    /// If this field is part of a oneof group, this is an index into the `oneofs` of the parent
    /// message. Otherwise None.
    pub oneof_index: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Label {
    Optional = 1,
    Required = 2,
    Repeated = 3,
}

/// Describes a oneof.
pub struct OneofDescriptor {
    pub name: &'static str,
}

impl MessageDescriptor {
    /// Gets a field by name.
    pub fn get_field(&self, name: &str) -> Option<&FieldDescriptor> {
        self.fields.iter().find(|f| f.name == name)
    }
}
