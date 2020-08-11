// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// The syntax in which a protocol buffer element is defined.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Syntax(i32);
impl Syntax {
  /// Syntax `proto2`.
  pub const SYNTAX_PROTO2: Syntax = Syntax(0);
  /// Syntax `proto3`.
  pub const SYNTAX_PROTO3: Syntax = Syntax(1);
  pub const KNOWN_VARIANTS: [Syntax; 2] = [Syntax::SYNTAX_PROTO2, Syntax::SYNTAX_PROTO3];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<Syntax_Closed> {
    match self {
      Syntax::SYNTAX_PROTO2 => Some(Syntax_Closed::SYNTAX_PROTO2),
      Syntax::SYNTAX_PROTO3 => Some(Syntax_Closed::SYNTAX_PROTO3),
      _ => None,
    }
  }
}
impl ::std::default::Default for Syntax {
  fn default() -> Self {
    Syntax::SYNTAX_PROTO2
  }
}
impl From<Syntax> for i32 {
  fn from(v: Syntax) -> i32 {
    v.0
  }
}
impl From<i32> for Syntax {
  fn from(v: i32) -> Syntax {
    Syntax(v)
  }
}
impl From<Syntax_Closed> for Syntax {
  fn from(v: Syntax_Closed) -> Syntax {
    Syntax(v as i32)
  }
}
impl ::pb::ProtoEnum for Syntax {
}
impl ::pb::OpenProtoEnum for Syntax {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      Syntax::SYNTAX_PROTO2 => Some("SYNTAX_PROTO2"),
      Syntax::SYNTAX_PROTO3 => Some("SYNTAX_PROTO3"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      Syntax::SYNTAX_PROTO2 => true,
      Syntax::SYNTAX_PROTO3 => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for Syntax {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// The syntax in which a protocol buffer element is defined.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum Syntax_Closed {
  /// Syntax `proto2`.
  SYNTAX_PROTO2 = 0,
  /// Syntax `proto3`.
  SYNTAX_PROTO3 = 1,
}
impl Syntax_Closed {
  pub const KNOWN_VARIANTS: [Syntax_Closed; 2] = [Syntax_Closed::SYNTAX_PROTO2, Syntax_Closed::SYNTAX_PROTO3];
}
impl ::std::default::Default for Syntax_Closed {
  fn default() -> Self {
    Syntax_Closed::SYNTAX_PROTO2
  }
}
impl From<Syntax_Closed> for i32 {
  fn from(v: Syntax_Closed) -> i32 {
    match v {
      Syntax_Closed::SYNTAX_PROTO2 => 0,
      Syntax_Closed::SYNTAX_PROTO3 => 1,
    }
  }
}
impl ::std::convert::TryFrom<i32> for Syntax_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(Syntax_Closed::SYNTAX_PROTO2),
      1 => Ok(Syntax_Closed::SYNTAX_PROTO3),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for Syntax_Closed {
}
impl ::pb::ClosedProtoEnum for Syntax_Closed {
  fn name(self) -> &'static str {
    match self {
      Syntax_Closed::SYNTAX_PROTO2 => "SYNTAX_PROTO2",
      Syntax_Closed::SYNTAX_PROTO3 => "SYNTAX_PROTO3",
    }
  }
}

/// Basic field types.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Field_Kind(i32);
impl Field_Kind {
  /// Field type unknown.
  pub const TYPE_UNKNOWN: Field_Kind = Field_Kind(0);
  /// Field type double.
  pub const TYPE_DOUBLE: Field_Kind = Field_Kind(1);
  /// Field type float.
  pub const TYPE_FLOAT: Field_Kind = Field_Kind(2);
  /// Field type int64.
  pub const TYPE_INT64: Field_Kind = Field_Kind(3);
  /// Field type uint64.
  pub const TYPE_UINT64: Field_Kind = Field_Kind(4);
  /// Field type int32.
  pub const TYPE_INT32: Field_Kind = Field_Kind(5);
  /// Field type fixed64.
  pub const TYPE_FIXED64: Field_Kind = Field_Kind(6);
  /// Field type fixed32.
  pub const TYPE_FIXED32: Field_Kind = Field_Kind(7);
  /// Field type bool.
  pub const TYPE_BOOL: Field_Kind = Field_Kind(8);
  /// Field type string.
  pub const TYPE_STRING: Field_Kind = Field_Kind(9);
  /// Field type group. Proto2 syntax only, and deprecated.
  pub const TYPE_GROUP: Field_Kind = Field_Kind(10);
  /// Field type message.
  pub const TYPE_MESSAGE: Field_Kind = Field_Kind(11);
  /// Field type bytes.
  pub const TYPE_BYTES: Field_Kind = Field_Kind(12);
  /// Field type uint32.
  pub const TYPE_UINT32: Field_Kind = Field_Kind(13);
  /// Field type enum.
  pub const TYPE_ENUM: Field_Kind = Field_Kind(14);
  /// Field type sfixed32.
  pub const TYPE_SFIXED32: Field_Kind = Field_Kind(15);
  /// Field type sfixed64.
  pub const TYPE_SFIXED64: Field_Kind = Field_Kind(16);
  /// Field type sint32.
  pub const TYPE_SINT32: Field_Kind = Field_Kind(17);
  /// Field type sint64.
  pub const TYPE_SINT64: Field_Kind = Field_Kind(18);
  pub const KNOWN_VARIANTS: [Field_Kind; 19] = [Field_Kind::TYPE_UNKNOWN, Field_Kind::TYPE_DOUBLE, Field_Kind::TYPE_FLOAT, Field_Kind::TYPE_INT64, Field_Kind::TYPE_UINT64, Field_Kind::TYPE_INT32, Field_Kind::TYPE_FIXED64, Field_Kind::TYPE_FIXED32, Field_Kind::TYPE_BOOL, Field_Kind::TYPE_STRING, Field_Kind::TYPE_GROUP, Field_Kind::TYPE_MESSAGE, Field_Kind::TYPE_BYTES, Field_Kind::TYPE_UINT32, Field_Kind::TYPE_ENUM, Field_Kind::TYPE_SFIXED32, Field_Kind::TYPE_SFIXED64, Field_Kind::TYPE_SINT32, Field_Kind::TYPE_SINT64];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<Field_Kind_Closed> {
    match self {
      Field_Kind::TYPE_UNKNOWN => Some(Field_Kind_Closed::TYPE_UNKNOWN),
      Field_Kind::TYPE_DOUBLE => Some(Field_Kind_Closed::TYPE_DOUBLE),
      Field_Kind::TYPE_FLOAT => Some(Field_Kind_Closed::TYPE_FLOAT),
      Field_Kind::TYPE_INT64 => Some(Field_Kind_Closed::TYPE_INT64),
      Field_Kind::TYPE_UINT64 => Some(Field_Kind_Closed::TYPE_UINT64),
      Field_Kind::TYPE_INT32 => Some(Field_Kind_Closed::TYPE_INT32),
      Field_Kind::TYPE_FIXED64 => Some(Field_Kind_Closed::TYPE_FIXED64),
      Field_Kind::TYPE_FIXED32 => Some(Field_Kind_Closed::TYPE_FIXED32),
      Field_Kind::TYPE_BOOL => Some(Field_Kind_Closed::TYPE_BOOL),
      Field_Kind::TYPE_STRING => Some(Field_Kind_Closed::TYPE_STRING),
      Field_Kind::TYPE_GROUP => Some(Field_Kind_Closed::TYPE_GROUP),
      Field_Kind::TYPE_MESSAGE => Some(Field_Kind_Closed::TYPE_MESSAGE),
      Field_Kind::TYPE_BYTES => Some(Field_Kind_Closed::TYPE_BYTES),
      Field_Kind::TYPE_UINT32 => Some(Field_Kind_Closed::TYPE_UINT32),
      Field_Kind::TYPE_ENUM => Some(Field_Kind_Closed::TYPE_ENUM),
      Field_Kind::TYPE_SFIXED32 => Some(Field_Kind_Closed::TYPE_SFIXED32),
      Field_Kind::TYPE_SFIXED64 => Some(Field_Kind_Closed::TYPE_SFIXED64),
      Field_Kind::TYPE_SINT32 => Some(Field_Kind_Closed::TYPE_SINT32),
      Field_Kind::TYPE_SINT64 => Some(Field_Kind_Closed::TYPE_SINT64),
      _ => None,
    }
  }
}
impl ::std::default::Default for Field_Kind {
  fn default() -> Self {
    Field_Kind::TYPE_UNKNOWN
  }
}
impl From<Field_Kind> for i32 {
  fn from(v: Field_Kind) -> i32 {
    v.0
  }
}
impl From<i32> for Field_Kind {
  fn from(v: i32) -> Field_Kind {
    Field_Kind(v)
  }
}
impl From<Field_Kind_Closed> for Field_Kind {
  fn from(v: Field_Kind_Closed) -> Field_Kind {
    Field_Kind(v as i32)
  }
}
impl ::pb::ProtoEnum for Field_Kind {
}
impl ::pb::OpenProtoEnum for Field_Kind {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      Field_Kind::TYPE_UNKNOWN => Some("TYPE_UNKNOWN"),
      Field_Kind::TYPE_DOUBLE => Some("TYPE_DOUBLE"),
      Field_Kind::TYPE_FLOAT => Some("TYPE_FLOAT"),
      Field_Kind::TYPE_INT64 => Some("TYPE_INT64"),
      Field_Kind::TYPE_UINT64 => Some("TYPE_UINT64"),
      Field_Kind::TYPE_INT32 => Some("TYPE_INT32"),
      Field_Kind::TYPE_FIXED64 => Some("TYPE_FIXED64"),
      Field_Kind::TYPE_FIXED32 => Some("TYPE_FIXED32"),
      Field_Kind::TYPE_BOOL => Some("TYPE_BOOL"),
      Field_Kind::TYPE_STRING => Some("TYPE_STRING"),
      Field_Kind::TYPE_GROUP => Some("TYPE_GROUP"),
      Field_Kind::TYPE_MESSAGE => Some("TYPE_MESSAGE"),
      Field_Kind::TYPE_BYTES => Some("TYPE_BYTES"),
      Field_Kind::TYPE_UINT32 => Some("TYPE_UINT32"),
      Field_Kind::TYPE_ENUM => Some("TYPE_ENUM"),
      Field_Kind::TYPE_SFIXED32 => Some("TYPE_SFIXED32"),
      Field_Kind::TYPE_SFIXED64 => Some("TYPE_SFIXED64"),
      Field_Kind::TYPE_SINT32 => Some("TYPE_SINT32"),
      Field_Kind::TYPE_SINT64 => Some("TYPE_SINT64"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      Field_Kind::TYPE_UNKNOWN => true,
      Field_Kind::TYPE_DOUBLE => true,
      Field_Kind::TYPE_FLOAT => true,
      Field_Kind::TYPE_INT64 => true,
      Field_Kind::TYPE_UINT64 => true,
      Field_Kind::TYPE_INT32 => true,
      Field_Kind::TYPE_FIXED64 => true,
      Field_Kind::TYPE_FIXED32 => true,
      Field_Kind::TYPE_BOOL => true,
      Field_Kind::TYPE_STRING => true,
      Field_Kind::TYPE_GROUP => true,
      Field_Kind::TYPE_MESSAGE => true,
      Field_Kind::TYPE_BYTES => true,
      Field_Kind::TYPE_UINT32 => true,
      Field_Kind::TYPE_ENUM => true,
      Field_Kind::TYPE_SFIXED32 => true,
      Field_Kind::TYPE_SFIXED64 => true,
      Field_Kind::TYPE_SINT32 => true,
      Field_Kind::TYPE_SINT64 => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for Field_Kind {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// Basic field types.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum Field_Kind_Closed {
  /// Field type unknown.
  TYPE_UNKNOWN = 0,
  /// Field type double.
  TYPE_DOUBLE = 1,
  /// Field type float.
  TYPE_FLOAT = 2,
  /// Field type int64.
  TYPE_INT64 = 3,
  /// Field type uint64.
  TYPE_UINT64 = 4,
  /// Field type int32.
  TYPE_INT32 = 5,
  /// Field type fixed64.
  TYPE_FIXED64 = 6,
  /// Field type fixed32.
  TYPE_FIXED32 = 7,
  /// Field type bool.
  TYPE_BOOL = 8,
  /// Field type string.
  TYPE_STRING = 9,
  /// Field type group. Proto2 syntax only, and deprecated.
  TYPE_GROUP = 10,
  /// Field type message.
  TYPE_MESSAGE = 11,
  /// Field type bytes.
  TYPE_BYTES = 12,
  /// Field type uint32.
  TYPE_UINT32 = 13,
  /// Field type enum.
  TYPE_ENUM = 14,
  /// Field type sfixed32.
  TYPE_SFIXED32 = 15,
  /// Field type sfixed64.
  TYPE_SFIXED64 = 16,
  /// Field type sint32.
  TYPE_SINT32 = 17,
  /// Field type sint64.
  TYPE_SINT64 = 18,
}
impl Field_Kind_Closed {
  pub const KNOWN_VARIANTS: [Field_Kind_Closed; 19] = [Field_Kind_Closed::TYPE_UNKNOWN, Field_Kind_Closed::TYPE_DOUBLE, Field_Kind_Closed::TYPE_FLOAT, Field_Kind_Closed::TYPE_INT64, Field_Kind_Closed::TYPE_UINT64, Field_Kind_Closed::TYPE_INT32, Field_Kind_Closed::TYPE_FIXED64, Field_Kind_Closed::TYPE_FIXED32, Field_Kind_Closed::TYPE_BOOL, Field_Kind_Closed::TYPE_STRING, Field_Kind_Closed::TYPE_GROUP, Field_Kind_Closed::TYPE_MESSAGE, Field_Kind_Closed::TYPE_BYTES, Field_Kind_Closed::TYPE_UINT32, Field_Kind_Closed::TYPE_ENUM, Field_Kind_Closed::TYPE_SFIXED32, Field_Kind_Closed::TYPE_SFIXED64, Field_Kind_Closed::TYPE_SINT32, Field_Kind_Closed::TYPE_SINT64];
}
impl ::std::default::Default for Field_Kind_Closed {
  fn default() -> Self {
    Field_Kind_Closed::TYPE_UNKNOWN
  }
}
impl From<Field_Kind_Closed> for i32 {
  fn from(v: Field_Kind_Closed) -> i32 {
    match v {
      Field_Kind_Closed::TYPE_UNKNOWN => 0,
      Field_Kind_Closed::TYPE_DOUBLE => 1,
      Field_Kind_Closed::TYPE_FLOAT => 2,
      Field_Kind_Closed::TYPE_INT64 => 3,
      Field_Kind_Closed::TYPE_UINT64 => 4,
      Field_Kind_Closed::TYPE_INT32 => 5,
      Field_Kind_Closed::TYPE_FIXED64 => 6,
      Field_Kind_Closed::TYPE_FIXED32 => 7,
      Field_Kind_Closed::TYPE_BOOL => 8,
      Field_Kind_Closed::TYPE_STRING => 9,
      Field_Kind_Closed::TYPE_GROUP => 10,
      Field_Kind_Closed::TYPE_MESSAGE => 11,
      Field_Kind_Closed::TYPE_BYTES => 12,
      Field_Kind_Closed::TYPE_UINT32 => 13,
      Field_Kind_Closed::TYPE_ENUM => 14,
      Field_Kind_Closed::TYPE_SFIXED32 => 15,
      Field_Kind_Closed::TYPE_SFIXED64 => 16,
      Field_Kind_Closed::TYPE_SINT32 => 17,
      Field_Kind_Closed::TYPE_SINT64 => 18,
    }
  }
}
impl ::std::convert::TryFrom<i32> for Field_Kind_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(Field_Kind_Closed::TYPE_UNKNOWN),
      1 => Ok(Field_Kind_Closed::TYPE_DOUBLE),
      2 => Ok(Field_Kind_Closed::TYPE_FLOAT),
      3 => Ok(Field_Kind_Closed::TYPE_INT64),
      4 => Ok(Field_Kind_Closed::TYPE_UINT64),
      5 => Ok(Field_Kind_Closed::TYPE_INT32),
      6 => Ok(Field_Kind_Closed::TYPE_FIXED64),
      7 => Ok(Field_Kind_Closed::TYPE_FIXED32),
      8 => Ok(Field_Kind_Closed::TYPE_BOOL),
      9 => Ok(Field_Kind_Closed::TYPE_STRING),
      10 => Ok(Field_Kind_Closed::TYPE_GROUP),
      11 => Ok(Field_Kind_Closed::TYPE_MESSAGE),
      12 => Ok(Field_Kind_Closed::TYPE_BYTES),
      13 => Ok(Field_Kind_Closed::TYPE_UINT32),
      14 => Ok(Field_Kind_Closed::TYPE_ENUM),
      15 => Ok(Field_Kind_Closed::TYPE_SFIXED32),
      16 => Ok(Field_Kind_Closed::TYPE_SFIXED64),
      17 => Ok(Field_Kind_Closed::TYPE_SINT32),
      18 => Ok(Field_Kind_Closed::TYPE_SINT64),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for Field_Kind_Closed {
}
impl ::pb::ClosedProtoEnum for Field_Kind_Closed {
  fn name(self) -> &'static str {
    match self {
      Field_Kind_Closed::TYPE_UNKNOWN => "TYPE_UNKNOWN",
      Field_Kind_Closed::TYPE_DOUBLE => "TYPE_DOUBLE",
      Field_Kind_Closed::TYPE_FLOAT => "TYPE_FLOAT",
      Field_Kind_Closed::TYPE_INT64 => "TYPE_INT64",
      Field_Kind_Closed::TYPE_UINT64 => "TYPE_UINT64",
      Field_Kind_Closed::TYPE_INT32 => "TYPE_INT32",
      Field_Kind_Closed::TYPE_FIXED64 => "TYPE_FIXED64",
      Field_Kind_Closed::TYPE_FIXED32 => "TYPE_FIXED32",
      Field_Kind_Closed::TYPE_BOOL => "TYPE_BOOL",
      Field_Kind_Closed::TYPE_STRING => "TYPE_STRING",
      Field_Kind_Closed::TYPE_GROUP => "TYPE_GROUP",
      Field_Kind_Closed::TYPE_MESSAGE => "TYPE_MESSAGE",
      Field_Kind_Closed::TYPE_BYTES => "TYPE_BYTES",
      Field_Kind_Closed::TYPE_UINT32 => "TYPE_UINT32",
      Field_Kind_Closed::TYPE_ENUM => "TYPE_ENUM",
      Field_Kind_Closed::TYPE_SFIXED32 => "TYPE_SFIXED32",
      Field_Kind_Closed::TYPE_SFIXED64 => "TYPE_SFIXED64",
      Field_Kind_Closed::TYPE_SINT32 => "TYPE_SINT32",
      Field_Kind_Closed::TYPE_SINT64 => "TYPE_SINT64",
    }
  }
}

/// Whether a field is optional, required, or repeated.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Field_Cardinality(i32);
impl Field_Cardinality {
  /// For fields with unknown cardinality.
  pub const CARDINALITY_UNKNOWN: Field_Cardinality = Field_Cardinality(0);
  /// For optional fields.
  pub const CARDINALITY_OPTIONAL: Field_Cardinality = Field_Cardinality(1);
  /// For required fields. Proto2 syntax only.
  pub const CARDINALITY_REQUIRED: Field_Cardinality = Field_Cardinality(2);
  /// For repeated fields.
  pub const CARDINALITY_REPEATED: Field_Cardinality = Field_Cardinality(3);
  pub const KNOWN_VARIANTS: [Field_Cardinality; 4] = [Field_Cardinality::CARDINALITY_UNKNOWN, Field_Cardinality::CARDINALITY_OPTIONAL, Field_Cardinality::CARDINALITY_REQUIRED, Field_Cardinality::CARDINALITY_REPEATED];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<Field_Cardinality_Closed> {
    match self {
      Field_Cardinality::CARDINALITY_UNKNOWN => Some(Field_Cardinality_Closed::CARDINALITY_UNKNOWN),
      Field_Cardinality::CARDINALITY_OPTIONAL => Some(Field_Cardinality_Closed::CARDINALITY_OPTIONAL),
      Field_Cardinality::CARDINALITY_REQUIRED => Some(Field_Cardinality_Closed::CARDINALITY_REQUIRED),
      Field_Cardinality::CARDINALITY_REPEATED => Some(Field_Cardinality_Closed::CARDINALITY_REPEATED),
      _ => None,
    }
  }
}
impl ::std::default::Default for Field_Cardinality {
  fn default() -> Self {
    Field_Cardinality::CARDINALITY_UNKNOWN
  }
}
impl From<Field_Cardinality> for i32 {
  fn from(v: Field_Cardinality) -> i32 {
    v.0
  }
}
impl From<i32> for Field_Cardinality {
  fn from(v: i32) -> Field_Cardinality {
    Field_Cardinality(v)
  }
}
impl From<Field_Cardinality_Closed> for Field_Cardinality {
  fn from(v: Field_Cardinality_Closed) -> Field_Cardinality {
    Field_Cardinality(v as i32)
  }
}
impl ::pb::ProtoEnum for Field_Cardinality {
}
impl ::pb::OpenProtoEnum for Field_Cardinality {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      Field_Cardinality::CARDINALITY_UNKNOWN => Some("CARDINALITY_UNKNOWN"),
      Field_Cardinality::CARDINALITY_OPTIONAL => Some("CARDINALITY_OPTIONAL"),
      Field_Cardinality::CARDINALITY_REQUIRED => Some("CARDINALITY_REQUIRED"),
      Field_Cardinality::CARDINALITY_REPEATED => Some("CARDINALITY_REPEATED"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      Field_Cardinality::CARDINALITY_UNKNOWN => true,
      Field_Cardinality::CARDINALITY_OPTIONAL => true,
      Field_Cardinality::CARDINALITY_REQUIRED => true,
      Field_Cardinality::CARDINALITY_REPEATED => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for Field_Cardinality {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// Whether a field is optional, required, or repeated.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum Field_Cardinality_Closed {
  /// For fields with unknown cardinality.
  CARDINALITY_UNKNOWN = 0,
  /// For optional fields.
  CARDINALITY_OPTIONAL = 1,
  /// For required fields. Proto2 syntax only.
  CARDINALITY_REQUIRED = 2,
  /// For repeated fields.
  CARDINALITY_REPEATED = 3,
}
impl Field_Cardinality_Closed {
  pub const KNOWN_VARIANTS: [Field_Cardinality_Closed; 4] = [Field_Cardinality_Closed::CARDINALITY_UNKNOWN, Field_Cardinality_Closed::CARDINALITY_OPTIONAL, Field_Cardinality_Closed::CARDINALITY_REQUIRED, Field_Cardinality_Closed::CARDINALITY_REPEATED];
}
impl ::std::default::Default for Field_Cardinality_Closed {
  fn default() -> Self {
    Field_Cardinality_Closed::CARDINALITY_UNKNOWN
  }
}
impl From<Field_Cardinality_Closed> for i32 {
  fn from(v: Field_Cardinality_Closed) -> i32 {
    match v {
      Field_Cardinality_Closed::CARDINALITY_UNKNOWN => 0,
      Field_Cardinality_Closed::CARDINALITY_OPTIONAL => 1,
      Field_Cardinality_Closed::CARDINALITY_REQUIRED => 2,
      Field_Cardinality_Closed::CARDINALITY_REPEATED => 3,
    }
  }
}
impl ::std::convert::TryFrom<i32> for Field_Cardinality_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(Field_Cardinality_Closed::CARDINALITY_UNKNOWN),
      1 => Ok(Field_Cardinality_Closed::CARDINALITY_OPTIONAL),
      2 => Ok(Field_Cardinality_Closed::CARDINALITY_REQUIRED),
      3 => Ok(Field_Cardinality_Closed::CARDINALITY_REPEATED),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for Field_Cardinality_Closed {
}
impl ::pb::ClosedProtoEnum for Field_Cardinality_Closed {
  fn name(self) -> &'static str {
    match self {
      Field_Cardinality_Closed::CARDINALITY_UNKNOWN => "CARDINALITY_UNKNOWN",
      Field_Cardinality_Closed::CARDINALITY_OPTIONAL => "CARDINALITY_OPTIONAL",
      Field_Cardinality_Closed::CARDINALITY_REQUIRED => "CARDINALITY_REQUIRED",
      Field_Cardinality_Closed::CARDINALITY_REPEATED => "CARDINALITY_REPEATED",
    }
  }
}

/// A protocol buffer message type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Type {
  /// The fully qualified message name.
  pub name: ::std::string::String,
  /// The list of fields.
  pub fields: ::std::vec::Vec<Field>,
  /// The list of types appearing in `oneof` definitions in this type.
  pub oneofs: ::std::vec::Vec<::std::string::String>,
  /// The protocol buffer options.
  pub options: ::std::vec::Vec<Option>,
  /// The source context.
  pub source_context: ::std::option::Option<super::super::protobuf::source_context::SourceContext>,
  /// The source syntax.
  pub syntax: Syntax,
}
impl ::std::default::Default for Type {
  fn default() -> Self {
    Type {
      name: ::std::default::Default::default(),
      fields: ::std::default::Default::default(),
      oneofs: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      source_context: ::std::default::Default::default(),
      syntax: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Type_default: Type = Type::default();
}
impl ::pb::Message for Type {
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
    let mut fields_size = 0;
    for val in &self.fields {
      let l = ::pb::Message::compute_size(val);
      fields_size += ::pb::wire_format::serialized_length(2);
      fields_size += ::pb::varint::serialized_length(l as u64);
      fields_size += l;
    }
    size += fields_size;
    let mut oneofs_size = 0;
    for val in &self.oneofs {
      let l = ::pb::Message::compute_size(val);
      oneofs_size += ::pb::wire_format::serialized_length(3);
      oneofs_size += ::pb::varint::serialized_length(l as u64);
      oneofs_size += l;
    }
    size += oneofs_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(4);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut source_context_size = 0;
    for val in &self.source_context {
      let l = ::pb::Message::compute_size(val);
      source_context_size += ::pb::wire_format::serialized_length(5);
      source_context_size += ::pb::varint::serialized_length(l as u64);
      source_context_size += l;
    }
    size += source_context_size;
    let mut syntax_size = 0;
    if self.syntax != <Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      let l = ::pb::Message::compute_size(val);
      syntax_size += ::pb::wire_format::serialized_length(6);
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
    for val in &self.fields {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.oneofs {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.source_context {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.syntax != <Syntax as ::std::default::Default>::default() {
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
    for val in &self.fields {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.oneofs {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
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
    if self.syntax != <Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      ::pb::wire_format::write(6, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Type", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Type", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Field = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.fields.push(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Type", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.oneofs.push(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Type", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Option = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options.push(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Type", 5)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: super::super::protobuf::source_context::SourceContext = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.source_context = Some(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Type", 6)?;
          let mut val: Syntax = ::std::default::Default::default();
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
impl ::pb::MessageDescriptor for Type {
  const NAME: &'static str = "Type";
  const FULL_NAME: &'static str = "google.protobuf.Type";
}

/// A single field of a message type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field {
  /// The field type.
  pub kind: Field_Kind,
  /// The field cardinality.
  pub cardinality: Field_Cardinality,
  /// The field number.
  pub number: i32,
  /// The field name.
  pub name: ::std::string::String,
  /// The field type URL, without the scheme, for message or enumeration
  /// types. Example: `"type.googleapis.com/google.protobuf.Timestamp"`.
  pub type_url: ::std::string::String,
  /// The index of the field type in `Type.oneofs`, for message or enumeration
  /// types. The first type has index 1; zero means the type is not in the list.
  pub oneof_index: i32,
  /// Whether to use alternative packed wire representation.
  pub packed: bool,
  /// The protocol buffer options.
  pub options: ::std::vec::Vec<Option>,
  /// The field JSON name.
  pub json_name: ::std::string::String,
  /// The string value of the default value of this field. Proto2 syntax only.
  pub default_value: ::std::string::String,
}
impl ::std::default::Default for Field {
  fn default() -> Self {
    Field {
      kind: ::std::default::Default::default(),
      cardinality: ::std::default::Default::default(),
      number: ::std::default::Default::default(),
      name: ::std::default::Default::default(),
      type_url: ::std::default::Default::default(),
      oneof_index: ::std::default::Default::default(),
      packed: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      json_name: ::std::default::Default::default(),
      default_value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Field_default: Field = Field::default();
}
impl ::pb::Message for Field {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut kind_size = 0;
    if self.kind != <Field_Kind as ::std::default::Default>::default() {
      let val = &self.kind;
      let l = ::pb::Message::compute_size(val);
      kind_size += ::pb::wire_format::serialized_length(1);
      kind_size += l;
    }
    size += kind_size;
    let mut cardinality_size = 0;
    if self.cardinality != <Field_Cardinality as ::std::default::Default>::default() {
      let val = &self.cardinality;
      let l = ::pb::Message::compute_size(val);
      cardinality_size += ::pb::wire_format::serialized_length(2);
      cardinality_size += l;
    }
    size += cardinality_size;
    let mut number_size = 0;
    if self.number != <i32 as ::std::default::Default>::default() {
      let val = &self.number;
      let l = ::pb::Message::compute_size(val);
      number_size += ::pb::wire_format::serialized_length(3);
      number_size += l;
    }
    size += number_size;
    let mut name_size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(4);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut type_url_size = 0;
    if self.type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.type_url;
      let l = ::pb::Message::compute_size(val);
      type_url_size += ::pb::wire_format::serialized_length(6);
      type_url_size += ::pb::varint::serialized_length(l as u64);
      type_url_size += l;
    }
    size += type_url_size;
    let mut oneof_index_size = 0;
    if self.oneof_index != <i32 as ::std::default::Default>::default() {
      let val = &self.oneof_index;
      let l = ::pb::Message::compute_size(val);
      oneof_index_size += ::pb::wire_format::serialized_length(7);
      oneof_index_size += l;
    }
    size += oneof_index_size;
    let mut packed_size = 0;
    if self.packed != <bool as ::std::default::Default>::default() {
      let val = &self.packed;
      let l = ::pb::Message::compute_size(val);
      packed_size += ::pb::wire_format::serialized_length(8);
      packed_size += l;
    }
    size += packed_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(9);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut json_name_size = 0;
    if self.json_name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.json_name;
      let l = ::pb::Message::compute_size(val);
      json_name_size += ::pb::wire_format::serialized_length(10);
      json_name_size += ::pb::varint::serialized_length(l as u64);
      json_name_size += l;
    }
    size += json_name_size;
    let mut default_value_size = 0;
    if self.default_value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.default_value;
      let l = ::pb::Message::compute_size(val);
      default_value_size += ::pb::wire_format::serialized_length(11);
      default_value_size += ::pb::varint::serialized_length(l as u64);
      default_value_size += l;
    }
    size += default_value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.kind != <Field_Kind as ::std::default::Default>::default() {
      let val = &self.kind;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.cardinality != <Field_Cardinality as ::std::default::Default>::default() {
      let val = &self.cardinality;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.number != <i32 as ::std::default::Default>::default() {
      let val = &self.number;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.type_url;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.oneof_index != <i32 as ::std::default::Default>::default() {
      let val = &self.oneof_index;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.packed != <bool as ::std::default::Default>::default() {
      let val = &self.packed;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.json_name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.json_name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.default_value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.default_value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.kind != <Field_Kind as ::std::default::Default>::default() {
      let val = &self.kind;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.cardinality != <Field_Cardinality as ::std::default::Default>::default() {
      let val = &self.cardinality;
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.number != <i32 as ::std::default::Default>::default() {
      let val = &self.number;
      ::pb::wire_format::write(3, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.type_url != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.type_url;
      ::pb::wire_format::write(6, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.oneof_index != <i32 as ::std::default::Default>::default() {
      let val = &self.oneof_index;
      ::pb::wire_format::write(7, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.packed != <bool as ::std::default::Default>::default() {
      let val = &self.packed;
      ::pb::wire_format::write(8, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
      ::pb::wire_format::write(9, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.json_name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.json_name;
      ::pb::wire_format::write(10, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.default_value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.default_value;
      ::pb::wire_format::write(11, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Field", 1)?;
          let mut val: Field_Kind = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.kind = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Field", 2)?;
          let mut val: Field_Cardinality = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.cardinality = val;
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Field", 3)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.number = val;
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Field", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Field", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.type_url = val;
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Field", 7)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.oneof_index = val;
        }
        8 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Field", 8)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.packed = val;
        }
        9 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Field", 9)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Option = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options.push(val);
        }
        10 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Field", 10)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.json_name = val;
        }
        11 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Field", 11)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.default_value = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Field {
  const NAME: &'static str = "Field";
  const FULL_NAME: &'static str = "google.protobuf.Field";
}

/// Enum type definition.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Enum {
  /// Enum type name.
  pub name: ::std::string::String,
  /// Enum value definitions.
  pub enumvalue: ::std::vec::Vec<EnumValue>,
  /// Protocol buffer options.
  pub options: ::std::vec::Vec<Option>,
  /// The source context.
  pub source_context: ::std::option::Option<super::super::protobuf::source_context::SourceContext>,
  /// The source syntax.
  pub syntax: Syntax,
}
impl ::std::default::Default for Enum {
  fn default() -> Self {
    Enum {
      name: ::std::default::Default::default(),
      enumvalue: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      source_context: ::std::default::Default::default(),
      syntax: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Enum_default: Enum = Enum::default();
}
impl ::pb::Message for Enum {
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
    let mut enumvalue_size = 0;
    for val in &self.enumvalue {
      let l = ::pb::Message::compute_size(val);
      enumvalue_size += ::pb::wire_format::serialized_length(2);
      enumvalue_size += ::pb::varint::serialized_length(l as u64);
      enumvalue_size += l;
    }
    size += enumvalue_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(3);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut source_context_size = 0;
    for val in &self.source_context {
      let l = ::pb::Message::compute_size(val);
      source_context_size += ::pb::wire_format::serialized_length(4);
      source_context_size += ::pb::varint::serialized_length(l as u64);
      source_context_size += l;
    }
    size += source_context_size;
    let mut syntax_size = 0;
    if self.syntax != <Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      let l = ::pb::Message::compute_size(val);
      syntax_size += ::pb::wire_format::serialized_length(5);
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
    for val in &self.enumvalue {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.source_context {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.syntax != <Syntax as ::std::default::Default>::default() {
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
    for val in &self.enumvalue {
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
    for val in &self.source_context {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.syntax != <Syntax as ::std::default::Default>::default() {
      let val = &self.syntax;
      ::pb::wire_format::write(5, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Enum", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Enum", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: EnumValue = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.enumvalue.push(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Enum", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Option = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options.push(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Enum", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: super::super::protobuf::source_context::SourceContext = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.source_context = Some(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Enum", 5)?;
          let mut val: Syntax = ::std::default::Default::default();
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
impl ::pb::MessageDescriptor for Enum {
  const NAME: &'static str = "Enum";
  const FULL_NAME: &'static str = "google.protobuf.Enum";
}

/// Enum value definition.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnumValue {
  /// Enum value name.
  pub name: ::std::string::String,
  /// Enum value number.
  pub number: i32,
  /// Protocol buffer options.
  pub options: ::std::vec::Vec<Option>,
}
impl ::std::default::Default for EnumValue {
  fn default() -> Self {
    EnumValue {
      name: ::std::default::Default::default(),
      number: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumValue_default: EnumValue = EnumValue::default();
}
impl ::pb::Message for EnumValue {
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
    let mut number_size = 0;
    if self.number != <i32 as ::std::default::Default>::default() {
      let val = &self.number;
      let l = ::pb::Message::compute_size(val);
      number_size += ::pb::wire_format::serialized_length(2);
      number_size += l;
    }
    size += number_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(3);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.number != <i32 as ::std::default::Default>::default() {
      let val = &self.number;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
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
    if self.number != <i32 as ::std::default::Default>::default() {
      let val = &self.number;
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumValue", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "EnumValue", 2)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.number = val;
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumValue", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Option = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for EnumValue {
  const NAME: &'static str = "EnumValue";
  const FULL_NAME: &'static str = "google.protobuf.EnumValue";
}

/// A protocol buffer option, which can be attached to a message, field,
/// enumeration, etc.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Option {
  /// The option's name. For protobuf built-in options (options defined in
  /// descriptor.proto), this is the short name. For example, `"map_entry"`.
  /// For custom options, it should be the fully-qualified name. For example,
  /// `"google.api.http"`.
  pub name: ::std::string::String,
  /// The option's value packed in an Any message. If the value is a primitive,
  /// the corresponding wrapper type defined in google/protobuf/wrappers.proto
  /// should be used. If the value is an enum, it should be stored as an int32
  /// value using the google.protobuf.Int32Value type.
  pub value: ::std::option::Option<super::super::protobuf::any::Any>,
}
impl ::std::default::Default for Option {
  fn default() -> Self {
    Option {
      name: ::std::default::Default::default(),
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Option_default: Option = Option::default();
}
impl ::pb::Message for Option {
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
    let mut value_size = 0;
    for val in &self.value {
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
    if self.name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.value {
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
    for val in &self.value {
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Option", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Option", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: super::super::protobuf::any::Any = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.value = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Option {
  const NAME: &'static str = "Option";
  const FULL_NAME: &'static str = "google.protobuf.Option";
}

