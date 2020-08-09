// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// Api is a light-weight descriptor for an API Interface.

/// Interfaces are also described as "protocol buffer services" in some contexts,
/// such as by the "service" keyword in a .proto file, but they are different
/// from API Services, which represent a concrete implementation of an interface
/// as opposed to simply a description of methods and bindings. They are also
/// sometimes simply referred to as "APIs" in other contexts, such as the name of
/// this message itself. See https://cloud.google.com/apis/design/glossary for
/// detailed terminology.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Api {
  /// The fully qualified name of this interface, including package name
  /// followed by the interface's simple name.
  pub name: ::std::string::String,
  /// The methods of this interface, in unspecified order.
  pub methods: ::std::vec::Vec<Method>,
  /// Any metadata attached to the interface.
  pub options: ::std::vec::Vec<super::super::protobuf::type_::Option>,
  /// A version string for this interface. If specified, must have the form
  /// `major-version.minor-version`, as in `1.10`. If the minor version is
  /// omitted, it defaults to zero. If the entire version field is empty, the
  /// major version is derived from the package name, as outlined below. If the
  /// field is not empty, the version in the package name will be verified to be
  /// consistent with what is provided here.

  /// The versioning schema uses [semantic
  /// versioning](http://semver.org) where the major version number
  /// indicates a breaking change and the minor version an additive,
  /// non-breaking change. Both version numbers are signals to users
  /// what to expect from different versions, and should be carefully
  /// chosen based on the product plan.

  /// The major version is also reflected in the package name of the
  /// interface, which must end in `v<major-version>`, as in
  /// `google.feature.v1`. For major versions 0 and 1, the suffix can
  /// be omitted. Zero major versions must only be used for
  /// experimental, non-GA interfaces.
  pub version: ::std::string::String,
  /// Source context for the protocol buffer service represented by this
  /// message.
  pub source_context: ::std::option::Option<super::super::protobuf::source_context::SourceContext>,
  /// Included interfaces. See [Mixin][].
  pub mixins: ::std::vec::Vec<Mixin>,
  /// The source syntax of the service.
  pub syntax: super::super::protobuf::type_::Syntax,
}
impl ::std::default::Default for Api {
  fn default() -> Self {
    Api {
      name: ::std::default::Default::default(),
      methods: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      version: ::std::default::Default::default(),
      source_context: ::std::default::Default::default(),
      mixins: ::std::default::Default::default(),
      syntax: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Api_default: Api = Api::default();
}
impl ::pb::Message for Api {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut methods_size = 0;
    for val in &self.methods {
      let l = ::pb::Message::compute_size(val);
      methods_size += ::pb::wire_format::serialized_length(2);
      methods_size += ::pb::varint::serialized_length(l as u64);
      methods_size += l;
    }
    size += methods_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(3);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut version_size = 0;
    if self.version != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.version;
      let l = ::pb::Message::compute_size(val);
      version_size += ::pb::wire_format::serialized_length(4);
      version_size += ::pb::varint::serialized_length(l as u64);
      version_size += l;
    }
    size += version_size;
    let mut source_context_size = 0;
    for val in &self.source_context {
      let l = ::pb::Message::compute_size(val);
      source_context_size += ::pb::wire_format::serialized_length(5);
      source_context_size += ::pb::varint::serialized_length(l as u64);
      source_context_size += l;
    }
    size += source_context_size;
    let mut mixins_size = 0;
    for val in &self.mixins {
      let l = ::pb::Message::compute_size(val);
      mixins_size += ::pb::wire_format::serialized_length(6);
      mixins_size += ::pb::varint::serialized_length(l as u64);
      mixins_size += l;
    }
    size += mixins_size;
    let mut syntax_size = 0;
    if self.syntax != <super::super::protobuf::type_::Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      let l = ::pb::Message::compute_size(val);
      syntax_size += ::pb::wire_format::serialized_length(7);
      syntax_size += l;
    }
    size += syntax_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.methods {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.version != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.version;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.source_context {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.mixins {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.syntax != <super::super::protobuf::type_::Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.methods {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.version != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.version;
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.source_context {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.mixins {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.syntax != <super::super::protobuf::type_::Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      ::pb::wire_format::write(7, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Api", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Api", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Method = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.methods.push(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Api", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: super::super::protobuf::type_::Option = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options.push(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Api", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.version = val;
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Api", 5)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: super::super::protobuf::source_context::SourceContext = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.source_context = Some(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Api", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Mixin = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.mixins.push(val);
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Api", 7)?;
          let mut val: super::super::protobuf::type_::Syntax = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.syntax = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Api {
  const NAME: &'static str = "Api";
  const FULL_NAME: &'static str = "google.protobuf.Api";
}

/// Method represents a method of an API interface.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Method {
  /// The simple name of this method.
  pub name: ::std::string::String,
  /// A URL of the input message type.
  pub request_type_url: ::std::string::String,
  /// If true, the request is streamed.
  pub request_streaming: bool,
  /// The URL of the output message type.
  pub response_type_url: ::std::string::String,
  /// If true, the response is streamed.
  pub response_streaming: bool,
  /// Any metadata attached to the method.
  pub options: ::std::vec::Vec<super::super::protobuf::type_::Option>,
  /// The source syntax of this method.
  pub syntax: super::super::protobuf::type_::Syntax,
}
impl ::std::default::Default for Method {
  fn default() -> Self {
    Method {
      name: ::std::default::Default::default(),
      request_type_url: ::std::default::Default::default(),
      request_streaming: ::std::default::Default::default(),
      response_type_url: ::std::default::Default::default(),
      response_streaming: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      syntax: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Method_default: Method = Method::default();
}
impl ::pb::Message for Method {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut request_type_url_size = 0;
    if self.request_type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.request_type_url;
      let l = ::pb::Message::compute_size(val);
      request_type_url_size += ::pb::wire_format::serialized_length(2);
      request_type_url_size += ::pb::varint::serialized_length(l as u64);
      request_type_url_size += l;
    }
    size += request_type_url_size;
    let mut request_streaming_size = 0;
    if self.request_streaming != <bool as ::std::default::Default>::default() {
      let val = &self.request_streaming;
      let l = ::pb::Message::compute_size(val);
      request_streaming_size += ::pb::wire_format::serialized_length(3);
      request_streaming_size += l;
    }
    size += request_streaming_size;
    let mut response_type_url_size = 0;
    if self.response_type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.response_type_url;
      let l = ::pb::Message::compute_size(val);
      response_type_url_size += ::pb::wire_format::serialized_length(4);
      response_type_url_size += ::pb::varint::serialized_length(l as u64);
      response_type_url_size += l;
    }
    size += response_type_url_size;
    let mut response_streaming_size = 0;
    if self.response_streaming != <bool as ::std::default::Default>::default() {
      let val = &self.response_streaming;
      let l = ::pb::Message::compute_size(val);
      response_streaming_size += ::pb::wire_format::serialized_length(5);
      response_streaming_size += l;
    }
    size += response_streaming_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(6);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut syntax_size = 0;
    if self.syntax != <super::super::protobuf::type_::Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      let l = ::pb::Message::compute_size(val);
      syntax_size += ::pb::wire_format::serialized_length(7);
      syntax_size += l;
    }
    size += syntax_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.request_type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.request_type_url;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.request_streaming != <bool as ::std::default::Default>::default() {
      let val = &self.request_streaming;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.response_type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.response_type_url;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.response_streaming != <bool as ::std::default::Default>::default() {
      let val = &self.response_streaming;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.syntax != <super::super::protobuf::type_::Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.request_type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.request_type_url;
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.request_streaming != <bool as ::std::default::Default>::default() {
      let val = &self.request_streaming;
      ::pb::wire_format::write(3, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.response_type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.response_type_url;
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.response_streaming != <bool as ::std::default::Default>::default() {
      let val = &self.response_streaming;
      ::pb::wire_format::write(5, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.syntax != <super::super::protobuf::type_::Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      ::pb::wire_format::write(7, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Method", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Method", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.request_type_url = val;
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Method", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.request_streaming = val;
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Method", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.response_type_url = val;
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Method", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.response_streaming = val;
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Method", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: super::super::protobuf::type_::Option = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options.push(val);
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Method", 7)?;
          let mut val: super::super::protobuf::type_::Syntax = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.syntax = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Method {
  const NAME: &'static str = "Method";
  const FULL_NAME: &'static str = "google.protobuf.Method";
}

/// Declares an API Interface to be included in this interface. The including
/// interface must redeclare all the methods from the included interface, but
/// documentation and options are inherited as follows:

/// - If after comment and whitespace stripping, the documentation
///   string of the redeclared method is empty, it will be inherited
///   from the original method.

/// - Each annotation belonging to the service config (http,
///   visibility) which is not set in the redeclared method will be
///   inherited.

/// - If an http annotation is inherited, the path pattern will be
///   modified as follows. Any version prefix will be replaced by the
///   version of the including interface plus the [root][] path if
///   specified.

/// Example of a simple mixin:

///     package google.acl.v1;
///     service AccessControl {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v1/{resource=**}:getAcl";
///       }
///     }

///     package google.storage.v2;
///     service Storage {
///       rpc GetAcl(GetAclRequest) returns (Acl);

///       // Get a data record.
///       rpc GetData(GetDataRequest) returns (Data) {
///         option (google.api.http).get = "/v2/{resource=**}";
///       }
///     }

/// Example of a mixin configuration:

///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl

/// The mixin construct implies that all methods in `AccessControl` are
/// also declared with same name and request/response types in
/// `Storage`. A documentation generator or annotation processor will
/// see the effective `Storage.GetAcl` method after inherting
/// documentation and annotations as follows:

///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/{resource=**}:getAcl";
///       }
///       ...
///     }

/// Note how the version in the path pattern changed from `v1` to `v2`.

/// If the `root` field in the mixin is specified, it should be a
/// relative path under which inherited HTTP paths are placed. Example:

///     apis:
///     - name: google.storage.v2.Storage
///       mixins:
///       - name: google.acl.v1.AccessControl
///         root: acls

/// This implies the following inherited HTTP annotation:

///     service Storage {
///       // Get the underlying ACL object.
///       rpc GetAcl(GetAclRequest) returns (Acl) {
///         option (google.api.http).get = "/v2/acls/{resource=**}:getAcl";
///       }
///       ...
///     }
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Mixin {
  /// The fully qualified name of the interface which is included.
  pub name: ::std::string::String,
  /// If non-empty specifies a path under which inherited HTTP paths
  /// are rooted.
  pub root: ::std::string::String,
}
impl ::std::default::Default for Mixin {
  fn default() -> Self {
    Mixin {
      name: ::std::default::Default::default(),
      root: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Mixin_default: Mixin = Mixin::default();
}
impl ::pb::Message for Mixin {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut root_size = 0;
    if self.root != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.root;
      let l = ::pb::Message::compute_size(val);
      root_size += ::pb::wire_format::serialized_length(2);
      root_size += ::pb::varint::serialized_length(l as u64);
      root_size += l;
    }
    size += root_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.root != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.root;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.root != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.root;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Mixin", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Mixin", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.root = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Mixin {
  const NAME: &'static str = "Mixin";
  const FULL_NAME: &'static str = "google.protobuf.Mixin";
}

