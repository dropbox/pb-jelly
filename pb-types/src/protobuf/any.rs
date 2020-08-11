// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// `Any` contains an arbitrary serialized protocol buffer message along with a
/// URL that describes the type of the serialized message.

/// Protobuf library provides support to pack/unpack Any values in the form
/// of utility functions or additional generated methods of the Any type.

/// Example 1: Pack and unpack a message in C++.

///     Foo foo = ...;
///     Any any;
///     any.PackFrom(foo);
///     ...
///     if (any.UnpackTo(&foo)) {
///       ...
///     }

/// Example 2: Pack and unpack a message in Java.

///     Foo foo = ...;
///     Any any = Any.pack(foo);
///     ...
///     if (any.is(Foo.class)) {
///       foo = any.unpack(Foo.class);
///     }

///  Example 3: Pack and unpack a message in Python.

///     foo = Foo(...)
///     any = Any()
///     any.Pack(foo)
///     ...
///     if any.Is(Foo.DESCRIPTOR):
///       any.Unpack(foo)
///       ...

///  Example 4: Pack and unpack a message in Go

///      foo := &pb.Foo{...}
///      any, err := ptypes.MarshalAny(foo)
///      ...
///      foo := &pb.Foo{}
///      if err := ptypes.UnmarshalAny(any, foo); err != nil {
///        ...
///      }

/// The pack methods provided by protobuf library will by default use
/// 'type.googleapis.com/full.type.name' as the type URL and the unpack
/// methods only use the fully qualified type name after the last '/'
/// in the type URL, for example "foo.bar.com/x/y.z" will yield type
/// name "y.z".


/// JSON
/// ====
/// The JSON representation of an `Any` value uses the regular
/// representation of the deserialized, embedded message, with an
/// additional field `@type` which contains the type URL. Example:

///     package google.profile;
///     message Person {
///       string first_name = 1;
///       string last_name = 2;
///     }

///     {
///       "@type": "type.googleapis.com/google.profile.Person",
///       "firstName": <string>,
///       "lastName": <string>
///     }

/// If the embedded message type is well-known and has a custom JSON
/// representation, that representation will be embedded adding a field
/// `value` which holds the custom JSON in addition to the `@type`
/// field. Example (for message [google.protobuf.Duration][]):

///     {
///       "@type": "type.googleapis.com/google.protobuf.Duration",
///       "value": "1.212s"
///     }
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Any {
  /// A URL/resource name that uniquely identifies the type of the serialized
  /// protocol buffer message. This string must contain at least
  /// one "/" character. The last segment of the URL's path must represent
  /// the fully qualified name of the type (as in
  /// `path/google.protobuf.Duration`). The name should be in a canonical form
  /// (e.g., leading "." is not accepted).

  /// In practice, teams usually precompile into the binary all types that they
  /// expect it to use in the context of Any. However, for URLs which use the
  /// scheme `http`, `https`, or no scheme, one can optionally set up a type
  /// server that maps type URLs to message definitions as follows:

  /// * If no scheme is provided, `https` is assumed.
  /// * An HTTP GET on the URL must yield a [google.protobuf.Type][]
  ///   value in binary format, or produce an error.
  /// * Applications are allowed to cache lookup results based on the
  ///   URL, or have them precompiled into a binary to avoid any
  ///   lookup. Therefore, binary compatibility needs to be preserved
  ///   on changes to types. (Use versioned type names to manage
  ///   breaking changes.)

  /// Note: this functionality is not currently available in the official
  /// protobuf release, and it is not used for type URLs beginning with
  /// type.googleapis.com.

  /// Schemes other than `http`, `https` (or the empty scheme) might be
  /// used with implementation specific semantics.
  pub type_url: ::std::string::String,
  /// Must be a valid serialized protocol buffer of the above specified type.
  pub value: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for Any {
  fn default() -> Self {
    Any {
      type_url: ::std::default::Default::default(),
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Any_default: Any = Any::default();
}
impl ::pb::Message for Any {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut type_url_size = 0;
    if self.type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.type_url;
      let l = ::pb::Message::compute_size(val);
      type_url_size += ::pb::wire_format::serialized_length(1);
      type_url_size += ::pb::varint::serialized_length(l as u64);
      type_url_size += l;
    }
    size += type_url_size;
    let mut value_size = 0;
    if self.value != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(2);
      value_size += ::pb::varint::serialized_length(l as u64);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.type_url;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.value != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.type_url;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.value != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Any", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.type_url = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Any", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::vec::Vec<u8> = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.value = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Any {
  const NAME: &'static str = "Any";
  const FULL_NAME: &'static str = "google.protobuf.Any";
}

