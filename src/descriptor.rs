/// Trait implemented by all the messages defined in proto files.
/// Provides rudimentary support for message descriptor, mostly as a way to get
/// the type name for a given message rather than trying to implement the full
/// reflection API. For more info, see:
/// <https://developers.google.com/protocol-buffers/docs/techniques>
/// <https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/descriptor.proto>
pub trait MessageDescriptor {
    /// The name of the message type, not including its scope.
    const NAME: &'static str;
    /// The fully-qualified name of the message type, scope delimited by periods.
    /// For example, message type "Foo" which is declared in package "bar" has full name "bar.Foo".
    /// If a type "Baz" is nested within Foo, Baz's full_name is "bar.Foo_Baz". To get only the
    /// part that comes after the last '.', use `message_name`.
    const FULL_NAME: &'static str;
}
