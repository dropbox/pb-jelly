// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldDescriptorProto_Type(i32);
impl FieldDescriptorProto_Type {
  /// 0 is reserved for errors.
  /// Order is weird for historical reasons.
  pub const TYPE_DOUBLE: FieldDescriptorProto_Type = FieldDescriptorProto_Type(1);
  pub const TYPE_FLOAT: FieldDescriptorProto_Type = FieldDescriptorProto_Type(2);
  /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
  /// negative values are likely.
  pub const TYPE_INT64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(3);
  pub const TYPE_UINT64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(4);
  /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
  /// negative values are likely.
  pub const TYPE_INT32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(5);
  pub const TYPE_FIXED64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(6);
  pub const TYPE_FIXED32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(7);
  pub const TYPE_BOOL: FieldDescriptorProto_Type = FieldDescriptorProto_Type(8);
  pub const TYPE_STRING: FieldDescriptorProto_Type = FieldDescriptorProto_Type(9);
  /// Tag-delimited aggregate.
  /// Group type is deprecated and not supported in proto3. However, Proto3
  /// implementations should still be able to parse the group wire format and
  /// treat group fields as unknown fields.
  pub const TYPE_GROUP: FieldDescriptorProto_Type = FieldDescriptorProto_Type(10);
  /// Length-delimited aggregate.
  pub const TYPE_MESSAGE: FieldDescriptorProto_Type = FieldDescriptorProto_Type(11);
  /// New in version 2.
  pub const TYPE_BYTES: FieldDescriptorProto_Type = FieldDescriptorProto_Type(12);
  pub const TYPE_UINT32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(13);
  pub const TYPE_ENUM: FieldDescriptorProto_Type = FieldDescriptorProto_Type(14);
  pub const TYPE_SFIXED32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(15);
  pub const TYPE_SFIXED64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(16);
  /// Uses ZigZag encoding.
  pub const TYPE_SINT32: FieldDescriptorProto_Type = FieldDescriptorProto_Type(17);
  /// Uses ZigZag encoding.
  pub const TYPE_SINT64: FieldDescriptorProto_Type = FieldDescriptorProto_Type(18);
  pub const KNOWN_VARIANTS: [FieldDescriptorProto_Type; 18] = [FieldDescriptorProto_Type::TYPE_DOUBLE, FieldDescriptorProto_Type::TYPE_FLOAT, FieldDescriptorProto_Type::TYPE_INT64, FieldDescriptorProto_Type::TYPE_UINT64, FieldDescriptorProto_Type::TYPE_INT32, FieldDescriptorProto_Type::TYPE_FIXED64, FieldDescriptorProto_Type::TYPE_FIXED32, FieldDescriptorProto_Type::TYPE_BOOL, FieldDescriptorProto_Type::TYPE_STRING, FieldDescriptorProto_Type::TYPE_GROUP, FieldDescriptorProto_Type::TYPE_MESSAGE, FieldDescriptorProto_Type::TYPE_BYTES, FieldDescriptorProto_Type::TYPE_UINT32, FieldDescriptorProto_Type::TYPE_ENUM, FieldDescriptorProto_Type::TYPE_SFIXED32, FieldDescriptorProto_Type::TYPE_SFIXED64, FieldDescriptorProto_Type::TYPE_SINT32, FieldDescriptorProto_Type::TYPE_SINT64];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<FieldDescriptorProto_Type_Closed> {
    match self {
      FieldDescriptorProto_Type::TYPE_DOUBLE => Some(FieldDescriptorProto_Type_Closed::TYPE_DOUBLE),
      FieldDescriptorProto_Type::TYPE_FLOAT => Some(FieldDescriptorProto_Type_Closed::TYPE_FLOAT),
      FieldDescriptorProto_Type::TYPE_INT64 => Some(FieldDescriptorProto_Type_Closed::TYPE_INT64),
      FieldDescriptorProto_Type::TYPE_UINT64 => Some(FieldDescriptorProto_Type_Closed::TYPE_UINT64),
      FieldDescriptorProto_Type::TYPE_INT32 => Some(FieldDescriptorProto_Type_Closed::TYPE_INT32),
      FieldDescriptorProto_Type::TYPE_FIXED64 => Some(FieldDescriptorProto_Type_Closed::TYPE_FIXED64),
      FieldDescriptorProto_Type::TYPE_FIXED32 => Some(FieldDescriptorProto_Type_Closed::TYPE_FIXED32),
      FieldDescriptorProto_Type::TYPE_BOOL => Some(FieldDescriptorProto_Type_Closed::TYPE_BOOL),
      FieldDescriptorProto_Type::TYPE_STRING => Some(FieldDescriptorProto_Type_Closed::TYPE_STRING),
      FieldDescriptorProto_Type::TYPE_GROUP => Some(FieldDescriptorProto_Type_Closed::TYPE_GROUP),
      FieldDescriptorProto_Type::TYPE_MESSAGE => Some(FieldDescriptorProto_Type_Closed::TYPE_MESSAGE),
      FieldDescriptorProto_Type::TYPE_BYTES => Some(FieldDescriptorProto_Type_Closed::TYPE_BYTES),
      FieldDescriptorProto_Type::TYPE_UINT32 => Some(FieldDescriptorProto_Type_Closed::TYPE_UINT32),
      FieldDescriptorProto_Type::TYPE_ENUM => Some(FieldDescriptorProto_Type_Closed::TYPE_ENUM),
      FieldDescriptorProto_Type::TYPE_SFIXED32 => Some(FieldDescriptorProto_Type_Closed::TYPE_SFIXED32),
      FieldDescriptorProto_Type::TYPE_SFIXED64 => Some(FieldDescriptorProto_Type_Closed::TYPE_SFIXED64),
      FieldDescriptorProto_Type::TYPE_SINT32 => Some(FieldDescriptorProto_Type_Closed::TYPE_SINT32),
      FieldDescriptorProto_Type::TYPE_SINT64 => Some(FieldDescriptorProto_Type_Closed::TYPE_SINT64),
      _ => None,
    }
  }
}
impl ::std::default::Default for FieldDescriptorProto_Type {
  fn default() -> Self {
    FieldDescriptorProto_Type::TYPE_DOUBLE
  }
}
impl From<FieldDescriptorProto_Type> for i32 {
  fn from(v: FieldDescriptorProto_Type) -> i32 {
    v.0
  }
}
impl From<i32> for FieldDescriptorProto_Type {
  fn from(v: i32) -> FieldDescriptorProto_Type {
    FieldDescriptorProto_Type(v)
  }
}
impl From<FieldDescriptorProto_Type_Closed> for FieldDescriptorProto_Type {
  fn from(v: FieldDescriptorProto_Type_Closed) -> FieldDescriptorProto_Type {
    FieldDescriptorProto_Type(v as i32)
  }
}
impl ::pb::ProtoEnum for FieldDescriptorProto_Type {
}
impl ::pb::OpenProtoEnum for FieldDescriptorProto_Type {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      FieldDescriptorProto_Type::TYPE_DOUBLE => Some("TYPE_DOUBLE"),
      FieldDescriptorProto_Type::TYPE_FLOAT => Some("TYPE_FLOAT"),
      FieldDescriptorProto_Type::TYPE_INT64 => Some("TYPE_INT64"),
      FieldDescriptorProto_Type::TYPE_UINT64 => Some("TYPE_UINT64"),
      FieldDescriptorProto_Type::TYPE_INT32 => Some("TYPE_INT32"),
      FieldDescriptorProto_Type::TYPE_FIXED64 => Some("TYPE_FIXED64"),
      FieldDescriptorProto_Type::TYPE_FIXED32 => Some("TYPE_FIXED32"),
      FieldDescriptorProto_Type::TYPE_BOOL => Some("TYPE_BOOL"),
      FieldDescriptorProto_Type::TYPE_STRING => Some("TYPE_STRING"),
      FieldDescriptorProto_Type::TYPE_GROUP => Some("TYPE_GROUP"),
      FieldDescriptorProto_Type::TYPE_MESSAGE => Some("TYPE_MESSAGE"),
      FieldDescriptorProto_Type::TYPE_BYTES => Some("TYPE_BYTES"),
      FieldDescriptorProto_Type::TYPE_UINT32 => Some("TYPE_UINT32"),
      FieldDescriptorProto_Type::TYPE_ENUM => Some("TYPE_ENUM"),
      FieldDescriptorProto_Type::TYPE_SFIXED32 => Some("TYPE_SFIXED32"),
      FieldDescriptorProto_Type::TYPE_SFIXED64 => Some("TYPE_SFIXED64"),
      FieldDescriptorProto_Type::TYPE_SINT32 => Some("TYPE_SINT32"),
      FieldDescriptorProto_Type::TYPE_SINT64 => Some("TYPE_SINT64"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      FieldDescriptorProto_Type::TYPE_DOUBLE => true,
      FieldDescriptorProto_Type::TYPE_FLOAT => true,
      FieldDescriptorProto_Type::TYPE_INT64 => true,
      FieldDescriptorProto_Type::TYPE_UINT64 => true,
      FieldDescriptorProto_Type::TYPE_INT32 => true,
      FieldDescriptorProto_Type::TYPE_FIXED64 => true,
      FieldDescriptorProto_Type::TYPE_FIXED32 => true,
      FieldDescriptorProto_Type::TYPE_BOOL => true,
      FieldDescriptorProto_Type::TYPE_STRING => true,
      FieldDescriptorProto_Type::TYPE_GROUP => true,
      FieldDescriptorProto_Type::TYPE_MESSAGE => true,
      FieldDescriptorProto_Type::TYPE_BYTES => true,
      FieldDescriptorProto_Type::TYPE_UINT32 => true,
      FieldDescriptorProto_Type::TYPE_ENUM => true,
      FieldDescriptorProto_Type::TYPE_SFIXED32 => true,
      FieldDescriptorProto_Type::TYPE_SFIXED64 => true,
      FieldDescriptorProto_Type::TYPE_SINT32 => true,
      FieldDescriptorProto_Type::TYPE_SINT64 => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for FieldDescriptorProto_Type {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FieldDescriptorProto_Type_Closed {
  /// 0 is reserved for errors.
  /// Order is weird for historical reasons.
  TYPE_DOUBLE = 1,
  TYPE_FLOAT = 2,
  /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT64 if
  /// negative values are likely.
  TYPE_INT64 = 3,
  TYPE_UINT64 = 4,
  /// Not ZigZag encoded.  Negative numbers take 10 bytes.  Use TYPE_SINT32 if
  /// negative values are likely.
  TYPE_INT32 = 5,
  TYPE_FIXED64 = 6,
  TYPE_FIXED32 = 7,
  TYPE_BOOL = 8,
  TYPE_STRING = 9,
  /// Tag-delimited aggregate.
  /// Group type is deprecated and not supported in proto3. However, Proto3
  /// implementations should still be able to parse the group wire format and
  /// treat group fields as unknown fields.
  TYPE_GROUP = 10,
  /// Length-delimited aggregate.
  TYPE_MESSAGE = 11,
  /// New in version 2.
  TYPE_BYTES = 12,
  TYPE_UINT32 = 13,
  TYPE_ENUM = 14,
  TYPE_SFIXED32 = 15,
  TYPE_SFIXED64 = 16,
  /// Uses ZigZag encoding.
  TYPE_SINT32 = 17,
  /// Uses ZigZag encoding.
  TYPE_SINT64 = 18,
}
impl FieldDescriptorProto_Type_Closed {
  pub const KNOWN_VARIANTS: [FieldDescriptorProto_Type_Closed; 18] = [FieldDescriptorProto_Type_Closed::TYPE_DOUBLE, FieldDescriptorProto_Type_Closed::TYPE_FLOAT, FieldDescriptorProto_Type_Closed::TYPE_INT64, FieldDescriptorProto_Type_Closed::TYPE_UINT64, FieldDescriptorProto_Type_Closed::TYPE_INT32, FieldDescriptorProto_Type_Closed::TYPE_FIXED64, FieldDescriptorProto_Type_Closed::TYPE_FIXED32, FieldDescriptorProto_Type_Closed::TYPE_BOOL, FieldDescriptorProto_Type_Closed::TYPE_STRING, FieldDescriptorProto_Type_Closed::TYPE_GROUP, FieldDescriptorProto_Type_Closed::TYPE_MESSAGE, FieldDescriptorProto_Type_Closed::TYPE_BYTES, FieldDescriptorProto_Type_Closed::TYPE_UINT32, FieldDescriptorProto_Type_Closed::TYPE_ENUM, FieldDescriptorProto_Type_Closed::TYPE_SFIXED32, FieldDescriptorProto_Type_Closed::TYPE_SFIXED64, FieldDescriptorProto_Type_Closed::TYPE_SINT32, FieldDescriptorProto_Type_Closed::TYPE_SINT64];
}
impl ::std::default::Default for FieldDescriptorProto_Type_Closed {
  fn default() -> Self {
    FieldDescriptorProto_Type_Closed::TYPE_DOUBLE
  }
}
impl From<FieldDescriptorProto_Type_Closed> for i32 {
  fn from(v: FieldDescriptorProto_Type_Closed) -> i32 {
    match v {
      FieldDescriptorProto_Type_Closed::TYPE_DOUBLE => 1,
      FieldDescriptorProto_Type_Closed::TYPE_FLOAT => 2,
      FieldDescriptorProto_Type_Closed::TYPE_INT64 => 3,
      FieldDescriptorProto_Type_Closed::TYPE_UINT64 => 4,
      FieldDescriptorProto_Type_Closed::TYPE_INT32 => 5,
      FieldDescriptorProto_Type_Closed::TYPE_FIXED64 => 6,
      FieldDescriptorProto_Type_Closed::TYPE_FIXED32 => 7,
      FieldDescriptorProto_Type_Closed::TYPE_BOOL => 8,
      FieldDescriptorProto_Type_Closed::TYPE_STRING => 9,
      FieldDescriptorProto_Type_Closed::TYPE_GROUP => 10,
      FieldDescriptorProto_Type_Closed::TYPE_MESSAGE => 11,
      FieldDescriptorProto_Type_Closed::TYPE_BYTES => 12,
      FieldDescriptorProto_Type_Closed::TYPE_UINT32 => 13,
      FieldDescriptorProto_Type_Closed::TYPE_ENUM => 14,
      FieldDescriptorProto_Type_Closed::TYPE_SFIXED32 => 15,
      FieldDescriptorProto_Type_Closed::TYPE_SFIXED64 => 16,
      FieldDescriptorProto_Type_Closed::TYPE_SINT32 => 17,
      FieldDescriptorProto_Type_Closed::TYPE_SINT64 => 18,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FieldDescriptorProto_Type_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      1 => Ok(FieldDescriptorProto_Type_Closed::TYPE_DOUBLE),
      2 => Ok(FieldDescriptorProto_Type_Closed::TYPE_FLOAT),
      3 => Ok(FieldDescriptorProto_Type_Closed::TYPE_INT64),
      4 => Ok(FieldDescriptorProto_Type_Closed::TYPE_UINT64),
      5 => Ok(FieldDescriptorProto_Type_Closed::TYPE_INT32),
      6 => Ok(FieldDescriptorProto_Type_Closed::TYPE_FIXED64),
      7 => Ok(FieldDescriptorProto_Type_Closed::TYPE_FIXED32),
      8 => Ok(FieldDescriptorProto_Type_Closed::TYPE_BOOL),
      9 => Ok(FieldDescriptorProto_Type_Closed::TYPE_STRING),
      10 => Ok(FieldDescriptorProto_Type_Closed::TYPE_GROUP),
      11 => Ok(FieldDescriptorProto_Type_Closed::TYPE_MESSAGE),
      12 => Ok(FieldDescriptorProto_Type_Closed::TYPE_BYTES),
      13 => Ok(FieldDescriptorProto_Type_Closed::TYPE_UINT32),
      14 => Ok(FieldDescriptorProto_Type_Closed::TYPE_ENUM),
      15 => Ok(FieldDescriptorProto_Type_Closed::TYPE_SFIXED32),
      16 => Ok(FieldDescriptorProto_Type_Closed::TYPE_SFIXED64),
      17 => Ok(FieldDescriptorProto_Type_Closed::TYPE_SINT32),
      18 => Ok(FieldDescriptorProto_Type_Closed::TYPE_SINT64),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for FieldDescriptorProto_Type_Closed {
}
impl ::pb::ClosedProtoEnum for FieldDescriptorProto_Type_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldDescriptorProto_Type_Closed::TYPE_DOUBLE => "TYPE_DOUBLE",
      FieldDescriptorProto_Type_Closed::TYPE_FLOAT => "TYPE_FLOAT",
      FieldDescriptorProto_Type_Closed::TYPE_INT64 => "TYPE_INT64",
      FieldDescriptorProto_Type_Closed::TYPE_UINT64 => "TYPE_UINT64",
      FieldDescriptorProto_Type_Closed::TYPE_INT32 => "TYPE_INT32",
      FieldDescriptorProto_Type_Closed::TYPE_FIXED64 => "TYPE_FIXED64",
      FieldDescriptorProto_Type_Closed::TYPE_FIXED32 => "TYPE_FIXED32",
      FieldDescriptorProto_Type_Closed::TYPE_BOOL => "TYPE_BOOL",
      FieldDescriptorProto_Type_Closed::TYPE_STRING => "TYPE_STRING",
      FieldDescriptorProto_Type_Closed::TYPE_GROUP => "TYPE_GROUP",
      FieldDescriptorProto_Type_Closed::TYPE_MESSAGE => "TYPE_MESSAGE",
      FieldDescriptorProto_Type_Closed::TYPE_BYTES => "TYPE_BYTES",
      FieldDescriptorProto_Type_Closed::TYPE_UINT32 => "TYPE_UINT32",
      FieldDescriptorProto_Type_Closed::TYPE_ENUM => "TYPE_ENUM",
      FieldDescriptorProto_Type_Closed::TYPE_SFIXED32 => "TYPE_SFIXED32",
      FieldDescriptorProto_Type_Closed::TYPE_SFIXED64 => "TYPE_SFIXED64",
      FieldDescriptorProto_Type_Closed::TYPE_SINT32 => "TYPE_SINT32",
      FieldDescriptorProto_Type_Closed::TYPE_SINT64 => "TYPE_SINT64",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldDescriptorProto_Label(i32);
impl FieldDescriptorProto_Label {
  /// 0 is reserved for errors
  pub const LABEL_OPTIONAL: FieldDescriptorProto_Label = FieldDescriptorProto_Label(1);
  pub const LABEL_REQUIRED: FieldDescriptorProto_Label = FieldDescriptorProto_Label(2);
  pub const LABEL_REPEATED: FieldDescriptorProto_Label = FieldDescriptorProto_Label(3);
  pub const KNOWN_VARIANTS: [FieldDescriptorProto_Label; 3] = [FieldDescriptorProto_Label::LABEL_OPTIONAL, FieldDescriptorProto_Label::LABEL_REQUIRED, FieldDescriptorProto_Label::LABEL_REPEATED];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<FieldDescriptorProto_Label_Closed> {
    match self {
      FieldDescriptorProto_Label::LABEL_OPTIONAL => Some(FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL),
      FieldDescriptorProto_Label::LABEL_REQUIRED => Some(FieldDescriptorProto_Label_Closed::LABEL_REQUIRED),
      FieldDescriptorProto_Label::LABEL_REPEATED => Some(FieldDescriptorProto_Label_Closed::LABEL_REPEATED),
      _ => None,
    }
  }
}
impl ::std::default::Default for FieldDescriptorProto_Label {
  fn default() -> Self {
    FieldDescriptorProto_Label::LABEL_OPTIONAL
  }
}
impl From<FieldDescriptorProto_Label> for i32 {
  fn from(v: FieldDescriptorProto_Label) -> i32 {
    v.0
  }
}
impl From<i32> for FieldDescriptorProto_Label {
  fn from(v: i32) -> FieldDescriptorProto_Label {
    FieldDescriptorProto_Label(v)
  }
}
impl From<FieldDescriptorProto_Label_Closed> for FieldDescriptorProto_Label {
  fn from(v: FieldDescriptorProto_Label_Closed) -> FieldDescriptorProto_Label {
    FieldDescriptorProto_Label(v as i32)
  }
}
impl ::pb::ProtoEnum for FieldDescriptorProto_Label {
}
impl ::pb::OpenProtoEnum for FieldDescriptorProto_Label {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      FieldDescriptorProto_Label::LABEL_OPTIONAL => Some("LABEL_OPTIONAL"),
      FieldDescriptorProto_Label::LABEL_REQUIRED => Some("LABEL_REQUIRED"),
      FieldDescriptorProto_Label::LABEL_REPEATED => Some("LABEL_REPEATED"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      FieldDescriptorProto_Label::LABEL_OPTIONAL => true,
      FieldDescriptorProto_Label::LABEL_REQUIRED => true,
      FieldDescriptorProto_Label::LABEL_REPEATED => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for FieldDescriptorProto_Label {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FieldDescriptorProto_Label_Closed {
  /// 0 is reserved for errors
  LABEL_OPTIONAL = 1,
  LABEL_REQUIRED = 2,
  LABEL_REPEATED = 3,
}
impl FieldDescriptorProto_Label_Closed {
  pub const KNOWN_VARIANTS: [FieldDescriptorProto_Label_Closed; 3] = [FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL, FieldDescriptorProto_Label_Closed::LABEL_REQUIRED, FieldDescriptorProto_Label_Closed::LABEL_REPEATED];
}
impl ::std::default::Default for FieldDescriptorProto_Label_Closed {
  fn default() -> Self {
    FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL
  }
}
impl From<FieldDescriptorProto_Label_Closed> for i32 {
  fn from(v: FieldDescriptorProto_Label_Closed) -> i32 {
    match v {
      FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL => 1,
      FieldDescriptorProto_Label_Closed::LABEL_REQUIRED => 2,
      FieldDescriptorProto_Label_Closed::LABEL_REPEATED => 3,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FieldDescriptorProto_Label_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      1 => Ok(FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL),
      2 => Ok(FieldDescriptorProto_Label_Closed::LABEL_REQUIRED),
      3 => Ok(FieldDescriptorProto_Label_Closed::LABEL_REPEATED),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for FieldDescriptorProto_Label_Closed {
}
impl ::pb::ClosedProtoEnum for FieldDescriptorProto_Label_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL => "LABEL_OPTIONAL",
      FieldDescriptorProto_Label_Closed::LABEL_REQUIRED => "LABEL_REQUIRED",
      FieldDescriptorProto_Label_Closed::LABEL_REPEATED => "LABEL_REPEATED",
    }
  }
}

/// Generated classes can be optimized for speed or code size.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FileOptions_OptimizeMode(i32);
impl FileOptions_OptimizeMode {
  /// Generate complete code for parsing, serialization,
  pub const SPEED: FileOptions_OptimizeMode = FileOptions_OptimizeMode(1);
  /// etc.
  /// Use ReflectionOps to implement these methods.
  pub const CODE_SIZE: FileOptions_OptimizeMode = FileOptions_OptimizeMode(2);
  /// Generate code using MessageLite and the lite runtime.
  pub const LITE_RUNTIME: FileOptions_OptimizeMode = FileOptions_OptimizeMode(3);
  pub const KNOWN_VARIANTS: [FileOptions_OptimizeMode; 3] = [FileOptions_OptimizeMode::SPEED, FileOptions_OptimizeMode::CODE_SIZE, FileOptions_OptimizeMode::LITE_RUNTIME];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<FileOptions_OptimizeMode_Closed> {
    match self {
      FileOptions_OptimizeMode::SPEED => Some(FileOptions_OptimizeMode_Closed::SPEED),
      FileOptions_OptimizeMode::CODE_SIZE => Some(FileOptions_OptimizeMode_Closed::CODE_SIZE),
      FileOptions_OptimizeMode::LITE_RUNTIME => Some(FileOptions_OptimizeMode_Closed::LITE_RUNTIME),
      _ => None,
    }
  }
}
impl ::std::default::Default for FileOptions_OptimizeMode {
  fn default() -> Self {
    FileOptions_OptimizeMode::SPEED
  }
}
impl From<FileOptions_OptimizeMode> for i32 {
  fn from(v: FileOptions_OptimizeMode) -> i32 {
    v.0
  }
}
impl From<i32> for FileOptions_OptimizeMode {
  fn from(v: i32) -> FileOptions_OptimizeMode {
    FileOptions_OptimizeMode(v)
  }
}
impl From<FileOptions_OptimizeMode_Closed> for FileOptions_OptimizeMode {
  fn from(v: FileOptions_OptimizeMode_Closed) -> FileOptions_OptimizeMode {
    FileOptions_OptimizeMode(v as i32)
  }
}
impl ::pb::ProtoEnum for FileOptions_OptimizeMode {
}
impl ::pb::OpenProtoEnum for FileOptions_OptimizeMode {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      FileOptions_OptimizeMode::SPEED => Some("SPEED"),
      FileOptions_OptimizeMode::CODE_SIZE => Some("CODE_SIZE"),
      FileOptions_OptimizeMode::LITE_RUNTIME => Some("LITE_RUNTIME"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      FileOptions_OptimizeMode::SPEED => true,
      FileOptions_OptimizeMode::CODE_SIZE => true,
      FileOptions_OptimizeMode::LITE_RUNTIME => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for FileOptions_OptimizeMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// Generated classes can be optimized for speed or code size.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FileOptions_OptimizeMode_Closed {
  /// Generate complete code for parsing, serialization,
  SPEED = 1,
  /// etc.
  /// Use ReflectionOps to implement these methods.
  CODE_SIZE = 2,
  /// Generate code using MessageLite and the lite runtime.
  LITE_RUNTIME = 3,
}
impl FileOptions_OptimizeMode_Closed {
  pub const KNOWN_VARIANTS: [FileOptions_OptimizeMode_Closed; 3] = [FileOptions_OptimizeMode_Closed::SPEED, FileOptions_OptimizeMode_Closed::CODE_SIZE, FileOptions_OptimizeMode_Closed::LITE_RUNTIME];
}
impl ::std::default::Default for FileOptions_OptimizeMode_Closed {
  fn default() -> Self {
    FileOptions_OptimizeMode_Closed::SPEED
  }
}
impl From<FileOptions_OptimizeMode_Closed> for i32 {
  fn from(v: FileOptions_OptimizeMode_Closed) -> i32 {
    match v {
      FileOptions_OptimizeMode_Closed::SPEED => 1,
      FileOptions_OptimizeMode_Closed::CODE_SIZE => 2,
      FileOptions_OptimizeMode_Closed::LITE_RUNTIME => 3,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FileOptions_OptimizeMode_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      1 => Ok(FileOptions_OptimizeMode_Closed::SPEED),
      2 => Ok(FileOptions_OptimizeMode_Closed::CODE_SIZE),
      3 => Ok(FileOptions_OptimizeMode_Closed::LITE_RUNTIME),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for FileOptions_OptimizeMode_Closed {
}
impl ::pb::ClosedProtoEnum for FileOptions_OptimizeMode_Closed {
  fn name(self) -> &'static str {
    match self {
      FileOptions_OptimizeMode_Closed::SPEED => "SPEED",
      FileOptions_OptimizeMode_Closed::CODE_SIZE => "CODE_SIZE",
      FileOptions_OptimizeMode_Closed::LITE_RUNTIME => "LITE_RUNTIME",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldOptions_CType(i32);
impl FieldOptions_CType {
  /// Default mode.
  pub const STRING: FieldOptions_CType = FieldOptions_CType(0);
  pub const CORD: FieldOptions_CType = FieldOptions_CType(1);
  pub const STRING_PIECE: FieldOptions_CType = FieldOptions_CType(2);
  pub const KNOWN_VARIANTS: [FieldOptions_CType; 3] = [FieldOptions_CType::STRING, FieldOptions_CType::CORD, FieldOptions_CType::STRING_PIECE];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<FieldOptions_CType_Closed> {
    match self {
      FieldOptions_CType::STRING => Some(FieldOptions_CType_Closed::STRING),
      FieldOptions_CType::CORD => Some(FieldOptions_CType_Closed::CORD),
      FieldOptions_CType::STRING_PIECE => Some(FieldOptions_CType_Closed::STRING_PIECE),
      _ => None,
    }
  }
}
impl ::std::default::Default for FieldOptions_CType {
  fn default() -> Self {
    FieldOptions_CType::STRING
  }
}
impl From<FieldOptions_CType> for i32 {
  fn from(v: FieldOptions_CType) -> i32 {
    v.0
  }
}
impl From<i32> for FieldOptions_CType {
  fn from(v: i32) -> FieldOptions_CType {
    FieldOptions_CType(v)
  }
}
impl From<FieldOptions_CType_Closed> for FieldOptions_CType {
  fn from(v: FieldOptions_CType_Closed) -> FieldOptions_CType {
    FieldOptions_CType(v as i32)
  }
}
impl ::pb::ProtoEnum for FieldOptions_CType {
}
impl ::pb::OpenProtoEnum for FieldOptions_CType {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      FieldOptions_CType::STRING => Some("STRING"),
      FieldOptions_CType::CORD => Some("CORD"),
      FieldOptions_CType::STRING_PIECE => Some("STRING_PIECE"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      FieldOptions_CType::STRING => true,
      FieldOptions_CType::CORD => true,
      FieldOptions_CType::STRING_PIECE => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for FieldOptions_CType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FieldOptions_CType_Closed {
  /// Default mode.
  STRING = 0,
  CORD = 1,
  STRING_PIECE = 2,
}
impl FieldOptions_CType_Closed {
  pub const KNOWN_VARIANTS: [FieldOptions_CType_Closed; 3] = [FieldOptions_CType_Closed::STRING, FieldOptions_CType_Closed::CORD, FieldOptions_CType_Closed::STRING_PIECE];
}
impl ::std::default::Default for FieldOptions_CType_Closed {
  fn default() -> Self {
    FieldOptions_CType_Closed::STRING
  }
}
impl From<FieldOptions_CType_Closed> for i32 {
  fn from(v: FieldOptions_CType_Closed) -> i32 {
    match v {
      FieldOptions_CType_Closed::STRING => 0,
      FieldOptions_CType_Closed::CORD => 1,
      FieldOptions_CType_Closed::STRING_PIECE => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FieldOptions_CType_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FieldOptions_CType_Closed::STRING),
      1 => Ok(FieldOptions_CType_Closed::CORD),
      2 => Ok(FieldOptions_CType_Closed::STRING_PIECE),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for FieldOptions_CType_Closed {
}
impl ::pb::ClosedProtoEnum for FieldOptions_CType_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldOptions_CType_Closed::STRING => "STRING",
      FieldOptions_CType_Closed::CORD => "CORD",
      FieldOptions_CType_Closed::STRING_PIECE => "STRING_PIECE",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldOptions_JSType(i32);
impl FieldOptions_JSType {
  /// Use the default type.
  pub const JS_NORMAL: FieldOptions_JSType = FieldOptions_JSType(0);
  /// Use JavaScript strings.
  pub const JS_STRING: FieldOptions_JSType = FieldOptions_JSType(1);
  /// Use JavaScript numbers.
  pub const JS_NUMBER: FieldOptions_JSType = FieldOptions_JSType(2);
  pub const KNOWN_VARIANTS: [FieldOptions_JSType; 3] = [FieldOptions_JSType::JS_NORMAL, FieldOptions_JSType::JS_STRING, FieldOptions_JSType::JS_NUMBER];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<FieldOptions_JSType_Closed> {
    match self {
      FieldOptions_JSType::JS_NORMAL => Some(FieldOptions_JSType_Closed::JS_NORMAL),
      FieldOptions_JSType::JS_STRING => Some(FieldOptions_JSType_Closed::JS_STRING),
      FieldOptions_JSType::JS_NUMBER => Some(FieldOptions_JSType_Closed::JS_NUMBER),
      _ => None,
    }
  }
}
impl ::std::default::Default for FieldOptions_JSType {
  fn default() -> Self {
    FieldOptions_JSType::JS_NORMAL
  }
}
impl From<FieldOptions_JSType> for i32 {
  fn from(v: FieldOptions_JSType) -> i32 {
    v.0
  }
}
impl From<i32> for FieldOptions_JSType {
  fn from(v: i32) -> FieldOptions_JSType {
    FieldOptions_JSType(v)
  }
}
impl From<FieldOptions_JSType_Closed> for FieldOptions_JSType {
  fn from(v: FieldOptions_JSType_Closed) -> FieldOptions_JSType {
    FieldOptions_JSType(v as i32)
  }
}
impl ::pb::ProtoEnum for FieldOptions_JSType {
}
impl ::pb::OpenProtoEnum for FieldOptions_JSType {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      FieldOptions_JSType::JS_NORMAL => Some("JS_NORMAL"),
      FieldOptions_JSType::JS_STRING => Some("JS_STRING"),
      FieldOptions_JSType::JS_NUMBER => Some("JS_NUMBER"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      FieldOptions_JSType::JS_NORMAL => true,
      FieldOptions_JSType::JS_STRING => true,
      FieldOptions_JSType::JS_NUMBER => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for FieldOptions_JSType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FieldOptions_JSType_Closed {
  /// Use the default type.
  JS_NORMAL = 0,
  /// Use JavaScript strings.
  JS_STRING = 1,
  /// Use JavaScript numbers.
  JS_NUMBER = 2,
}
impl FieldOptions_JSType_Closed {
  pub const KNOWN_VARIANTS: [FieldOptions_JSType_Closed; 3] = [FieldOptions_JSType_Closed::JS_NORMAL, FieldOptions_JSType_Closed::JS_STRING, FieldOptions_JSType_Closed::JS_NUMBER];
}
impl ::std::default::Default for FieldOptions_JSType_Closed {
  fn default() -> Self {
    FieldOptions_JSType_Closed::JS_NORMAL
  }
}
impl From<FieldOptions_JSType_Closed> for i32 {
  fn from(v: FieldOptions_JSType_Closed) -> i32 {
    match v {
      FieldOptions_JSType_Closed::JS_NORMAL => 0,
      FieldOptions_JSType_Closed::JS_STRING => 1,
      FieldOptions_JSType_Closed::JS_NUMBER => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FieldOptions_JSType_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FieldOptions_JSType_Closed::JS_NORMAL),
      1 => Ok(FieldOptions_JSType_Closed::JS_STRING),
      2 => Ok(FieldOptions_JSType_Closed::JS_NUMBER),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for FieldOptions_JSType_Closed {
}
impl ::pb::ClosedProtoEnum for FieldOptions_JSType_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldOptions_JSType_Closed::JS_NORMAL => "JS_NORMAL",
      FieldOptions_JSType_Closed::JS_STRING => "JS_STRING",
      FieldOptions_JSType_Closed::JS_NUMBER => "JS_NUMBER",
    }
  }
}

/// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
/// or neither? HTTP based RPC implementation may choose GET verb for safe
/// methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct MethodOptions_IdempotencyLevel(i32);
impl MethodOptions_IdempotencyLevel {
  pub const IDEMPOTENCY_UNKNOWN: MethodOptions_IdempotencyLevel = MethodOptions_IdempotencyLevel(0);
  /// implies idempotent
  pub const NO_SIDE_EFFECTS: MethodOptions_IdempotencyLevel = MethodOptions_IdempotencyLevel(1);
  /// idempotent, but may have side effects
  pub const IDEMPOTENT: MethodOptions_IdempotencyLevel = MethodOptions_IdempotencyLevel(2);
  pub const KNOWN_VARIANTS: [MethodOptions_IdempotencyLevel; 3] = [MethodOptions_IdempotencyLevel::IDEMPOTENCY_UNKNOWN, MethodOptions_IdempotencyLevel::NO_SIDE_EFFECTS, MethodOptions_IdempotencyLevel::IDEMPOTENT];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<MethodOptions_IdempotencyLevel_Closed> {
    match self {
      MethodOptions_IdempotencyLevel::IDEMPOTENCY_UNKNOWN => Some(MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN),
      MethodOptions_IdempotencyLevel::NO_SIDE_EFFECTS => Some(MethodOptions_IdempotencyLevel_Closed::NO_SIDE_EFFECTS),
      MethodOptions_IdempotencyLevel::IDEMPOTENT => Some(MethodOptions_IdempotencyLevel_Closed::IDEMPOTENT),
      _ => None,
    }
  }
}
impl ::std::default::Default for MethodOptions_IdempotencyLevel {
  fn default() -> Self {
    MethodOptions_IdempotencyLevel::IDEMPOTENCY_UNKNOWN
  }
}
impl From<MethodOptions_IdempotencyLevel> for i32 {
  fn from(v: MethodOptions_IdempotencyLevel) -> i32 {
    v.0
  }
}
impl From<i32> for MethodOptions_IdempotencyLevel {
  fn from(v: i32) -> MethodOptions_IdempotencyLevel {
    MethodOptions_IdempotencyLevel(v)
  }
}
impl From<MethodOptions_IdempotencyLevel_Closed> for MethodOptions_IdempotencyLevel {
  fn from(v: MethodOptions_IdempotencyLevel_Closed) -> MethodOptions_IdempotencyLevel {
    MethodOptions_IdempotencyLevel(v as i32)
  }
}
impl ::pb::ProtoEnum for MethodOptions_IdempotencyLevel {
}
impl ::pb::OpenProtoEnum for MethodOptions_IdempotencyLevel {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      MethodOptions_IdempotencyLevel::IDEMPOTENCY_UNKNOWN => Some("IDEMPOTENCY_UNKNOWN"),
      MethodOptions_IdempotencyLevel::NO_SIDE_EFFECTS => Some("NO_SIDE_EFFECTS"),
      MethodOptions_IdempotencyLevel::IDEMPOTENT => Some("IDEMPOTENT"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      MethodOptions_IdempotencyLevel::IDEMPOTENCY_UNKNOWN => true,
      MethodOptions_IdempotencyLevel::NO_SIDE_EFFECTS => true,
      MethodOptions_IdempotencyLevel::IDEMPOTENT => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for MethodOptions_IdempotencyLevel {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// Is this method side-effect-free (or safe in HTTP parlance), or idempotent,
/// or neither? HTTP based RPC implementation may choose GET verb for safe
/// methods, and PUT verb for idempotent methods instead of the default POST.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum MethodOptions_IdempotencyLevel_Closed {
  IDEMPOTENCY_UNKNOWN = 0,
  /// implies idempotent
  NO_SIDE_EFFECTS = 1,
  /// idempotent, but may have side effects
  IDEMPOTENT = 2,
}
impl MethodOptions_IdempotencyLevel_Closed {
  pub const KNOWN_VARIANTS: [MethodOptions_IdempotencyLevel_Closed; 3] = [MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN, MethodOptions_IdempotencyLevel_Closed::NO_SIDE_EFFECTS, MethodOptions_IdempotencyLevel_Closed::IDEMPOTENT];
}
impl ::std::default::Default for MethodOptions_IdempotencyLevel_Closed {
  fn default() -> Self {
    MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN
  }
}
impl From<MethodOptions_IdempotencyLevel_Closed> for i32 {
  fn from(v: MethodOptions_IdempotencyLevel_Closed) -> i32 {
    match v {
      MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN => 0,
      MethodOptions_IdempotencyLevel_Closed::NO_SIDE_EFFECTS => 1,
      MethodOptions_IdempotencyLevel_Closed::IDEMPOTENT => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for MethodOptions_IdempotencyLevel_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN),
      1 => Ok(MethodOptions_IdempotencyLevel_Closed::NO_SIDE_EFFECTS),
      2 => Ok(MethodOptions_IdempotencyLevel_Closed::IDEMPOTENT),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for MethodOptions_IdempotencyLevel_Closed {
}
impl ::pb::ClosedProtoEnum for MethodOptions_IdempotencyLevel_Closed {
  fn name(self) -> &'static str {
    match self {
      MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN => "IDEMPOTENCY_UNKNOWN",
      MethodOptions_IdempotencyLevel_Closed::NO_SIDE_EFFECTS => "NO_SIDE_EFFECTS",
      MethodOptions_IdempotencyLevel_Closed::IDEMPOTENT => "IDEMPOTENT",
    }
  }
}

/// The protocol compiler can output a FileDescriptorSet containing the .proto
/// files it parses.
#[derive(Clone, Debug, PartialEq)]
pub struct FileDescriptorSet {
  pub file: ::std::vec::Vec<FileDescriptorProto>,
}
impl FileDescriptorSet {
  pub fn set_file(&mut self, v: ::std::vec::Vec<FileDescriptorProto>) {
    self.file = v;
  }
  pub fn take_file(&mut self) -> ::std::vec::Vec<FileDescriptorProto> {
    ::std::mem::replace(&mut self.file, ::std::vec::Vec::new())
  }
  pub fn get_file(&self) -> &[FileDescriptorProto] {
    &self.file
  }
  pub fn mut_file(&mut self) -> &mut ::std::vec::Vec<FileDescriptorProto> {
    &mut self.file
  }
}
impl ::std::default::Default for FileDescriptorSet {
  fn default() -> Self {
    FileDescriptorSet {
      file: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FileDescriptorSet_default: FileDescriptorSet = FileDescriptorSet::default();
}
impl ::pb::Message for FileDescriptorSet {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut file_size = 0;
    for val in &self.file {
      let l = ::pb::Message::compute_size(val);
      file_size += ::pb::wire_format::serialized_length(1);
      file_size += ::pb::varint::serialized_length(l as u64);
      file_size += l;
    }
    size += file_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.file {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.file {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorSet", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: FileDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.file.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for FileDescriptorSet {
  const NAME: &'static str = "FileDescriptorSet";
  const FULL_NAME: &'static str = "google.protobuf.FileDescriptorSet";
}

/// Describes a complete .proto file.
#[derive(Clone, Debug, PartialEq)]
pub struct FileDescriptorProto {
  /// file name, relative to root of source tree
  pub name: ::std::option::Option<::std::string::String>,
  /// e.g. "foo", "foo.bar", etc.
  pub package: ::std::option::Option<::std::string::String>,
  /// Names of files imported by this file.
  pub dependency: ::std::vec::Vec<::std::string::String>,
  /// Indexes of the public imported files in the dependency list above.
  pub public_dependency: ::std::vec::Vec<i32>,
  /// Indexes of the weak imported files in the dependency list.
  /// For Google-internal migration only. Do not use.
  pub weak_dependency: ::std::vec::Vec<i32>,
  /// All top-level definitions in this file.
  pub message_type: ::std::vec::Vec<DescriptorProto>,
  pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,
  pub service: ::std::vec::Vec<ServiceDescriptorProto>,
  pub extension: ::std::vec::Vec<FieldDescriptorProto>,
  pub options: ::std::option::Option<FileOptions>,
  /// This field contains optional information about the original source code.
  /// You may safely remove this entire field without harming runtime
  /// functionality of the descriptors -- the information is needed only by
  /// development tools.
  pub source_code_info: ::std::option::Option<SourceCodeInfo>,
  /// The syntax of the proto file.
  /// The supported values are "proto2" and "proto3".
  pub syntax: ::std::option::Option<::std::string::String>,
}
impl FileDescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_package(&self) -> bool {
    self.package.is_some()
  }
  pub fn set_package(&mut self, v: ::std::string::String) {
    self.package = Some(v);
  }
  pub fn take_package(&mut self) -> ::std::string::String {
    self.package.take().unwrap_or_default()
  }
  pub fn get_package(&self) -> &str {
    self.package.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn set_dependency(&mut self, v: ::std::vec::Vec<::std::string::String>) {
    self.dependency = v;
  }
  pub fn take_dependency(&mut self) -> ::std::vec::Vec<::std::string::String> {
    ::std::mem::replace(&mut self.dependency, ::std::vec::Vec::new())
  }
  pub fn get_dependency(&self) -> &[::std::string::String] {
    &self.dependency
  }
  pub fn mut_dependency(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
    &mut self.dependency
  }
  pub fn set_public_dependency(&mut self, v: ::std::vec::Vec<i32>) {
    self.public_dependency = v;
  }
  pub fn take_public_dependency(&mut self) -> ::std::vec::Vec<i32> {
    ::std::mem::replace(&mut self.public_dependency, ::std::vec::Vec::new())
  }
  pub fn get_public_dependency(&self) -> &[i32] {
    &self.public_dependency
  }
  pub fn mut_public_dependency(&mut self) -> &mut ::std::vec::Vec<i32> {
    &mut self.public_dependency
  }
  pub fn set_weak_dependency(&mut self, v: ::std::vec::Vec<i32>) {
    self.weak_dependency = v;
  }
  pub fn take_weak_dependency(&mut self) -> ::std::vec::Vec<i32> {
    ::std::mem::replace(&mut self.weak_dependency, ::std::vec::Vec::new())
  }
  pub fn get_weak_dependency(&self) -> &[i32] {
    &self.weak_dependency
  }
  pub fn mut_weak_dependency(&mut self) -> &mut ::std::vec::Vec<i32> {
    &mut self.weak_dependency
  }
  pub fn set_message_type(&mut self, v: ::std::vec::Vec<DescriptorProto>) {
    self.message_type = v;
  }
  pub fn take_message_type(&mut self) -> ::std::vec::Vec<DescriptorProto> {
    ::std::mem::replace(&mut self.message_type, ::std::vec::Vec::new())
  }
  pub fn get_message_type(&self) -> &[DescriptorProto] {
    &self.message_type
  }
  pub fn mut_message_type(&mut self) -> &mut ::std::vec::Vec<DescriptorProto> {
    &mut self.message_type
  }
  pub fn set_enum_type(&mut self, v: ::std::vec::Vec<EnumDescriptorProto>) {
    self.enum_type = v;
  }
  pub fn take_enum_type(&mut self) -> ::std::vec::Vec<EnumDescriptorProto> {
    ::std::mem::replace(&mut self.enum_type, ::std::vec::Vec::new())
  }
  pub fn get_enum_type(&self) -> &[EnumDescriptorProto] {
    &self.enum_type
  }
  pub fn mut_enum_type(&mut self) -> &mut ::std::vec::Vec<EnumDescriptorProto> {
    &mut self.enum_type
  }
  pub fn set_service(&mut self, v: ::std::vec::Vec<ServiceDescriptorProto>) {
    self.service = v;
  }
  pub fn take_service(&mut self) -> ::std::vec::Vec<ServiceDescriptorProto> {
    ::std::mem::replace(&mut self.service, ::std::vec::Vec::new())
  }
  pub fn get_service(&self) -> &[ServiceDescriptorProto] {
    &self.service
  }
  pub fn mut_service(&mut self) -> &mut ::std::vec::Vec<ServiceDescriptorProto> {
    &mut self.service
  }
  pub fn set_extension(&mut self, v: ::std::vec::Vec<FieldDescriptorProto>) {
    self.extension = v;
  }
  pub fn take_extension(&mut self) -> ::std::vec::Vec<FieldDescriptorProto> {
    ::std::mem::replace(&mut self.extension, ::std::vec::Vec::new())
  }
  pub fn get_extension(&self) -> &[FieldDescriptorProto] {
    &self.extension
  }
  pub fn mut_extension(&mut self) -> &mut ::std::vec::Vec<FieldDescriptorProto> {
    &mut self.extension
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: FileOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> FileOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &FileOptions {
    self.options.as_ref().unwrap_or(&FileOptions_default)
  }
  pub fn has_source_code_info(&self) -> bool {
    self.source_code_info.is_some()
  }
  pub fn set_source_code_info(&mut self, v: SourceCodeInfo) {
    self.source_code_info = Some(v);
  }
  pub fn take_source_code_info(&mut self) -> SourceCodeInfo {
    self.source_code_info.take().unwrap_or_default()
  }
  pub fn get_source_code_info(&self) -> &SourceCodeInfo {
    self.source_code_info.as_ref().unwrap_or(&SourceCodeInfo_default)
  }
  pub fn has_syntax(&self) -> bool {
    self.syntax.is_some()
  }
  pub fn set_syntax(&mut self, v: ::std::string::String) {
    self.syntax = Some(v);
  }
  pub fn take_syntax(&mut self) -> ::std::string::String {
    self.syntax.take().unwrap_or_default()
  }
  pub fn get_syntax(&self) -> &str {
    self.syntax.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
}
impl ::std::default::Default for FileDescriptorProto {
  fn default() -> Self {
    FileDescriptorProto {
      name: ::std::default::Default::default(),
      package: ::std::default::Default::default(),
      dependency: ::std::default::Default::default(),
      public_dependency: ::std::default::Default::default(),
      weak_dependency: ::std::default::Default::default(),
      message_type: ::std::default::Default::default(),
      enum_type: ::std::default::Default::default(),
      service: ::std::default::Default::default(),
      extension: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      source_code_info: ::std::default::Default::default(),
      syntax: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FileDescriptorProto_default: FileDescriptorProto = FileDescriptorProto::default();
}
impl ::pb::Message for FileDescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut package_size = 0;
    for val in &self.package {
      let l = ::pb::Message::compute_size(val);
      package_size += ::pb::wire_format::serialized_length(2);
      package_size += ::pb::varint::serialized_length(l as u64);
      package_size += l;
    }
    size += package_size;
    let mut dependency_size = 0;
    for val in &self.dependency {
      let l = ::pb::Message::compute_size(val);
      dependency_size += ::pb::wire_format::serialized_length(3);
      dependency_size += ::pb::varint::serialized_length(l as u64);
      dependency_size += l;
    }
    size += dependency_size;
    let mut public_dependency_size = 0;
    for val in &self.public_dependency {
      let l = ::pb::Message::compute_size(val);
      public_dependency_size += ::pb::wire_format::serialized_length(10);
      public_dependency_size += l;
    }
    size += public_dependency_size;
    let mut weak_dependency_size = 0;
    for val in &self.weak_dependency {
      let l = ::pb::Message::compute_size(val);
      weak_dependency_size += ::pb::wire_format::serialized_length(11);
      weak_dependency_size += l;
    }
    size += weak_dependency_size;
    let mut message_type_size = 0;
    for val in &self.message_type {
      let l = ::pb::Message::compute_size(val);
      message_type_size += ::pb::wire_format::serialized_length(4);
      message_type_size += ::pb::varint::serialized_length(l as u64);
      message_type_size += l;
    }
    size += message_type_size;
    let mut enum_type_size = 0;
    for val in &self.enum_type {
      let l = ::pb::Message::compute_size(val);
      enum_type_size += ::pb::wire_format::serialized_length(5);
      enum_type_size += ::pb::varint::serialized_length(l as u64);
      enum_type_size += l;
    }
    size += enum_type_size;
    let mut service_size = 0;
    for val in &self.service {
      let l = ::pb::Message::compute_size(val);
      service_size += ::pb::wire_format::serialized_length(6);
      service_size += ::pb::varint::serialized_length(l as u64);
      service_size += l;
    }
    size += service_size;
    let mut extension_size = 0;
    for val in &self.extension {
      let l = ::pb::Message::compute_size(val);
      extension_size += ::pb::wire_format::serialized_length(7);
      extension_size += ::pb::varint::serialized_length(l as u64);
      extension_size += l;
    }
    size += extension_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(8);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut source_code_info_size = 0;
    for val in &self.source_code_info {
      let l = ::pb::Message::compute_size(val);
      source_code_info_size += ::pb::wire_format::serialized_length(9);
      source_code_info_size += ::pb::varint::serialized_length(l as u64);
      source_code_info_size += l;
    }
    size += source_code_info_size;
    let mut syntax_size = 0;
    for val in &self.syntax {
      let l = ::pb::Message::compute_size(val);
      syntax_size += ::pb::wire_format::serialized_length(12);
      syntax_size += ::pb::varint::serialized_length(l as u64);
      syntax_size += l;
    }
    size += syntax_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.package {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.dependency {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.public_dependency {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.weak_dependency {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.message_type {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.enum_type {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.service {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.extension {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.source_code_info {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.syntax {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.package {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.dependency {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.message_type {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.enum_type {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.service {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.extension {
      ::pb::wire_format::write(7, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
      ::pb::wire_format::write(8, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.source_code_info {
      ::pb::wire_format::write(9, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.public_dependency {
      ::pb::wire_format::write(10, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.weak_dependency {
      ::pb::wire_format::write(11, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.syntax {
      ::pb::wire_format::write(12, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.package = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.dependency.push(val);
        }
        10 => {
          match typ {
            ::pb::wire_format::Type::LengthDelimited => {
              let len = ::pb::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: i32 = ::std::default::Default::default();
                ::pb::Message::deserialize(&mut val, &mut vals)?;
                self.public_dependency.push(val);
              }
            }
            _ => {
              ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileDescriptorProto", 10)?;
              let mut val: i32 = ::std::default::Default::default();
              ::pb::Message::deserialize(&mut val, buf)?;
              self.public_dependency.push(val);
            }
          }
        }
        11 => {
          match typ {
            ::pb::wire_format::Type::LengthDelimited => {
              let len = ::pb::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: i32 = ::std::default::Default::default();
                ::pb::Message::deserialize(&mut val, &mut vals)?;
                self.weak_dependency.push(val);
              }
            }
            _ => {
              ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileDescriptorProto", 11)?;
              let mut val: i32 = ::std::default::Default::default();
              ::pb::Message::deserialize(&mut val, buf)?;
              self.weak_dependency.push(val);
            }
          }
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: DescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.message_type.push(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 5)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: EnumDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.enum_type.push(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ServiceDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.service.push(val);
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 7)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: FieldDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.extension.push(val);
        }
        8 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 8)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: FileOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        9 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 9)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: SourceCodeInfo = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.source_code_info = Some(val);
        }
        12 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileDescriptorProto", 12)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.syntax = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for FileDescriptorProto {
  const NAME: &'static str = "FileDescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.FileDescriptorProto";
}

/// Describes a message type.
#[derive(Clone, Debug, PartialEq)]
pub struct DescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  pub field: ::std::vec::Vec<FieldDescriptorProto>,
  pub extension: ::std::vec::Vec<FieldDescriptorProto>,
  pub nested_type: ::std::vec::Vec<DescriptorProto>,
  pub enum_type: ::std::vec::Vec<EnumDescriptorProto>,
  pub extension_range: ::std::vec::Vec<DescriptorProto_ExtensionRange>,
  pub oneof_decl: ::std::vec::Vec<OneofDescriptorProto>,
  pub options: ::std::option::Option<MessageOptions>,
  pub reserved_range: ::std::vec::Vec<DescriptorProto_ReservedRange>,
  /// Reserved field names, which may not be used by fields in the same message.
  /// A given name may only be reserved once.
  pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
impl DescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn set_field(&mut self, v: ::std::vec::Vec<FieldDescriptorProto>) {
    self.field = v;
  }
  pub fn take_field(&mut self) -> ::std::vec::Vec<FieldDescriptorProto> {
    ::std::mem::replace(&mut self.field, ::std::vec::Vec::new())
  }
  pub fn get_field(&self) -> &[FieldDescriptorProto] {
    &self.field
  }
  pub fn mut_field(&mut self) -> &mut ::std::vec::Vec<FieldDescriptorProto> {
    &mut self.field
  }
  pub fn set_extension(&mut self, v: ::std::vec::Vec<FieldDescriptorProto>) {
    self.extension = v;
  }
  pub fn take_extension(&mut self) -> ::std::vec::Vec<FieldDescriptorProto> {
    ::std::mem::replace(&mut self.extension, ::std::vec::Vec::new())
  }
  pub fn get_extension(&self) -> &[FieldDescriptorProto] {
    &self.extension
  }
  pub fn mut_extension(&mut self) -> &mut ::std::vec::Vec<FieldDescriptorProto> {
    &mut self.extension
  }
  pub fn set_nested_type(&mut self, v: ::std::vec::Vec<DescriptorProto>) {
    self.nested_type = v;
  }
  pub fn take_nested_type(&mut self) -> ::std::vec::Vec<DescriptorProto> {
    ::std::mem::replace(&mut self.nested_type, ::std::vec::Vec::new())
  }
  pub fn get_nested_type(&self) -> &[DescriptorProto] {
    &self.nested_type
  }
  pub fn mut_nested_type(&mut self) -> &mut ::std::vec::Vec<DescriptorProto> {
    &mut self.nested_type
  }
  pub fn set_enum_type(&mut self, v: ::std::vec::Vec<EnumDescriptorProto>) {
    self.enum_type = v;
  }
  pub fn take_enum_type(&mut self) -> ::std::vec::Vec<EnumDescriptorProto> {
    ::std::mem::replace(&mut self.enum_type, ::std::vec::Vec::new())
  }
  pub fn get_enum_type(&self) -> &[EnumDescriptorProto] {
    &self.enum_type
  }
  pub fn mut_enum_type(&mut self) -> &mut ::std::vec::Vec<EnumDescriptorProto> {
    &mut self.enum_type
  }
  pub fn set_extension_range(&mut self, v: ::std::vec::Vec<DescriptorProto_ExtensionRange>) {
    self.extension_range = v;
  }
  pub fn take_extension_range(&mut self) -> ::std::vec::Vec<DescriptorProto_ExtensionRange> {
    ::std::mem::replace(&mut self.extension_range, ::std::vec::Vec::new())
  }
  pub fn get_extension_range(&self) -> &[DescriptorProto_ExtensionRange] {
    &self.extension_range
  }
  pub fn mut_extension_range(&mut self) -> &mut ::std::vec::Vec<DescriptorProto_ExtensionRange> {
    &mut self.extension_range
  }
  pub fn set_oneof_decl(&mut self, v: ::std::vec::Vec<OneofDescriptorProto>) {
    self.oneof_decl = v;
  }
  pub fn take_oneof_decl(&mut self) -> ::std::vec::Vec<OneofDescriptorProto> {
    ::std::mem::replace(&mut self.oneof_decl, ::std::vec::Vec::new())
  }
  pub fn get_oneof_decl(&self) -> &[OneofDescriptorProto] {
    &self.oneof_decl
  }
  pub fn mut_oneof_decl(&mut self) -> &mut ::std::vec::Vec<OneofDescriptorProto> {
    &mut self.oneof_decl
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: MessageOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> MessageOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &MessageOptions {
    self.options.as_ref().unwrap_or(&MessageOptions_default)
  }
  pub fn set_reserved_range(&mut self, v: ::std::vec::Vec<DescriptorProto_ReservedRange>) {
    self.reserved_range = v;
  }
  pub fn take_reserved_range(&mut self) -> ::std::vec::Vec<DescriptorProto_ReservedRange> {
    ::std::mem::replace(&mut self.reserved_range, ::std::vec::Vec::new())
  }
  pub fn get_reserved_range(&self) -> &[DescriptorProto_ReservedRange] {
    &self.reserved_range
  }
  pub fn mut_reserved_range(&mut self) -> &mut ::std::vec::Vec<DescriptorProto_ReservedRange> {
    &mut self.reserved_range
  }
  pub fn set_reserved_name(&mut self, v: ::std::vec::Vec<::std::string::String>) {
    self.reserved_name = v;
  }
  pub fn take_reserved_name(&mut self) -> ::std::vec::Vec<::std::string::String> {
    ::std::mem::replace(&mut self.reserved_name, ::std::vec::Vec::new())
  }
  pub fn get_reserved_name(&self) -> &[::std::string::String] {
    &self.reserved_name
  }
  pub fn mut_reserved_name(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
    &mut self.reserved_name
  }
}
impl ::std::default::Default for DescriptorProto {
  fn default() -> Self {
    DescriptorProto {
      name: ::std::default::Default::default(),
      field: ::std::default::Default::default(),
      extension: ::std::default::Default::default(),
      nested_type: ::std::default::Default::default(),
      enum_type: ::std::default::Default::default(),
      extension_range: ::std::default::Default::default(),
      oneof_decl: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      reserved_range: ::std::default::Default::default(),
      reserved_name: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref DescriptorProto_default: DescriptorProto = DescriptorProto::default();
}
impl ::pb::Message for DescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut field_size = 0;
    for val in &self.field {
      let l = ::pb::Message::compute_size(val);
      field_size += ::pb::wire_format::serialized_length(2);
      field_size += ::pb::varint::serialized_length(l as u64);
      field_size += l;
    }
    size += field_size;
    let mut extension_size = 0;
    for val in &self.extension {
      let l = ::pb::Message::compute_size(val);
      extension_size += ::pb::wire_format::serialized_length(6);
      extension_size += ::pb::varint::serialized_length(l as u64);
      extension_size += l;
    }
    size += extension_size;
    let mut nested_type_size = 0;
    for val in &self.nested_type {
      let l = ::pb::Message::compute_size(val);
      nested_type_size += ::pb::wire_format::serialized_length(3);
      nested_type_size += ::pb::varint::serialized_length(l as u64);
      nested_type_size += l;
    }
    size += nested_type_size;
    let mut enum_type_size = 0;
    for val in &self.enum_type {
      let l = ::pb::Message::compute_size(val);
      enum_type_size += ::pb::wire_format::serialized_length(4);
      enum_type_size += ::pb::varint::serialized_length(l as u64);
      enum_type_size += l;
    }
    size += enum_type_size;
    let mut extension_range_size = 0;
    for val in &self.extension_range {
      let l = ::pb::Message::compute_size(val);
      extension_range_size += ::pb::wire_format::serialized_length(5);
      extension_range_size += ::pb::varint::serialized_length(l as u64);
      extension_range_size += l;
    }
    size += extension_range_size;
    let mut oneof_decl_size = 0;
    for val in &self.oneof_decl {
      let l = ::pb::Message::compute_size(val);
      oneof_decl_size += ::pb::wire_format::serialized_length(8);
      oneof_decl_size += ::pb::varint::serialized_length(l as u64);
      oneof_decl_size += l;
    }
    size += oneof_decl_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(7);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut reserved_range_size = 0;
    for val in &self.reserved_range {
      let l = ::pb::Message::compute_size(val);
      reserved_range_size += ::pb::wire_format::serialized_length(9);
      reserved_range_size += ::pb::varint::serialized_length(l as u64);
      reserved_range_size += l;
    }
    size += reserved_range_size;
    let mut reserved_name_size = 0;
    for val in &self.reserved_name {
      let l = ::pb::Message::compute_size(val);
      reserved_name_size += ::pb::wire_format::serialized_length(10);
      reserved_name_size += ::pb::varint::serialized_length(l as u64);
      reserved_name_size += l;
    }
    size += reserved_name_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.field {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.extension {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.nested_type {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.enum_type {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.extension_range {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.oneof_decl {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.reserved_range {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.reserved_name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.field {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.nested_type {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.enum_type {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.extension_range {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.extension {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
      ::pb::wire_format::write(7, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.oneof_decl {
      ::pb::wire_format::write(8, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.reserved_range {
      ::pb::wire_format::write(9, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.reserved_name {
      ::pb::wire_format::write(10, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: FieldDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.field.push(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: FieldDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.extension.push(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: DescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.nested_type.push(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: EnumDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.enum_type.push(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 5)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: DescriptorProto_ExtensionRange = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.extension_range.push(val);
        }
        8 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 8)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: OneofDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.oneof_decl.push(val);
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 7)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: MessageOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        9 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 9)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: DescriptorProto_ReservedRange = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.reserved_range.push(val);
        }
        10 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto", 10)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.reserved_name.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for DescriptorProto {
  const NAME: &'static str = "DescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.DescriptorProto";
}

#[derive(Clone, Debug, PartialEq)]
pub struct DescriptorProto_ExtensionRange {
  /// Inclusive.
  pub start: ::std::option::Option<i32>,
  /// Exclusive.
  pub end: ::std::option::Option<i32>,
  pub options: ::std::option::Option<ExtensionRangeOptions>,
}
impl DescriptorProto_ExtensionRange {
  pub fn has_start(&self) -> bool {
    self.start.is_some()
  }
  pub fn set_start(&mut self, v: i32) {
    self.start = Some(v);
  }
  pub fn get_start(&self) -> i32 {
    self.start.unwrap_or(0)
  }
  pub fn has_end(&self) -> bool {
    self.end.is_some()
  }
  pub fn set_end(&mut self, v: i32) {
    self.end = Some(v);
  }
  pub fn get_end(&self) -> i32 {
    self.end.unwrap_or(0)
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: ExtensionRangeOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> ExtensionRangeOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &ExtensionRangeOptions {
    self.options.as_ref().unwrap_or(&ExtensionRangeOptions_default)
  }
}
impl ::std::default::Default for DescriptorProto_ExtensionRange {
  fn default() -> Self {
    DescriptorProto_ExtensionRange {
      start: ::std::default::Default::default(),
      end: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref DescriptorProto_ExtensionRange_default: DescriptorProto_ExtensionRange = DescriptorProto_ExtensionRange::default();
}
impl ::pb::Message for DescriptorProto_ExtensionRange {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut start_size = 0;
    for val in &self.start {
      let l = ::pb::Message::compute_size(val);
      start_size += ::pb::wire_format::serialized_length(1);
      start_size += l;
    }
    size += start_size;
    let mut end_size = 0;
    for val in &self.end {
      let l = ::pb::Message::compute_size(val);
      end_size += ::pb::wire_format::serialized_length(2);
      end_size += l;
    }
    size += end_size;
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
    for val in &self.start {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.end {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.start {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.end {
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "DescriptorProto_ExtensionRange", 1)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.start = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "DescriptorProto_ExtensionRange", 2)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.end = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "DescriptorProto_ExtensionRange", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ExtensionRangeOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for DescriptorProto_ExtensionRange {
  const NAME: &'static str = "DescriptorProto_ExtensionRange";
  const FULL_NAME: &'static str = "google.protobuf.DescriptorProto_ExtensionRange";
}

/// Range of reserved tag numbers. Reserved tag numbers may not be used by
/// fields or extension ranges in the same message. Reserved ranges may
/// not overlap.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DescriptorProto_ReservedRange {
  /// Inclusive.
  pub start: ::std::option::Option<i32>,
  /// Exclusive.
  pub end: ::std::option::Option<i32>,
}
impl DescriptorProto_ReservedRange {
  pub fn has_start(&self) -> bool {
    self.start.is_some()
  }
  pub fn set_start(&mut self, v: i32) {
    self.start = Some(v);
  }
  pub fn get_start(&self) -> i32 {
    self.start.unwrap_or(0)
  }
  pub fn has_end(&self) -> bool {
    self.end.is_some()
  }
  pub fn set_end(&mut self, v: i32) {
    self.end = Some(v);
  }
  pub fn get_end(&self) -> i32 {
    self.end.unwrap_or(0)
  }
}
impl ::std::default::Default for DescriptorProto_ReservedRange {
  fn default() -> Self {
    DescriptorProto_ReservedRange {
      start: ::std::default::Default::default(),
      end: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref DescriptorProto_ReservedRange_default: DescriptorProto_ReservedRange = DescriptorProto_ReservedRange::default();
}
impl ::pb::Message for DescriptorProto_ReservedRange {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut start_size = 0;
    for val in &self.start {
      let l = ::pb::Message::compute_size(val);
      start_size += ::pb::wire_format::serialized_length(1);
      start_size += l;
    }
    size += start_size;
    let mut end_size = 0;
    for val in &self.end {
      let l = ::pb::Message::compute_size(val);
      end_size += ::pb::wire_format::serialized_length(2);
      end_size += l;
    }
    size += end_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.start {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.end {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.start {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.end {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "DescriptorProto_ReservedRange", 1)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.start = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "DescriptorProto_ReservedRange", 2)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.end = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for DescriptorProto_ReservedRange {
  const NAME: &'static str = "DescriptorProto_ReservedRange";
  const FULL_NAME: &'static str = "google.protobuf.DescriptorProto_ReservedRange";
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionRangeOptions {
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl ExtensionRangeOptions {
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for ExtensionRangeOptions {
  fn default() -> Self {
    ExtensionRangeOptions {
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ExtensionRangeOptions_default: ExtensionRangeOptions = ExtensionRangeOptions::default();
}
impl ::pb::Message for ExtensionRangeOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "ExtensionRangeOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for ExtensionRangeOptions {
  const NAME: &'static str = "ExtensionRangeOptions";
  const FULL_NAME: &'static str = "google.protobuf.ExtensionRangeOptions";
}

/// Describes a field within a message.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldDescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  pub number: ::std::option::Option<i32>,
  pub label: ::std::option::Option<FieldDescriptorProto_Label>,
  /// If type_name is set, this need not be set.  If both this and type_name
  /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
  pub type_: ::std::option::Option<FieldDescriptorProto_Type>,
  /// For message and enum types, this is the name of the type.  If the name
  /// starts with a '.', it is fully-qualified.  Otherwise, C++-like scoping
  /// rules are used to find the type (i.e. first the nested types within this
  /// message are searched, then within the parent, on up to the root
  /// namespace).
  pub type_name: ::std::option::Option<::std::string::String>,
  /// For extensions, this is the name of the type being extended.  It is
  /// resolved in the same manner as type_name.
  pub extendee: ::std::option::Option<::std::string::String>,
  /// For numeric types, contains the original text representation of the value.
  /// For booleans, "true" or "false".
  /// For strings, contains the default text contents (not escaped in any way).
  /// For bytes, contains the C escaped value.  All bytes >= 128 are escaped.
  /// TODO(kenton):  Base-64 encode?
  pub default_value: ::std::option::Option<::std::string::String>,
  /// If set, gives the index of a oneof in the containing type's oneof_decl
  /// list.  This field is a member of that oneof.
  pub oneof_index: ::std::option::Option<i32>,
  /// JSON name of this field. The value is set by protocol compiler. If the
  /// user has set a "json_name" option on this field, that option's value
  /// will be used. Otherwise, it's deduced from the field's name by converting
  /// it to camelCase.
  pub json_name: ::std::option::Option<::std::string::String>,
  pub options: ::std::option::Option<FieldOptions>,
  /// If true, this is a proto3 "optional". When a proto3 field is optional, it
  /// tracks presence regardless of field type.

  /// When proto3_optional is true, this field must be belong to a oneof to
  /// signal to old proto3 clients that presence is tracked for this field. This
  /// oneof is known as a "synthetic" oneof, and this field must be its sole
  /// member (each proto3 optional field gets its own synthetic oneof). Synthetic
  /// oneofs exist in the descriptor only, and do not generate any API. Synthetic
  /// oneofs must be ordered after all "real" oneofs.

  /// For message fields, proto3_optional doesn't create any semantic change,
  /// since non-repeated message fields always track presence. However it still
  /// indicates the semantic detail of whether the user wrote "optional" or not.
  /// This can be useful for round-tripping the .proto file. For consistency we
  /// give message fields a synthetic oneof also, even though it is not required
  /// to track presence. This is especially important because the parser can't
  /// tell if a field is a message or an enum, so it must always create a
  /// synthetic oneof.

  /// Proto2 optional fields do not set this flag, because they already indicate
  /// optional with `LABEL_OPTIONAL`.
  pub proto3_optional: ::std::option::Option<bool>,
}
impl FieldDescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_number(&self) -> bool {
    self.number.is_some()
  }
  pub fn set_number(&mut self, v: i32) {
    self.number = Some(v);
  }
  pub fn get_number(&self) -> i32 {
    self.number.unwrap_or(0)
  }
  pub fn has_label(&self) -> bool {
    self.label.is_some()
  }
  pub fn set_label(&mut self, v: FieldDescriptorProto_Label) {
    self.label = Some(v);
  }
  pub fn get_label(&self) -> FieldDescriptorProto_Label {
    self.label.unwrap_or_default()
  }
  pub fn has_type_(&self) -> bool {
    self.type_.is_some()
  }
  pub fn set_type_(&mut self, v: FieldDescriptorProto_Type) {
    self.type_ = Some(v);
  }
  pub fn get_type_(&self) -> FieldDescriptorProto_Type {
    self.type_.unwrap_or_default()
  }
  pub fn has_type_name(&self) -> bool {
    self.type_name.is_some()
  }
  pub fn set_type_name(&mut self, v: ::std::string::String) {
    self.type_name = Some(v);
  }
  pub fn take_type_name(&mut self) -> ::std::string::String {
    self.type_name.take().unwrap_or_default()
  }
  pub fn get_type_name(&self) -> &str {
    self.type_name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_extendee(&self) -> bool {
    self.extendee.is_some()
  }
  pub fn set_extendee(&mut self, v: ::std::string::String) {
    self.extendee = Some(v);
  }
  pub fn take_extendee(&mut self) -> ::std::string::String {
    self.extendee.take().unwrap_or_default()
  }
  pub fn get_extendee(&self) -> &str {
    self.extendee.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_default_value(&self) -> bool {
    self.default_value.is_some()
  }
  pub fn set_default_value(&mut self, v: ::std::string::String) {
    self.default_value = Some(v);
  }
  pub fn take_default_value(&mut self) -> ::std::string::String {
    self.default_value.take().unwrap_or_default()
  }
  pub fn get_default_value(&self) -> &str {
    self.default_value.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_oneof_index(&self) -> bool {
    self.oneof_index.is_some()
  }
  pub fn set_oneof_index(&mut self, v: i32) {
    self.oneof_index = Some(v);
  }
  pub fn get_oneof_index(&self) -> i32 {
    self.oneof_index.unwrap_or(0)
  }
  pub fn has_json_name(&self) -> bool {
    self.json_name.is_some()
  }
  pub fn set_json_name(&mut self, v: ::std::string::String) {
    self.json_name = Some(v);
  }
  pub fn take_json_name(&mut self) -> ::std::string::String {
    self.json_name.take().unwrap_or_default()
  }
  pub fn get_json_name(&self) -> &str {
    self.json_name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: FieldOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> FieldOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &FieldOptions {
    self.options.as_ref().unwrap_or(&FieldOptions_default)
  }
  pub fn has_proto3_optional(&self) -> bool {
    self.proto3_optional.is_some()
  }
  pub fn set_proto3_optional(&mut self, v: bool) {
    self.proto3_optional = Some(v);
  }
  pub fn get_proto3_optional(&self) -> bool {
    self.proto3_optional.unwrap_or(false)
  }
}
impl ::std::default::Default for FieldDescriptorProto {
  fn default() -> Self {
    FieldDescriptorProto {
      name: ::std::default::Default::default(),
      number: ::std::default::Default::default(),
      label: ::std::default::Default::default(),
      type_: ::std::default::Default::default(),
      type_name: ::std::default::Default::default(),
      extendee: ::std::default::Default::default(),
      default_value: ::std::default::Default::default(),
      oneof_index: ::std::default::Default::default(),
      json_name: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      proto3_optional: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FieldDescriptorProto_default: FieldDescriptorProto = FieldDescriptorProto::default();
}
impl ::pb::Message for FieldDescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut number_size = 0;
    for val in &self.number {
      let l = ::pb::Message::compute_size(val);
      number_size += ::pb::wire_format::serialized_length(3);
      number_size += l;
    }
    size += number_size;
    let mut label_size = 0;
    for val in &self.label {
      let l = ::pb::Message::compute_size(val);
      label_size += ::pb::wire_format::serialized_length(4);
      label_size += l;
    }
    size += label_size;
    let mut type__size = 0;
    for val in &self.type_ {
      let l = ::pb::Message::compute_size(val);
      type__size += ::pb::wire_format::serialized_length(5);
      type__size += l;
    }
    size += type__size;
    let mut type_name_size = 0;
    for val in &self.type_name {
      let l = ::pb::Message::compute_size(val);
      type_name_size += ::pb::wire_format::serialized_length(6);
      type_name_size += ::pb::varint::serialized_length(l as u64);
      type_name_size += l;
    }
    size += type_name_size;
    let mut extendee_size = 0;
    for val in &self.extendee {
      let l = ::pb::Message::compute_size(val);
      extendee_size += ::pb::wire_format::serialized_length(2);
      extendee_size += ::pb::varint::serialized_length(l as u64);
      extendee_size += l;
    }
    size += extendee_size;
    let mut default_value_size = 0;
    for val in &self.default_value {
      let l = ::pb::Message::compute_size(val);
      default_value_size += ::pb::wire_format::serialized_length(7);
      default_value_size += ::pb::varint::serialized_length(l as u64);
      default_value_size += l;
    }
    size += default_value_size;
    let mut oneof_index_size = 0;
    for val in &self.oneof_index {
      let l = ::pb::Message::compute_size(val);
      oneof_index_size += ::pb::wire_format::serialized_length(9);
      oneof_index_size += l;
    }
    size += oneof_index_size;
    let mut json_name_size = 0;
    for val in &self.json_name {
      let l = ::pb::Message::compute_size(val);
      json_name_size += ::pb::wire_format::serialized_length(10);
      json_name_size += ::pb::varint::serialized_length(l as u64);
      json_name_size += l;
    }
    size += json_name_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(8);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut proto3_optional_size = 0;
    for val in &self.proto3_optional {
      let l = ::pb::Message::compute_size(val);
      proto3_optional_size += ::pb::wire_format::serialized_length(17);
      proto3_optional_size += l;
    }
    size += proto3_optional_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.number {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.label {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.type_ {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.type_name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.extendee {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.default_value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.oneof_index {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.json_name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.proto3_optional {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.extendee {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.number {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.label {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.type_ {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.type_name {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.default_value {
      ::pb::wire_format::write(7, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
      ::pb::wire_format::write(8, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.oneof_index {
      ::pb::wire_format::write(9, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.json_name {
      ::pb::wire_format::write(10, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.proto3_optional {
      ::pb::wire_format::write(17, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FieldDescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldDescriptorProto", 3)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.number = Some(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldDescriptorProto", 4)?;
          let mut val: FieldDescriptorProto_Label = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.label = Some(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldDescriptorProto", 5)?;
          let mut val: FieldDescriptorProto_Type = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.type_ = Some(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FieldDescriptorProto", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.type_name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FieldDescriptorProto", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.extendee = Some(val);
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FieldDescriptorProto", 7)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.default_value = Some(val);
        }
        9 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldDescriptorProto", 9)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.oneof_index = Some(val);
        }
        10 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FieldDescriptorProto", 10)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.json_name = Some(val);
        }
        8 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FieldDescriptorProto", 8)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: FieldOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        17 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldDescriptorProto", 17)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.proto3_optional = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for FieldDescriptorProto {
  const NAME: &'static str = "FieldDescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.FieldDescriptorProto";
}

/// Describes a oneof.
#[derive(Clone, Debug, PartialEq)]
pub struct OneofDescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  pub options: ::std::option::Option<OneofOptions>,
}
impl OneofDescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: OneofOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> OneofOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &OneofOptions {
    self.options.as_ref().unwrap_or(&OneofOptions_default)
  }
}
impl ::std::default::Default for OneofDescriptorProto {
  fn default() -> Self {
    OneofDescriptorProto {
      name: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref OneofDescriptorProto_default: OneofDescriptorProto = OneofDescriptorProto::default();
}
impl ::pb::Message for OneofDescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(2);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.options {
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "OneofDescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "OneofDescriptorProto", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: OneofOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for OneofDescriptorProto {
  const NAME: &'static str = "OneofDescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.OneofDescriptorProto";
}

/// Describes an enum type.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumDescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  pub value: ::std::vec::Vec<EnumValueDescriptorProto>,
  pub options: ::std::option::Option<EnumOptions>,
  /// Range of reserved numeric values. Reserved numeric values may not be used
  /// by enum values in the same enum declaration. Reserved ranges may not
  /// overlap.
  pub reserved_range: ::std::vec::Vec<EnumDescriptorProto_EnumReservedRange>,
  /// Reserved enum value names, which may not be reused. A given name may only
  /// be reserved once.
  pub reserved_name: ::std::vec::Vec<::std::string::String>,
}
impl EnumDescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn set_value(&mut self, v: ::std::vec::Vec<EnumValueDescriptorProto>) {
    self.value = v;
  }
  pub fn take_value(&mut self) -> ::std::vec::Vec<EnumValueDescriptorProto> {
    ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
  }
  pub fn get_value(&self) -> &[EnumValueDescriptorProto] {
    &self.value
  }
  pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<EnumValueDescriptorProto> {
    &mut self.value
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: EnumOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> EnumOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &EnumOptions {
    self.options.as_ref().unwrap_or(&EnumOptions_default)
  }
  pub fn set_reserved_range(&mut self, v: ::std::vec::Vec<EnumDescriptorProto_EnumReservedRange>) {
    self.reserved_range = v;
  }
  pub fn take_reserved_range(&mut self) -> ::std::vec::Vec<EnumDescriptorProto_EnumReservedRange> {
    ::std::mem::replace(&mut self.reserved_range, ::std::vec::Vec::new())
  }
  pub fn get_reserved_range(&self) -> &[EnumDescriptorProto_EnumReservedRange] {
    &self.reserved_range
  }
  pub fn mut_reserved_range(&mut self) -> &mut ::std::vec::Vec<EnumDescriptorProto_EnumReservedRange> {
    &mut self.reserved_range
  }
  pub fn set_reserved_name(&mut self, v: ::std::vec::Vec<::std::string::String>) {
    self.reserved_name = v;
  }
  pub fn take_reserved_name(&mut self) -> ::std::vec::Vec<::std::string::String> {
    ::std::mem::replace(&mut self.reserved_name, ::std::vec::Vec::new())
  }
  pub fn get_reserved_name(&self) -> &[::std::string::String] {
    &self.reserved_name
  }
  pub fn mut_reserved_name(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
    &mut self.reserved_name
  }
}
impl ::std::default::Default for EnumDescriptorProto {
  fn default() -> Self {
    EnumDescriptorProto {
      name: ::std::default::Default::default(),
      value: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      reserved_range: ::std::default::Default::default(),
      reserved_name: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumDescriptorProto_default: EnumDescriptorProto = EnumDescriptorProto::default();
}
impl ::pb::Message for EnumDescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
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
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(3);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut reserved_range_size = 0;
    for val in &self.reserved_range {
      let l = ::pb::Message::compute_size(val);
      reserved_range_size += ::pb::wire_format::serialized_length(4);
      reserved_range_size += ::pb::varint::serialized_length(l as u64);
      reserved_range_size += l;
    }
    size += reserved_range_size;
    let mut reserved_name_size = 0;
    for val in &self.reserved_name {
      let l = ::pb::Message::compute_size(val);
      reserved_name_size += ::pb::wire_format::serialized_length(5);
      reserved_name_size += ::pb::varint::serialized_length(l as u64);
      reserved_name_size += l;
    }
    size += reserved_name_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.reserved_range {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.reserved_name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
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
    for val in &self.options {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.reserved_range {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.reserved_name {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumDescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumDescriptorProto", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: EnumValueDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.value.push(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumDescriptorProto", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: EnumOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumDescriptorProto", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: EnumDescriptorProto_EnumReservedRange = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.reserved_range.push(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumDescriptorProto", 5)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.reserved_name.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for EnumDescriptorProto {
  const NAME: &'static str = "EnumDescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.EnumDescriptorProto";
}

/// Range of reserved numeric values. Reserved values may not be used by
/// entries in the same enum. Reserved ranges may not overlap.

/// Note that this is distinct from DescriptorProto.ReservedRange in that it
/// is inclusive such that it can appropriately represent the entire int32
/// domain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnumDescriptorProto_EnumReservedRange {
  /// Inclusive.
  pub start: ::std::option::Option<i32>,
  /// Inclusive.
  pub end: ::std::option::Option<i32>,
}
impl EnumDescriptorProto_EnumReservedRange {
  pub fn has_start(&self) -> bool {
    self.start.is_some()
  }
  pub fn set_start(&mut self, v: i32) {
    self.start = Some(v);
  }
  pub fn get_start(&self) -> i32 {
    self.start.unwrap_or(0)
  }
  pub fn has_end(&self) -> bool {
    self.end.is_some()
  }
  pub fn set_end(&mut self, v: i32) {
    self.end = Some(v);
  }
  pub fn get_end(&self) -> i32 {
    self.end.unwrap_or(0)
  }
}
impl ::std::default::Default for EnumDescriptorProto_EnumReservedRange {
  fn default() -> Self {
    EnumDescriptorProto_EnumReservedRange {
      start: ::std::default::Default::default(),
      end: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumDescriptorProto_EnumReservedRange_default: EnumDescriptorProto_EnumReservedRange = EnumDescriptorProto_EnumReservedRange::default();
}
impl ::pb::Message for EnumDescriptorProto_EnumReservedRange {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut start_size = 0;
    for val in &self.start {
      let l = ::pb::Message::compute_size(val);
      start_size += ::pb::wire_format::serialized_length(1);
      start_size += l;
    }
    size += start_size;
    let mut end_size = 0;
    for val in &self.end {
      let l = ::pb::Message::compute_size(val);
      end_size += ::pb::wire_format::serialized_length(2);
      end_size += l;
    }
    size += end_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.start {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.end {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.start {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.end {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "EnumDescriptorProto_EnumReservedRange", 1)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.start = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "EnumDescriptorProto_EnumReservedRange", 2)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.end = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for EnumDescriptorProto_EnumReservedRange {
  const NAME: &'static str = "EnumDescriptorProto_EnumReservedRange";
  const FULL_NAME: &'static str = "google.protobuf.EnumDescriptorProto_EnumReservedRange";
}

/// Describes a value within an enum.
#[derive(Clone, Debug, PartialEq)]
pub struct EnumValueDescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  pub number: ::std::option::Option<i32>,
  pub options: ::std::option::Option<EnumValueOptions>,
}
impl EnumValueDescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_number(&self) -> bool {
    self.number.is_some()
  }
  pub fn set_number(&mut self, v: i32) {
    self.number = Some(v);
  }
  pub fn get_number(&self) -> i32 {
    self.number.unwrap_or(0)
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: EnumValueOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> EnumValueOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &EnumValueOptions {
    self.options.as_ref().unwrap_or(&EnumValueOptions_default)
  }
}
impl ::std::default::Default for EnumValueDescriptorProto {
  fn default() -> Self {
    EnumValueDescriptorProto {
      name: ::std::default::Default::default(),
      number: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumValueDescriptorProto_default: EnumValueDescriptorProto = EnumValueDescriptorProto::default();
}
impl ::pb::Message for EnumValueDescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut number_size = 0;
    for val in &self.number {
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
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.number {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.number {
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumValueDescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "EnumValueDescriptorProto", 2)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.number = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumValueDescriptorProto", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: EnumValueOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for EnumValueDescriptorProto {
  const NAME: &'static str = "EnumValueDescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.EnumValueDescriptorProto";
}

/// Describes a service.
#[derive(Clone, Debug, PartialEq)]
pub struct ServiceDescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  pub method: ::std::vec::Vec<MethodDescriptorProto>,
  pub options: ::std::option::Option<ServiceOptions>,
}
impl ServiceDescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn set_method(&mut self, v: ::std::vec::Vec<MethodDescriptorProto>) {
    self.method = v;
  }
  pub fn take_method(&mut self) -> ::std::vec::Vec<MethodDescriptorProto> {
    ::std::mem::replace(&mut self.method, ::std::vec::Vec::new())
  }
  pub fn get_method(&self) -> &[MethodDescriptorProto] {
    &self.method
  }
  pub fn mut_method(&mut self) -> &mut ::std::vec::Vec<MethodDescriptorProto> {
    &mut self.method
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: ServiceOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> ServiceOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &ServiceOptions {
    self.options.as_ref().unwrap_or(&ServiceOptions_default)
  }
}
impl ::std::default::Default for ServiceDescriptorProto {
  fn default() -> Self {
    ServiceDescriptorProto {
      name: ::std::default::Default::default(),
      method: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ServiceDescriptorProto_default: ServiceDescriptorProto = ServiceDescriptorProto::default();
}
impl ::pb::Message for ServiceDescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut method_size = 0;
    for val in &self.method {
      let l = ::pb::Message::compute_size(val);
      method_size += ::pb::wire_format::serialized_length(2);
      method_size += ::pb::varint::serialized_length(l as u64);
      method_size += l;
    }
    size += method_size;
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
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.method {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.method {
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
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "ServiceDescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "ServiceDescriptorProto", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: MethodDescriptorProto = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.method.push(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "ServiceDescriptorProto", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ServiceOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for ServiceDescriptorProto {
  const NAME: &'static str = "ServiceDescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.ServiceDescriptorProto";
}

/// Describes a method of a service.
#[derive(Clone, Debug, PartialEq)]
pub struct MethodDescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  /// Input and output type names.  These are resolved in the same way as
  /// FieldDescriptorProto.type_name, but must refer to a message type.
  pub input_type: ::std::option::Option<::std::string::String>,
  pub output_type: ::std::option::Option<::std::string::String>,
  pub options: ::std::option::Option<MethodOptions>,
  /// Identifies if client streams multiple client messages
  pub client_streaming: ::std::option::Option<bool>,
  /// Identifies if server streams multiple server messages
  pub server_streaming: ::std::option::Option<bool>,
}
impl MethodDescriptorProto {
  pub fn has_name(&self) -> bool {
    self.name.is_some()
  }
  pub fn set_name(&mut self, v: ::std::string::String) {
    self.name = Some(v);
  }
  pub fn take_name(&mut self) -> ::std::string::String {
    self.name.take().unwrap_or_default()
  }
  pub fn get_name(&self) -> &str {
    self.name.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_input_type(&self) -> bool {
    self.input_type.is_some()
  }
  pub fn set_input_type(&mut self, v: ::std::string::String) {
    self.input_type = Some(v);
  }
  pub fn take_input_type(&mut self) -> ::std::string::String {
    self.input_type.take().unwrap_or_default()
  }
  pub fn get_input_type(&self) -> &str {
    self.input_type.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_output_type(&self) -> bool {
    self.output_type.is_some()
  }
  pub fn set_output_type(&mut self, v: ::std::string::String) {
    self.output_type = Some(v);
  }
  pub fn take_output_type(&mut self) -> ::std::string::String {
    self.output_type.take().unwrap_or_default()
  }
  pub fn get_output_type(&self) -> &str {
    self.output_type.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_options(&self) -> bool {
    self.options.is_some()
  }
  pub fn set_options(&mut self, v: MethodOptions) {
    self.options = Some(v);
  }
  pub fn take_options(&mut self) -> MethodOptions {
    self.options.take().unwrap_or_default()
  }
  pub fn get_options(&self) -> &MethodOptions {
    self.options.as_ref().unwrap_or(&MethodOptions_default)
  }
  pub fn has_client_streaming(&self) -> bool {
    self.client_streaming.is_some()
  }
  pub fn set_client_streaming(&mut self, v: bool) {
    self.client_streaming = Some(v);
  }
  pub fn get_client_streaming(&self) -> bool {
    self.client_streaming.unwrap_or(false)
  }
  pub fn has_server_streaming(&self) -> bool {
    self.server_streaming.is_some()
  }
  pub fn set_server_streaming(&mut self, v: bool) {
    self.server_streaming = Some(v);
  }
  pub fn get_server_streaming(&self) -> bool {
    self.server_streaming.unwrap_or(false)
  }
}
impl ::std::default::Default for MethodDescriptorProto {
  fn default() -> Self {
    MethodDescriptorProto {
      name: ::std::default::Default::default(),
      input_type: ::std::default::Default::default(),
      output_type: ::std::default::Default::default(),
      options: ::std::default::Default::default(),
      client_streaming: Some(false),
      server_streaming: Some(false),
    }
  }
}
lazy_static! {
  pub static ref MethodDescriptorProto_default: MethodDescriptorProto = MethodDescriptorProto::default();
}
impl ::pb::Message for MethodDescriptorProto {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(1);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut input_type_size = 0;
    for val in &self.input_type {
      let l = ::pb::Message::compute_size(val);
      input_type_size += ::pb::wire_format::serialized_length(2);
      input_type_size += ::pb::varint::serialized_length(l as u64);
      input_type_size += l;
    }
    size += input_type_size;
    let mut output_type_size = 0;
    for val in &self.output_type {
      let l = ::pb::Message::compute_size(val);
      output_type_size += ::pb::wire_format::serialized_length(3);
      output_type_size += ::pb::varint::serialized_length(l as u64);
      output_type_size += l;
    }
    size += output_type_size;
    let mut options_size = 0;
    for val in &self.options {
      let l = ::pb::Message::compute_size(val);
      options_size += ::pb::wire_format::serialized_length(4);
      options_size += ::pb::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut client_streaming_size = 0;
    for val in &self.client_streaming {
      let l = ::pb::Message::compute_size(val);
      client_streaming_size += ::pb::wire_format::serialized_length(5);
      client_streaming_size += l;
    }
    size += client_streaming_size;
    let mut server_streaming_size = 0;
    for val in &self.server_streaming {
      let l = ::pb::Message::compute_size(val);
      server_streaming_size += ::pb::wire_format::serialized_length(6);
      server_streaming_size += l;
    }
    size += server_streaming_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.input_type {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.output_type {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.options {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.client_streaming {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.server_streaming {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.input_type {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.output_type {
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
    for val in &self.client_streaming {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.server_streaming {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "MethodDescriptorProto", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "MethodDescriptorProto", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.input_type = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "MethodDescriptorProto", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.output_type = Some(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "MethodDescriptorProto", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: MethodOptions = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.options = Some(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MethodDescriptorProto", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.client_streaming = Some(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MethodDescriptorProto", 6)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.server_streaming = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for MethodDescriptorProto {
  const NAME: &'static str = "MethodDescriptorProto";
  const FULL_NAME: &'static str = "google.protobuf.MethodDescriptorProto";
}

// ===================================================================
// Options

// Each of the definitions above may have "options" attached.  These are
// just annotations which may cause code to be generated slightly differently
// or may contain hints for code that manipulates protocol messages.

// Clients may define custom options as extensions of the *Options messages.
// These extensions may not yet be known at parsing time, so the parser cannot
// store the values in them.  Instead it stores them in a field in the *Options
// message called uninterpreted_option. This field must have the same name
// across all *Options messages. We then use this field to populate the
// extensions when we build a descriptor, at which point all protos have been
// parsed and so all extensions are known.

// Extension numbers for custom options may be chosen as follows:
// * For options which will only be used within a single application or
//   organization, or for experimental options, use field numbers 50000
//   through 99999.  It is up to you to ensure that you do not use the
//   same number for multiple options.
// * For options which will be published and used publicly by multiple
//   independent entities, e-mail protobuf-global-extension-registry@google.com
//   to reserve extension numbers. Simply provide your project name (e.g.
//   Objective-C plugin) and your project website (if available) -- there's no
//   need to explain how you intend to use them. Usually you only need one
//   extension number. You can declare multiple options with only one extension
//   number by putting them in a sub-message. See the Custom Options section of
//   the docs for examples:
//   https://developers.google.com/protocol-buffers/docs/proto#options
//   If this turns out to be popular, a web service will be set up
//   to automatically assign option numbers.

#[derive(Clone, Debug, PartialEq)]
pub struct FileOptions {
  /// Sets the Java package where classes generated from this .proto will be
  /// placed.  By default, the proto package is used, but this is often
  /// inappropriate because proto packages do not normally start with backwards
  /// domain names.
  pub java_package: ::std::option::Option<::std::string::String>,
  /// If set, all the classes from the .proto file are wrapped in a single
  /// outer class with the given name.  This applies to both Proto1
  /// (equivalent to the old "--one_java_file" option) and Proto2 (where
  /// a .proto always translates to a single class, but you may want to
  /// explicitly choose the class name).
  pub java_outer_classname: ::std::option::Option<::std::string::String>,
  /// If set true, then the Java code generator will generate a separate .java
  /// file for each top-level message, enum, and service defined in the .proto
  /// file.  Thus, these types will *not* be nested inside the outer class
  /// named by java_outer_classname.  However, the outer class will still be
  /// generated to contain the file's getDescriptor() method as well as any
  /// top-level extensions defined in the file.
  pub java_multiple_files: ::std::option::Option<bool>,
  /// This option does nothing.
  pub java_generate_equals_and_hash: ::std::option::Option<bool>,
  /// If set true, then the Java2 code generator will generate code that
  /// throws an exception whenever an attempt is made to assign a non-UTF-8
  /// byte sequence to a string field.
  /// Message reflection will do the same.
  /// However, an extension field still accepts non-UTF-8 byte sequences.
  /// This option has no effect on when used with the lite runtime.
  pub java_string_check_utf8: ::std::option::Option<bool>,
  pub optimize_for: ::std::option::Option<FileOptions_OptimizeMode>,
  /// Sets the Go package where structs generated from this .proto will be
  /// placed. If omitted, the Go package will be derived from the following:
  ///   - The basename of the package import path, if provided.
  ///   - Otherwise, the package statement in the .proto file, if present.
  ///   - Otherwise, the basename of the .proto file, without extension.
  pub go_package: ::std::option::Option<::std::string::String>,
  /// Should generic services be generated in each language?  "Generic" services
  /// are not specific to any particular RPC system.  They are generated by the
  /// main code generators in each language (without additional plugins).
  /// Generic services were the only kind of service generation supported by
  /// early versions of google.protobuf.

  /// Generic services are now considered deprecated in favor of using plugins
  /// that generate code specific to your particular RPC system.  Therefore,
  /// these default to false.  Old code which depends on generic services should
  /// explicitly set them to true.
  pub cc_generic_services: ::std::option::Option<bool>,
  pub java_generic_services: ::std::option::Option<bool>,
  pub py_generic_services: ::std::option::Option<bool>,
  pub php_generic_services: ::std::option::Option<bool>,
  /// Is this file deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for everything in the file, or it will be completely ignored; in the very
  /// least, this is a formalization for deprecating files.
  pub deprecated: ::std::option::Option<bool>,
  /// Enables the use of arenas for the proto messages in this file. This applies
  /// only to generated classes for C++.
  pub cc_enable_arenas: ::std::option::Option<bool>,
  /// Sets the objective c class prefix which is prepended to all objective c
  /// generated classes from this .proto. There is no default.
  pub objc_class_prefix: ::std::option::Option<::std::string::String>,
  /// Namespace for generated classes; defaults to the package.
  pub csharp_namespace: ::std::option::Option<::std::string::String>,
  /// By default Swift generators will take the proto package and CamelCase it
  /// replacing '.' with underscore and use that to prefix the types/symbols
  /// defined. When this options is provided, they will use this value instead
  /// to prefix the types/symbols defined.
  pub swift_prefix: ::std::option::Option<::std::string::String>,
  /// Sets the php class prefix which is prepended to all php generated classes
  /// from this .proto. Default is empty.
  pub php_class_prefix: ::std::option::Option<::std::string::String>,
  /// Use this option to change the namespace of php generated classes. Default
  /// is empty. When this option is empty, the package name will be used for
  /// determining the namespace.
  pub php_namespace: ::std::option::Option<::std::string::String>,
  /// Use this option to change the namespace of php generated metadata classes.
  /// Default is empty. When this option is empty, the proto file name will be
  /// used for determining the namespace.
  pub php_metadata_namespace: ::std::option::Option<::std::string::String>,
  /// Use this option to change the package of ruby generated classes. Default
  /// is empty. When this option is not set, the package name will be used for
  /// determining the ruby package.
  pub ruby_package: ::std::option::Option<::std::string::String>,
  /// The parser stores options it doesn't recognize here.
  /// See the documentation for the "Options" section above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl FileOptions {
  pub fn has_java_package(&self) -> bool {
    self.java_package.is_some()
  }
  pub fn set_java_package(&mut self, v: ::std::string::String) {
    self.java_package = Some(v);
  }
  pub fn take_java_package(&mut self) -> ::std::string::String {
    self.java_package.take().unwrap_or_default()
  }
  pub fn get_java_package(&self) -> &str {
    self.java_package.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_java_outer_classname(&self) -> bool {
    self.java_outer_classname.is_some()
  }
  pub fn set_java_outer_classname(&mut self, v: ::std::string::String) {
    self.java_outer_classname = Some(v);
  }
  pub fn take_java_outer_classname(&mut self) -> ::std::string::String {
    self.java_outer_classname.take().unwrap_or_default()
  }
  pub fn get_java_outer_classname(&self) -> &str {
    self.java_outer_classname.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_java_multiple_files(&self) -> bool {
    self.java_multiple_files.is_some()
  }
  pub fn set_java_multiple_files(&mut self, v: bool) {
    self.java_multiple_files = Some(v);
  }
  pub fn get_java_multiple_files(&self) -> bool {
    self.java_multiple_files.unwrap_or(false)
  }
  pub fn has_java_generate_equals_and_hash(&self) -> bool {
    self.java_generate_equals_and_hash.is_some()
  }
  pub fn set_java_generate_equals_and_hash(&mut self, v: bool) {
    self.java_generate_equals_and_hash = Some(v);
  }
  pub fn get_java_generate_equals_and_hash(&self) -> bool {
    self.java_generate_equals_and_hash.unwrap_or(false)
  }
  pub fn has_java_string_check_utf8(&self) -> bool {
    self.java_string_check_utf8.is_some()
  }
  pub fn set_java_string_check_utf8(&mut self, v: bool) {
    self.java_string_check_utf8 = Some(v);
  }
  pub fn get_java_string_check_utf8(&self) -> bool {
    self.java_string_check_utf8.unwrap_or(false)
  }
  pub fn has_optimize_for(&self) -> bool {
    self.optimize_for.is_some()
  }
  pub fn set_optimize_for(&mut self, v: FileOptions_OptimizeMode) {
    self.optimize_for = Some(v);
  }
  pub fn get_optimize_for(&self) -> FileOptions_OptimizeMode {
    self.optimize_for.unwrap_or_default()
  }
  pub fn has_go_package(&self) -> bool {
    self.go_package.is_some()
  }
  pub fn set_go_package(&mut self, v: ::std::string::String) {
    self.go_package = Some(v);
  }
  pub fn take_go_package(&mut self) -> ::std::string::String {
    self.go_package.take().unwrap_or_default()
  }
  pub fn get_go_package(&self) -> &str {
    self.go_package.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_cc_generic_services(&self) -> bool {
    self.cc_generic_services.is_some()
  }
  pub fn set_cc_generic_services(&mut self, v: bool) {
    self.cc_generic_services = Some(v);
  }
  pub fn get_cc_generic_services(&self) -> bool {
    self.cc_generic_services.unwrap_or(false)
  }
  pub fn has_java_generic_services(&self) -> bool {
    self.java_generic_services.is_some()
  }
  pub fn set_java_generic_services(&mut self, v: bool) {
    self.java_generic_services = Some(v);
  }
  pub fn get_java_generic_services(&self) -> bool {
    self.java_generic_services.unwrap_or(false)
  }
  pub fn has_py_generic_services(&self) -> bool {
    self.py_generic_services.is_some()
  }
  pub fn set_py_generic_services(&mut self, v: bool) {
    self.py_generic_services = Some(v);
  }
  pub fn get_py_generic_services(&self) -> bool {
    self.py_generic_services.unwrap_or(false)
  }
  pub fn has_php_generic_services(&self) -> bool {
    self.php_generic_services.is_some()
  }
  pub fn set_php_generic_services(&mut self, v: bool) {
    self.php_generic_services = Some(v);
  }
  pub fn get_php_generic_services(&self) -> bool {
    self.php_generic_services.unwrap_or(false)
  }
  pub fn has_deprecated(&self) -> bool {
    self.deprecated.is_some()
  }
  pub fn set_deprecated(&mut self, v: bool) {
    self.deprecated = Some(v);
  }
  pub fn get_deprecated(&self) -> bool {
    self.deprecated.unwrap_or(false)
  }
  pub fn has_cc_enable_arenas(&self) -> bool {
    self.cc_enable_arenas.is_some()
  }
  pub fn set_cc_enable_arenas(&mut self, v: bool) {
    self.cc_enable_arenas = Some(v);
  }
  pub fn get_cc_enable_arenas(&self) -> bool {
    self.cc_enable_arenas.unwrap_or(false)
  }
  pub fn has_objc_class_prefix(&self) -> bool {
    self.objc_class_prefix.is_some()
  }
  pub fn set_objc_class_prefix(&mut self, v: ::std::string::String) {
    self.objc_class_prefix = Some(v);
  }
  pub fn take_objc_class_prefix(&mut self) -> ::std::string::String {
    self.objc_class_prefix.take().unwrap_or_default()
  }
  pub fn get_objc_class_prefix(&self) -> &str {
    self.objc_class_prefix.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_csharp_namespace(&self) -> bool {
    self.csharp_namespace.is_some()
  }
  pub fn set_csharp_namespace(&mut self, v: ::std::string::String) {
    self.csharp_namespace = Some(v);
  }
  pub fn take_csharp_namespace(&mut self) -> ::std::string::String {
    self.csharp_namespace.take().unwrap_or_default()
  }
  pub fn get_csharp_namespace(&self) -> &str {
    self.csharp_namespace.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_swift_prefix(&self) -> bool {
    self.swift_prefix.is_some()
  }
  pub fn set_swift_prefix(&mut self, v: ::std::string::String) {
    self.swift_prefix = Some(v);
  }
  pub fn take_swift_prefix(&mut self) -> ::std::string::String {
    self.swift_prefix.take().unwrap_or_default()
  }
  pub fn get_swift_prefix(&self) -> &str {
    self.swift_prefix.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_php_class_prefix(&self) -> bool {
    self.php_class_prefix.is_some()
  }
  pub fn set_php_class_prefix(&mut self, v: ::std::string::String) {
    self.php_class_prefix = Some(v);
  }
  pub fn take_php_class_prefix(&mut self) -> ::std::string::String {
    self.php_class_prefix.take().unwrap_or_default()
  }
  pub fn get_php_class_prefix(&self) -> &str {
    self.php_class_prefix.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_php_namespace(&self) -> bool {
    self.php_namespace.is_some()
  }
  pub fn set_php_namespace(&mut self, v: ::std::string::String) {
    self.php_namespace = Some(v);
  }
  pub fn take_php_namespace(&mut self) -> ::std::string::String {
    self.php_namespace.take().unwrap_or_default()
  }
  pub fn get_php_namespace(&self) -> &str {
    self.php_namespace.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_php_metadata_namespace(&self) -> bool {
    self.php_metadata_namespace.is_some()
  }
  pub fn set_php_metadata_namespace(&mut self, v: ::std::string::String) {
    self.php_metadata_namespace = Some(v);
  }
  pub fn take_php_metadata_namespace(&mut self) -> ::std::string::String {
    self.php_metadata_namespace.take().unwrap_or_default()
  }
  pub fn get_php_metadata_namespace(&self) -> &str {
    self.php_metadata_namespace.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_ruby_package(&self) -> bool {
    self.ruby_package.is_some()
  }
  pub fn set_ruby_package(&mut self, v: ::std::string::String) {
    self.ruby_package = Some(v);
  }
  pub fn take_ruby_package(&mut self) -> ::std::string::String {
    self.ruby_package.take().unwrap_or_default()
  }
  pub fn get_ruby_package(&self) -> &str {
    self.ruby_package.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for FileOptions {
  fn default() -> Self {
    FileOptions {
      java_package: ::std::default::Default::default(),
      java_outer_classname: ::std::default::Default::default(),
      java_multiple_files: Some(false),
      java_generate_equals_and_hash: ::std::default::Default::default(),
      java_string_check_utf8: Some(false),
      optimize_for: Some(FileOptions_OptimizeMode::SPEED),
      go_package: ::std::default::Default::default(),
      cc_generic_services: Some(false),
      java_generic_services: Some(false),
      py_generic_services: Some(false),
      php_generic_services: Some(false),
      deprecated: Some(false),
      cc_enable_arenas: Some(true),
      objc_class_prefix: ::std::default::Default::default(),
      csharp_namespace: ::std::default::Default::default(),
      swift_prefix: ::std::default::Default::default(),
      php_class_prefix: ::std::default::Default::default(),
      php_namespace: ::std::default::Default::default(),
      php_metadata_namespace: ::std::default::Default::default(),
      ruby_package: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FileOptions_default: FileOptions = FileOptions::default();
}
impl ::pb::Message for FileOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut java_package_size = 0;
    for val in &self.java_package {
      let l = ::pb::Message::compute_size(val);
      java_package_size += ::pb::wire_format::serialized_length(1);
      java_package_size += ::pb::varint::serialized_length(l as u64);
      java_package_size += l;
    }
    size += java_package_size;
    let mut java_outer_classname_size = 0;
    for val in &self.java_outer_classname {
      let l = ::pb::Message::compute_size(val);
      java_outer_classname_size += ::pb::wire_format::serialized_length(8);
      java_outer_classname_size += ::pb::varint::serialized_length(l as u64);
      java_outer_classname_size += l;
    }
    size += java_outer_classname_size;
    let mut java_multiple_files_size = 0;
    for val in &self.java_multiple_files {
      let l = ::pb::Message::compute_size(val);
      java_multiple_files_size += ::pb::wire_format::serialized_length(10);
      java_multiple_files_size += l;
    }
    size += java_multiple_files_size;
    let mut java_generate_equals_and_hash_size = 0;
    for val in &self.java_generate_equals_and_hash {
      let l = ::pb::Message::compute_size(val);
      java_generate_equals_and_hash_size += ::pb::wire_format::serialized_length(20);
      java_generate_equals_and_hash_size += l;
    }
    size += java_generate_equals_and_hash_size;
    let mut java_string_check_utf8_size = 0;
    for val in &self.java_string_check_utf8 {
      let l = ::pb::Message::compute_size(val);
      java_string_check_utf8_size += ::pb::wire_format::serialized_length(27);
      java_string_check_utf8_size += l;
    }
    size += java_string_check_utf8_size;
    let mut optimize_for_size = 0;
    for val in &self.optimize_for {
      let l = ::pb::Message::compute_size(val);
      optimize_for_size += ::pb::wire_format::serialized_length(9);
      optimize_for_size += l;
    }
    size += optimize_for_size;
    let mut go_package_size = 0;
    for val in &self.go_package {
      let l = ::pb::Message::compute_size(val);
      go_package_size += ::pb::wire_format::serialized_length(11);
      go_package_size += ::pb::varint::serialized_length(l as u64);
      go_package_size += l;
    }
    size += go_package_size;
    let mut cc_generic_services_size = 0;
    for val in &self.cc_generic_services {
      let l = ::pb::Message::compute_size(val);
      cc_generic_services_size += ::pb::wire_format::serialized_length(16);
      cc_generic_services_size += l;
    }
    size += cc_generic_services_size;
    let mut java_generic_services_size = 0;
    for val in &self.java_generic_services {
      let l = ::pb::Message::compute_size(val);
      java_generic_services_size += ::pb::wire_format::serialized_length(17);
      java_generic_services_size += l;
    }
    size += java_generic_services_size;
    let mut py_generic_services_size = 0;
    for val in &self.py_generic_services {
      let l = ::pb::Message::compute_size(val);
      py_generic_services_size += ::pb::wire_format::serialized_length(18);
      py_generic_services_size += l;
    }
    size += py_generic_services_size;
    let mut php_generic_services_size = 0;
    for val in &self.php_generic_services {
      let l = ::pb::Message::compute_size(val);
      php_generic_services_size += ::pb::wire_format::serialized_length(42);
      php_generic_services_size += l;
    }
    size += php_generic_services_size;
    let mut deprecated_size = 0;
    for val in &self.deprecated {
      let l = ::pb::Message::compute_size(val);
      deprecated_size += ::pb::wire_format::serialized_length(23);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut cc_enable_arenas_size = 0;
    for val in &self.cc_enable_arenas {
      let l = ::pb::Message::compute_size(val);
      cc_enable_arenas_size += ::pb::wire_format::serialized_length(31);
      cc_enable_arenas_size += l;
    }
    size += cc_enable_arenas_size;
    let mut objc_class_prefix_size = 0;
    for val in &self.objc_class_prefix {
      let l = ::pb::Message::compute_size(val);
      objc_class_prefix_size += ::pb::wire_format::serialized_length(36);
      objc_class_prefix_size += ::pb::varint::serialized_length(l as u64);
      objc_class_prefix_size += l;
    }
    size += objc_class_prefix_size;
    let mut csharp_namespace_size = 0;
    for val in &self.csharp_namespace {
      let l = ::pb::Message::compute_size(val);
      csharp_namespace_size += ::pb::wire_format::serialized_length(37);
      csharp_namespace_size += ::pb::varint::serialized_length(l as u64);
      csharp_namespace_size += l;
    }
    size += csharp_namespace_size;
    let mut swift_prefix_size = 0;
    for val in &self.swift_prefix {
      let l = ::pb::Message::compute_size(val);
      swift_prefix_size += ::pb::wire_format::serialized_length(39);
      swift_prefix_size += ::pb::varint::serialized_length(l as u64);
      swift_prefix_size += l;
    }
    size += swift_prefix_size;
    let mut php_class_prefix_size = 0;
    for val in &self.php_class_prefix {
      let l = ::pb::Message::compute_size(val);
      php_class_prefix_size += ::pb::wire_format::serialized_length(40);
      php_class_prefix_size += ::pb::varint::serialized_length(l as u64);
      php_class_prefix_size += l;
    }
    size += php_class_prefix_size;
    let mut php_namespace_size = 0;
    for val in &self.php_namespace {
      let l = ::pb::Message::compute_size(val);
      php_namespace_size += ::pb::wire_format::serialized_length(41);
      php_namespace_size += ::pb::varint::serialized_length(l as u64);
      php_namespace_size += l;
    }
    size += php_namespace_size;
    let mut php_metadata_namespace_size = 0;
    for val in &self.php_metadata_namespace {
      let l = ::pb::Message::compute_size(val);
      php_metadata_namespace_size += ::pb::wire_format::serialized_length(44);
      php_metadata_namespace_size += ::pb::varint::serialized_length(l as u64);
      php_metadata_namespace_size += l;
    }
    size += php_metadata_namespace_size;
    let mut ruby_package_size = 0;
    for val in &self.ruby_package {
      let l = ::pb::Message::compute_size(val);
      ruby_package_size += ::pb::wire_format::serialized_length(45);
      ruby_package_size += ::pb::varint::serialized_length(l as u64);
      ruby_package_size += l;
    }
    size += ruby_package_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.java_package {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.java_outer_classname {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.java_multiple_files {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.java_generate_equals_and_hash {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.java_string_check_utf8 {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.optimize_for {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.go_package {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.cc_generic_services {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.java_generic_services {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.py_generic_services {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.php_generic_services {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.deprecated {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.cc_enable_arenas {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.objc_class_prefix {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.csharp_namespace {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.swift_prefix {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.php_class_prefix {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.php_namespace {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.php_metadata_namespace {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.ruby_package {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.java_package {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.java_outer_classname {
      ::pb::wire_format::write(8, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.optimize_for {
      ::pb::wire_format::write(9, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.java_multiple_files {
      ::pb::wire_format::write(10, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.go_package {
      ::pb::wire_format::write(11, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.cc_generic_services {
      ::pb::wire_format::write(16, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.java_generic_services {
      ::pb::wire_format::write(17, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.py_generic_services {
      ::pb::wire_format::write(18, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.java_generate_equals_and_hash {
      ::pb::wire_format::write(20, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.deprecated {
      ::pb::wire_format::write(23, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.java_string_check_utf8 {
      ::pb::wire_format::write(27, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.cc_enable_arenas {
      ::pb::wire_format::write(31, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.objc_class_prefix {
      ::pb::wire_format::write(36, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.csharp_namespace {
      ::pb::wire_format::write(37, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.swift_prefix {
      ::pb::wire_format::write(39, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.php_class_prefix {
      ::pb::wire_format::write(40, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.php_namespace {
      ::pb::wire_format::write(41, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.php_generic_services {
      ::pb::wire_format::write(42, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.php_metadata_namespace {
      ::pb::wire_format::write(44, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.ruby_package {
      ::pb::wire_format::write(45, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.java_package = Some(val);
        }
        8 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 8)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.java_outer_classname = Some(val);
        }
        10 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 10)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.java_multiple_files = Some(val);
        }
        20 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 20)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.java_generate_equals_and_hash = Some(val);
        }
        27 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 27)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.java_string_check_utf8 = Some(val);
        }
        9 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 9)?;
          let mut val: FileOptions_OptimizeMode = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.optimize_for = Some(val);
        }
        11 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 11)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.go_package = Some(val);
        }
        16 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 16)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.cc_generic_services = Some(val);
        }
        17 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 17)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.java_generic_services = Some(val);
        }
        18 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 18)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.py_generic_services = Some(val);
        }
        42 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 42)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.php_generic_services = Some(val);
        }
        23 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 23)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.deprecated = Some(val);
        }
        31 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FileOptions", 31)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.cc_enable_arenas = Some(val);
        }
        36 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 36)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.objc_class_prefix = Some(val);
        }
        37 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 37)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.csharp_namespace = Some(val);
        }
        39 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 39)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.swift_prefix = Some(val);
        }
        40 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 40)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.php_class_prefix = Some(val);
        }
        41 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 41)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.php_namespace = Some(val);
        }
        44 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 44)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.php_metadata_namespace = Some(val);
        }
        45 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 45)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.ruby_package = Some(val);
        }
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FileOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for FileOptions {
  const NAME: &'static str = "FileOptions";
  const FULL_NAME: &'static str = "google.protobuf.FileOptions";
}

#[derive(Clone, Debug, PartialEq)]
pub struct MessageOptions {
  /// Set true to use the old proto1 MessageSet wire format for extensions.
  /// This is provided for backwards-compatibility with the MessageSet wire
  /// format.  You should not use this for any other reason:  It's less
  /// efficient, has fewer features, and is more complicated.

  /// The message must be defined exactly as follows:
  ///   message Foo {
  ///     option message_set_wire_format = true;
  ///     extensions 4 to max;
  ///   }
  /// Note that the message cannot have any defined fields; MessageSets only
  /// have extensions.

  /// All extensions of your type must be singular messages; e.g. they cannot
  /// be int32s, enums, or repeated messages.

  /// Because this is an option, the above two restrictions are not enforced by
  /// the protocol compiler.
  pub message_set_wire_format: ::std::option::Option<bool>,
  /// Disables the generation of the standard "descriptor()" accessor, which can
  /// conflict with a field of the same name.  This is meant to make migration
  /// from proto1 easier; new code should avoid fields named "descriptor".
  pub no_standard_descriptor_accessor: ::std::option::Option<bool>,
  /// Is this message deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for the message, or it will be completely ignored; in the very least,
  /// this is a formalization for deprecating messages.
  pub deprecated: ::std::option::Option<bool>,
  /// Whether the message is an automatically generated map entry type for the
  /// maps field.

  /// For maps fields:
  ///     map<KeyType, ValueType> map_field = 1;
  /// The parsed descriptor looks like:
  ///     message MapFieldEntry {
  ///         option map_entry = true;
  ///         optional KeyType key = 1;
  ///         optional ValueType value = 2;
  ///     }
  ///     repeated MapFieldEntry map_field = 1;

  /// Implementations may choose not to generate the map_entry=true message, but
  /// use a native map in the target language to hold the keys and values.
  /// The reflection APIs in such implementations still need to work as
  /// if the field is a repeated message field.

  /// NOTE: Do not set the option in .proto files. Always use the maps syntax
  /// instead. The option should only be implicitly set by the proto compiler
  /// parser.
  pub map_entry: ::std::option::Option<bool>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl MessageOptions {
  pub fn has_message_set_wire_format(&self) -> bool {
    self.message_set_wire_format.is_some()
  }
  pub fn set_message_set_wire_format(&mut self, v: bool) {
    self.message_set_wire_format = Some(v);
  }
  pub fn get_message_set_wire_format(&self) -> bool {
    self.message_set_wire_format.unwrap_or(false)
  }
  pub fn has_no_standard_descriptor_accessor(&self) -> bool {
    self.no_standard_descriptor_accessor.is_some()
  }
  pub fn set_no_standard_descriptor_accessor(&mut self, v: bool) {
    self.no_standard_descriptor_accessor = Some(v);
  }
  pub fn get_no_standard_descriptor_accessor(&self) -> bool {
    self.no_standard_descriptor_accessor.unwrap_or(false)
  }
  pub fn has_deprecated(&self) -> bool {
    self.deprecated.is_some()
  }
  pub fn set_deprecated(&mut self, v: bool) {
    self.deprecated = Some(v);
  }
  pub fn get_deprecated(&self) -> bool {
    self.deprecated.unwrap_or(false)
  }
  pub fn has_map_entry(&self) -> bool {
    self.map_entry.is_some()
  }
  pub fn set_map_entry(&mut self, v: bool) {
    self.map_entry = Some(v);
  }
  pub fn get_map_entry(&self) -> bool {
    self.map_entry.unwrap_or(false)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for MessageOptions {
  fn default() -> Self {
    MessageOptions {
      message_set_wire_format: Some(false),
      no_standard_descriptor_accessor: Some(false),
      deprecated: Some(false),
      map_entry: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref MessageOptions_default: MessageOptions = MessageOptions::default();
}
impl ::pb::Message for MessageOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut message_set_wire_format_size = 0;
    for val in &self.message_set_wire_format {
      let l = ::pb::Message::compute_size(val);
      message_set_wire_format_size += ::pb::wire_format::serialized_length(1);
      message_set_wire_format_size += l;
    }
    size += message_set_wire_format_size;
    let mut no_standard_descriptor_accessor_size = 0;
    for val in &self.no_standard_descriptor_accessor {
      let l = ::pb::Message::compute_size(val);
      no_standard_descriptor_accessor_size += ::pb::wire_format::serialized_length(2);
      no_standard_descriptor_accessor_size += l;
    }
    size += no_standard_descriptor_accessor_size;
    let mut deprecated_size = 0;
    for val in &self.deprecated {
      let l = ::pb::Message::compute_size(val);
      deprecated_size += ::pb::wire_format::serialized_length(3);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut map_entry_size = 0;
    for val in &self.map_entry {
      let l = ::pb::Message::compute_size(val);
      map_entry_size += ::pb::wire_format::serialized_length(7);
      map_entry_size += l;
    }
    size += map_entry_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.message_set_wire_format {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.no_standard_descriptor_accessor {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.deprecated {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.map_entry {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.message_set_wire_format {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.no_standard_descriptor_accessor {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.deprecated {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.map_entry {
      ::pb::wire_format::write(7, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MessageOptions", 1)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.message_set_wire_format = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MessageOptions", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.no_standard_descriptor_accessor = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MessageOptions", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.deprecated = Some(val);
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MessageOptions", 7)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.map_entry = Some(val);
        }
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "MessageOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for MessageOptions {
  const NAME: &'static str = "MessageOptions";
  const FULL_NAME: &'static str = "google.protobuf.MessageOptions";
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOptions {
  /// The ctype option instructs the C++ code generator to use a different
  /// representation of the field than it normally would.  See the specific
  /// options below.  This option is not yet implemented in the open source
  /// release -- sorry, we'll try to include it in a future version!
  pub ctype: ::std::option::Option<FieldOptions_CType>,
  /// The packed option can be enabled for repeated primitive fields to enable
  /// a more efficient representation on the wire. Rather than repeatedly
  /// writing the tag and type for each element, the entire array is encoded as
  /// a single length-delimited blob. In proto3, only explicit setting it to
  /// false will avoid using packed encoding.
  pub packed: ::std::option::Option<bool>,
  /// The jstype option determines the JavaScript type used for values of the
  /// field.  The option is permitted only for 64 bit integral and fixed types
  /// (int64, uint64, sint64, fixed64, sfixed64).  A field with jstype JS_STRING
  /// is represented as JavaScript string, which avoids loss of precision that
  /// can happen when a large value is converted to a floating point JavaScript.
  /// Specifying JS_NUMBER for the jstype causes the generated JavaScript code to
  /// use the JavaScript "number" type.  The behavior of the default option
  /// JS_NORMAL is implementation dependent.

  /// This option is an enum to permit additional types to be added, e.g.
  /// goog.math.Integer.
  pub jstype: ::std::option::Option<FieldOptions_JSType>,
  /// Should this field be parsed lazily?  Lazy applies only to message-type
  /// fields.  It means that when the outer message is initially parsed, the
  /// inner message's contents will not be parsed but instead stored in encoded
  /// form.  The inner message will actually be parsed when it is first accessed.

  /// This is only a hint.  Implementations are free to choose whether to use
  /// eager or lazy parsing regardless of the value of this option.  However,
  /// setting this option true suggests that the protocol author believes that
  /// using lazy parsing on this field is worth the additional bookkeeping
  /// overhead typically needed to implement it.

  /// This option does not affect the public interface of any generated code;
  /// all method signatures remain the same.  Furthermore, thread-safety of the
  /// interface is not affected by this option; const methods remain safe to
  /// call from multiple threads concurrently, while non-const methods continue
  /// to require exclusive access.


  /// Note that implementations may choose not to check required fields within
  /// a lazy sub-message.  That is, calling IsInitialized() on the outer message
  /// may return true even if the inner message has missing required fields.
  /// This is necessary because otherwise the inner message would have to be
  /// parsed in order to perform the check, defeating the purpose of lazy
  /// parsing.  An implementation which chooses not to check required fields
  /// must be consistent about it.  That is, for any particular sub-message, the
  /// implementation must either *always* check its required fields, or *never*
  /// check its required fields, regardless of whether or not the message has
  /// been parsed.
  pub lazy: ::std::option::Option<bool>,
  /// Is this field deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for accessors, or it will be completely ignored; in the very least, this
  /// is a formalization for deprecating fields.
  pub deprecated: ::std::option::Option<bool>,
  /// For Google-internal migration only. Do not use.
  pub weak: ::std::option::Option<bool>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl FieldOptions {
  pub fn has_ctype(&self) -> bool {
    self.ctype.is_some()
  }
  pub fn set_ctype(&mut self, v: FieldOptions_CType) {
    self.ctype = Some(v);
  }
  pub fn get_ctype(&self) -> FieldOptions_CType {
    self.ctype.unwrap_or_default()
  }
  pub fn has_packed(&self) -> bool {
    self.packed.is_some()
  }
  pub fn set_packed(&mut self, v: bool) {
    self.packed = Some(v);
  }
  pub fn get_packed(&self) -> bool {
    self.packed.unwrap_or(false)
  }
  pub fn has_jstype(&self) -> bool {
    self.jstype.is_some()
  }
  pub fn set_jstype(&mut self, v: FieldOptions_JSType) {
    self.jstype = Some(v);
  }
  pub fn get_jstype(&self) -> FieldOptions_JSType {
    self.jstype.unwrap_or_default()
  }
  pub fn has_lazy(&self) -> bool {
    self.lazy.is_some()
  }
  pub fn set_lazy(&mut self, v: bool) {
    self.lazy = Some(v);
  }
  pub fn get_lazy(&self) -> bool {
    self.lazy.unwrap_or(false)
  }
  pub fn has_deprecated(&self) -> bool {
    self.deprecated.is_some()
  }
  pub fn set_deprecated(&mut self, v: bool) {
    self.deprecated = Some(v);
  }
  pub fn get_deprecated(&self) -> bool {
    self.deprecated.unwrap_or(false)
  }
  pub fn has_weak(&self) -> bool {
    self.weak.is_some()
  }
  pub fn set_weak(&mut self, v: bool) {
    self.weak = Some(v);
  }
  pub fn get_weak(&self) -> bool {
    self.weak.unwrap_or(false)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for FieldOptions {
  fn default() -> Self {
    FieldOptions {
      ctype: Some(FieldOptions_CType::STRING),
      packed: ::std::default::Default::default(),
      jstype: Some(FieldOptions_JSType::JS_NORMAL),
      lazy: Some(false),
      deprecated: Some(false),
      weak: Some(false),
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FieldOptions_default: FieldOptions = FieldOptions::default();
}
impl ::pb::Message for FieldOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut ctype_size = 0;
    for val in &self.ctype {
      let l = ::pb::Message::compute_size(val);
      ctype_size += ::pb::wire_format::serialized_length(1);
      ctype_size += l;
    }
    size += ctype_size;
    let mut packed_size = 0;
    for val in &self.packed {
      let l = ::pb::Message::compute_size(val);
      packed_size += ::pb::wire_format::serialized_length(2);
      packed_size += l;
    }
    size += packed_size;
    let mut jstype_size = 0;
    for val in &self.jstype {
      let l = ::pb::Message::compute_size(val);
      jstype_size += ::pb::wire_format::serialized_length(6);
      jstype_size += l;
    }
    size += jstype_size;
    let mut lazy_size = 0;
    for val in &self.lazy {
      let l = ::pb::Message::compute_size(val);
      lazy_size += ::pb::wire_format::serialized_length(5);
      lazy_size += l;
    }
    size += lazy_size;
    let mut deprecated_size = 0;
    for val in &self.deprecated {
      let l = ::pb::Message::compute_size(val);
      deprecated_size += ::pb::wire_format::serialized_length(3);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut weak_size = 0;
    for val in &self.weak {
      let l = ::pb::Message::compute_size(val);
      weak_size += ::pb::wire_format::serialized_length(10);
      weak_size += l;
    }
    size += weak_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.ctype {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.packed {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.jstype {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.lazy {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.deprecated {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.weak {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.ctype {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.packed {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.deprecated {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.lazy {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.jstype {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.weak {
      ::pb::wire_format::write(10, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldOptions", 1)?;
          let mut val: FieldOptions_CType = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.ctype = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldOptions", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.packed = Some(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldOptions", 6)?;
          let mut val: FieldOptions_JSType = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.jstype = Some(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldOptions", 5)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.lazy = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldOptions", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.deprecated = Some(val);
        }
        10 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "FieldOptions", 10)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.weak = Some(val);
        }
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "FieldOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for FieldOptions {
  const NAME: &'static str = "FieldOptions";
  const FULL_NAME: &'static str = "google.protobuf.FieldOptions";
}

#[derive(Clone, Debug, PartialEq)]
pub struct OneofOptions {
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl OneofOptions {
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for OneofOptions {
  fn default() -> Self {
    OneofOptions {
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref OneofOptions_default: OneofOptions = OneofOptions::default();
}
impl ::pb::Message for OneofOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "OneofOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for OneofOptions {
  const NAME: &'static str = "OneofOptions";
  const FULL_NAME: &'static str = "google.protobuf.OneofOptions";
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumOptions {
  /// Set this option to true to allow mapping different tag names to the same
  /// value.
  pub allow_alias: ::std::option::Option<bool>,
  /// Is this enum deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for the enum, or it will be completely ignored; in the very least, this
  /// is a formalization for deprecating enums.
  pub deprecated: ::std::option::Option<bool>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl EnumOptions {
  pub fn has_allow_alias(&self) -> bool {
    self.allow_alias.is_some()
  }
  pub fn set_allow_alias(&mut self, v: bool) {
    self.allow_alias = Some(v);
  }
  pub fn get_allow_alias(&self) -> bool {
    self.allow_alias.unwrap_or(false)
  }
  pub fn has_deprecated(&self) -> bool {
    self.deprecated.is_some()
  }
  pub fn set_deprecated(&mut self, v: bool) {
    self.deprecated = Some(v);
  }
  pub fn get_deprecated(&self) -> bool {
    self.deprecated.unwrap_or(false)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for EnumOptions {
  fn default() -> Self {
    EnumOptions {
      allow_alias: ::std::default::Default::default(),
      deprecated: Some(false),
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumOptions_default: EnumOptions = EnumOptions::default();
}
impl ::pb::Message for EnumOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut allow_alias_size = 0;
    for val in &self.allow_alias {
      let l = ::pb::Message::compute_size(val);
      allow_alias_size += ::pb::wire_format::serialized_length(2);
      allow_alias_size += l;
    }
    size += allow_alias_size;
    let mut deprecated_size = 0;
    for val in &self.deprecated {
      let l = ::pb::Message::compute_size(val);
      deprecated_size += ::pb::wire_format::serialized_length(3);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.allow_alias {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.deprecated {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.allow_alias {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.deprecated {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "EnumOptions", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.allow_alias = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "EnumOptions", 3)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.deprecated = Some(val);
        }
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for EnumOptions {
  const NAME: &'static str = "EnumOptions";
  const FULL_NAME: &'static str = "google.protobuf.EnumOptions";
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumValueOptions {
  /// Is this enum value deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for the enum value, or it will be completely ignored; in the very least,
  /// this is a formalization for deprecating enum values.
  pub deprecated: ::std::option::Option<bool>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl EnumValueOptions {
  pub fn has_deprecated(&self) -> bool {
    self.deprecated.is_some()
  }
  pub fn set_deprecated(&mut self, v: bool) {
    self.deprecated = Some(v);
  }
  pub fn get_deprecated(&self) -> bool {
    self.deprecated.unwrap_or(false)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for EnumValueOptions {
  fn default() -> Self {
    EnumValueOptions {
      deprecated: Some(false),
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumValueOptions_default: EnumValueOptions = EnumValueOptions::default();
}
impl ::pb::Message for EnumValueOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut deprecated_size = 0;
    for val in &self.deprecated {
      let l = ::pb::Message::compute_size(val);
      deprecated_size += ::pb::wire_format::serialized_length(1);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.deprecated {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.deprecated {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "EnumValueOptions", 1)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.deprecated = Some(val);
        }
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "EnumValueOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for EnumValueOptions {
  const NAME: &'static str = "EnumValueOptions";
  const FULL_NAME: &'static str = "google.protobuf.EnumValueOptions";
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServiceOptions {
  // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
  //   framework.  We apologize for hoarding these numbers to ourselves, but
  //   we were already using them long before we decided to release Protocol
  //   Buffers.

  /// Is this service deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for the service, or it will be completely ignored; in the very least,
  /// this is a formalization for deprecating services.
  pub deprecated: ::std::option::Option<bool>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl ServiceOptions {
  pub fn has_deprecated(&self) -> bool {
    self.deprecated.is_some()
  }
  pub fn set_deprecated(&mut self, v: bool) {
    self.deprecated = Some(v);
  }
  pub fn get_deprecated(&self) -> bool {
    self.deprecated.unwrap_or(false)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for ServiceOptions {
  fn default() -> Self {
    ServiceOptions {
      deprecated: Some(false),
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ServiceOptions_default: ServiceOptions = ServiceOptions::default();
}
impl ::pb::Message for ServiceOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut deprecated_size = 0;
    for val in &self.deprecated {
      let l = ::pb::Message::compute_size(val);
      deprecated_size += ::pb::wire_format::serialized_length(33);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.deprecated {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.deprecated {
      ::pb::wire_format::write(33, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        33 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "ServiceOptions", 33)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.deprecated = Some(val);
        }
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "ServiceOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for ServiceOptions {
  const NAME: &'static str = "ServiceOptions";
  const FULL_NAME: &'static str = "google.protobuf.ServiceOptions";
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodOptions {
  // Note:  Field numbers 1 through 32 are reserved for Google's internal RPC
  //   framework.  We apologize for hoarding these numbers to ourselves, but
  //   we were already using them long before we decided to release Protocol
  //   Buffers.

  /// Is this method deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for the method, or it will be completely ignored; in the very least,
  /// this is a formalization for deprecating methods.
  pub deprecated: ::std::option::Option<bool>,
  pub idempotency_level: ::std::option::Option<MethodOptions_IdempotencyLevel>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
}
impl MethodOptions {
  pub fn has_deprecated(&self) -> bool {
    self.deprecated.is_some()
  }
  pub fn set_deprecated(&mut self, v: bool) {
    self.deprecated = Some(v);
  }
  pub fn get_deprecated(&self) -> bool {
    self.deprecated.unwrap_or(false)
  }
  pub fn has_idempotency_level(&self) -> bool {
    self.idempotency_level.is_some()
  }
  pub fn set_idempotency_level(&mut self, v: MethodOptions_IdempotencyLevel) {
    self.idempotency_level = Some(v);
  }
  pub fn get_idempotency_level(&self) -> MethodOptions_IdempotencyLevel {
    self.idempotency_level.unwrap_or_default()
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::replace(&mut self.uninterpreted_option, ::std::vec::Vec::new())
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
}
impl ::std::default::Default for MethodOptions {
  fn default() -> Self {
    MethodOptions {
      deprecated: Some(false),
      idempotency_level: Some(MethodOptions_IdempotencyLevel::IDEMPOTENCY_UNKNOWN),
      uninterpreted_option: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref MethodOptions_default: MethodOptions = MethodOptions::default();
}
impl ::pb::Message for MethodOptions {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut deprecated_size = 0;
    for val in &self.deprecated {
      let l = ::pb::Message::compute_size(val);
      deprecated_size += ::pb::wire_format::serialized_length(33);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut idempotency_level_size = 0;
    for val in &self.idempotency_level {
      let l = ::pb::Message::compute_size(val);
      idempotency_level_size += ::pb::wire_format::serialized_length(34);
      idempotency_level_size += l;
    }
    size += idempotency_level_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb::Message::compute_size(val);
      uninterpreted_option_size += ::pb::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.deprecated {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.idempotency_level {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.uninterpreted_option {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.deprecated {
      ::pb::wire_format::write(33, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.idempotency_level {
      ::pb::wire_format::write(34, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb::wire_format::write(999, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        33 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MethodOptions", 33)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.deprecated = Some(val);
        }
        34 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "MethodOptions", 34)?;
          let mut val: MethodOptions_IdempotencyLevel = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.idempotency_level = Some(val);
        }
        999 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "MethodOptions", 999)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.uninterpreted_option.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for MethodOptions {
  const NAME: &'static str = "MethodOptions";
  const FULL_NAME: &'static str = "google.protobuf.MethodOptions";
}

/// A message representing a option the parser does not recognize. This only
/// appears in options protos created by the compiler::Parser class.
/// DescriptorPool resolves these when building Descriptor objects. Therefore,
/// options protos in descriptor objects (e.g. returned by Descriptor::options(),
/// or produced by Descriptor::CopyTo()) will never have UninterpretedOptions
/// in them.
#[derive(Clone, Debug, PartialEq)]
pub struct UninterpretedOption {
  pub name: ::std::vec::Vec<UninterpretedOption_NamePart>,
  /// The value of the uninterpreted option, in whatever type the tokenizer
  /// identified it as during parsing. Exactly one of these should be set.
  pub identifier_value: ::std::option::Option<::std::string::String>,
  pub positive_int_value: ::std::option::Option<u64>,
  pub negative_int_value: ::std::option::Option<i64>,
  pub double_value: ::std::option::Option<f64>,
  pub string_value: ::std::option::Option<::std::vec::Vec<u8>>,
  pub aggregate_value: ::std::option::Option<::std::string::String>,
}
impl UninterpretedOption {
  pub fn set_name(&mut self, v: ::std::vec::Vec<UninterpretedOption_NamePart>) {
    self.name = v;
  }
  pub fn take_name(&mut self) -> ::std::vec::Vec<UninterpretedOption_NamePart> {
    ::std::mem::replace(&mut self.name, ::std::vec::Vec::new())
  }
  pub fn get_name(&self) -> &[UninterpretedOption_NamePart] {
    &self.name
  }
  pub fn mut_name(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption_NamePart> {
    &mut self.name
  }
  pub fn has_identifier_value(&self) -> bool {
    self.identifier_value.is_some()
  }
  pub fn set_identifier_value(&mut self, v: ::std::string::String) {
    self.identifier_value = Some(v);
  }
  pub fn take_identifier_value(&mut self) -> ::std::string::String {
    self.identifier_value.take().unwrap_or_default()
  }
  pub fn get_identifier_value(&self) -> &str {
    self.identifier_value.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_positive_int_value(&self) -> bool {
    self.positive_int_value.is_some()
  }
  pub fn set_positive_int_value(&mut self, v: u64) {
    self.positive_int_value = Some(v);
  }
  pub fn get_positive_int_value(&self) -> u64 {
    self.positive_int_value.unwrap_or(0)
  }
  pub fn has_negative_int_value(&self) -> bool {
    self.negative_int_value.is_some()
  }
  pub fn set_negative_int_value(&mut self, v: i64) {
    self.negative_int_value = Some(v);
  }
  pub fn get_negative_int_value(&self) -> i64 {
    self.negative_int_value.unwrap_or(0)
  }
  pub fn has_double_value(&self) -> bool {
    self.double_value.is_some()
  }
  pub fn set_double_value(&mut self, v: f64) {
    self.double_value = Some(v);
  }
  pub fn get_double_value(&self) -> f64 {
    self.double_value.unwrap_or(0.)
  }
  pub fn has_string_value(&self) -> bool {
    self.string_value.is_some()
  }
  pub fn set_string_value(&mut self, v: ::std::vec::Vec<u8>) {
    self.string_value = Some(v);
  }
  pub fn take_string_value(&mut self) -> ::std::vec::Vec<u8> {
    self.string_value.take().unwrap_or_default()
  }
  pub fn get_string_value(&self) -> &[u8] {
    self.string_value.as_ref().map(|v| &**v).unwrap_or(&[])
  }
  pub fn has_aggregate_value(&self) -> bool {
    self.aggregate_value.is_some()
  }
  pub fn set_aggregate_value(&mut self, v: ::std::string::String) {
    self.aggregate_value = Some(v);
  }
  pub fn take_aggregate_value(&mut self) -> ::std::string::String {
    self.aggregate_value.take().unwrap_or_default()
  }
  pub fn get_aggregate_value(&self) -> &str {
    self.aggregate_value.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
}
impl ::std::default::Default for UninterpretedOption {
  fn default() -> Self {
    UninterpretedOption {
      name: ::std::default::Default::default(),
      identifier_value: ::std::default::Default::default(),
      positive_int_value: ::std::default::Default::default(),
      negative_int_value: ::std::default::Default::default(),
      double_value: ::std::default::Default::default(),
      string_value: ::std::default::Default::default(),
      aggregate_value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref UninterpretedOption_default: UninterpretedOption = UninterpretedOption::default();
}
impl ::pb::Message for UninterpretedOption {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb::Message::compute_size(val);
      name_size += ::pb::wire_format::serialized_length(2);
      name_size += ::pb::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut identifier_value_size = 0;
    for val in &self.identifier_value {
      let l = ::pb::Message::compute_size(val);
      identifier_value_size += ::pb::wire_format::serialized_length(3);
      identifier_value_size += ::pb::varint::serialized_length(l as u64);
      identifier_value_size += l;
    }
    size += identifier_value_size;
    let mut positive_int_value_size = 0;
    for val in &self.positive_int_value {
      let l = ::pb::Message::compute_size(val);
      positive_int_value_size += ::pb::wire_format::serialized_length(4);
      positive_int_value_size += l;
    }
    size += positive_int_value_size;
    let mut negative_int_value_size = 0;
    for val in &self.negative_int_value {
      let l = ::pb::Message::compute_size(val);
      negative_int_value_size += ::pb::wire_format::serialized_length(5);
      negative_int_value_size += l;
    }
    size += negative_int_value_size;
    let mut double_value_size = 0;
    for val in &self.double_value {
      let l = ::pb::Message::compute_size(val);
      double_value_size += ::pb::wire_format::serialized_length(6);
      double_value_size += l;
    }
    size += double_value_size;
    let mut string_value_size = 0;
    for val in &self.string_value {
      let l = ::pb::Message::compute_size(val);
      string_value_size += ::pb::wire_format::serialized_length(7);
      string_value_size += ::pb::varint::serialized_length(l as u64);
      string_value_size += l;
    }
    size += string_value_size;
    let mut aggregate_value_size = 0;
    for val in &self.aggregate_value {
      let l = ::pb::Message::compute_size(val);
      aggregate_value_size += ::pb::wire_format::serialized_length(8);
      aggregate_value_size += ::pb::varint::serialized_length(l as u64);
      aggregate_value_size += l;
    }
    size += aggregate_value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.identifier_value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.positive_int_value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.negative_int_value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.double_value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.string_value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.aggregate_value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.identifier_value {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.positive_int_value {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.negative_int_value {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.double_value {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::Fixed64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.string_value {
      ::pb::wire_format::write(7, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.aggregate_value {
      ::pb::wire_format::write(8, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "UninterpretedOption", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: UninterpretedOption_NamePart = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name.push(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "UninterpretedOption", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.identifier_value = Some(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "UninterpretedOption", 4)?;
          let mut val: u64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.positive_int_value = Some(val);
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "UninterpretedOption", 5)?;
          let mut val: i64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.negative_int_value = Some(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Fixed64, "UninterpretedOption", 6)?;
          let mut val: f64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.double_value = Some(val);
        }
        7 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "UninterpretedOption", 7)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::vec::Vec<u8> = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.string_value = Some(val);
        }
        8 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "UninterpretedOption", 8)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.aggregate_value = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for UninterpretedOption {
  const NAME: &'static str = "UninterpretedOption";
  const FULL_NAME: &'static str = "google.protobuf.UninterpretedOption";
}

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name.  is_extension is true iff a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
/// E.g.,{ ["foo", false], ["bar.baz", true], ["qux", false] } represents
/// "foo.(bar.baz).qux".
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UninterpretedOption_NamePart {
  pub name_part: ::std::option::Option<::std::string::String>,
  pub is_extension: ::std::option::Option<bool>,
}
impl UninterpretedOption_NamePart {
  pub fn has_name_part(&self) -> bool {
    self.name_part.is_some()
  }
  pub fn set_name_part(&mut self, v: ::std::string::String) {
    self.name_part = Some(v);
  }
  pub fn take_name_part(&mut self) -> ::std::string::String {
    self.name_part.take().unwrap_or_default()
  }
  pub fn get_name_part(&self) -> &str {
    self.name_part.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_is_extension(&self) -> bool {
    self.is_extension.is_some()
  }
  pub fn set_is_extension(&mut self, v: bool) {
    self.is_extension = Some(v);
  }
  pub fn get_is_extension(&self) -> bool {
    self.is_extension.unwrap_or(false)
  }
}
impl ::std::default::Default for UninterpretedOption_NamePart {
  fn default() -> Self {
    UninterpretedOption_NamePart {
      name_part: ::std::default::Default::default(),
      is_extension: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref UninterpretedOption_NamePart_default: UninterpretedOption_NamePart = UninterpretedOption_NamePart::default();
}
impl ::pb::Message for UninterpretedOption_NamePart {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut name_part_size = 0;
    for val in &self.name_part {
      let l = ::pb::Message::compute_size(val);
      name_part_size += ::pb::wire_format::serialized_length(1);
      name_part_size += ::pb::varint::serialized_length(l as u64);
      name_part_size += l;
    }
    size += name_part_size;
    let mut is_extension_size = 0;
    for val in &self.is_extension {
      let l = ::pb::Message::compute_size(val);
      is_extension_size += ::pb::wire_format::serialized_length(2);
      is_extension_size += l;
    }
    size += is_extension_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.name_part {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.is_extension {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name_part {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.is_extension {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "UninterpretedOption_NamePart", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.name_part = Some(val);
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "UninterpretedOption_NamePart", 2)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.is_extension = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for UninterpretedOption_NamePart {
  const NAME: &'static str = "UninterpretedOption_NamePart";
  const FULL_NAME: &'static str = "google.protobuf.UninterpretedOption_NamePart";
}

// ===================================================================
// Optional source code info

/// Encapsulates information about the original source file from which a
/// FileDescriptorProto was generated.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceCodeInfo {
  /// A Location identifies a piece of source code in a .proto file which
  /// corresponds to a particular definition.  This information is intended
  /// to be useful to IDEs, code indexers, documentation generators, and similar
  /// tools.

  /// For example, say we have a file like:
  ///   message Foo {
  ///     optional string foo = 1;
  ///   }
  /// Let's look at just the field definition:
  ///   optional string foo = 1;
  ///   ^       ^^     ^^  ^  ^^^
  ///   a       bc     de  f  ghi
  /// We have the following locations:
  ///   span   path               represents
  ///   [a,i)  [ 4, 0, 2, 0 ]     The whole field definition.
  ///   [a,b)  [ 4, 0, 2, 0, 4 ]  The label (optional).
  ///   [c,d)  [ 4, 0, 2, 0, 5 ]  The type (string).
  ///   [e,f)  [ 4, 0, 2, 0, 1 ]  The name (foo).
  ///   [g,h)  [ 4, 0, 2, 0, 3 ]  The number (1).

  /// Notes:
  /// - A location may refer to a repeated field itself (i.e. not to any
  ///   particular index within it).  This is used whenever a set of elements are
  ///   logically enclosed in a single code segment.  For example, an entire
  ///   extend block (possibly containing multiple extension definitions) will
  ///   have an outer location whose path refers to the "extensions" repeated
  ///   field without an index.
  /// - Multiple locations may have the same path.  This happens when a single
  ///   logical declaration is spread out across multiple places.  The most
  ///   obvious example is the "extend" block again -- there may be multiple
  ///   extend blocks in the same scope, each of which will have the same path.
  /// - A location's span is not always a subset of its parent's span.  For
  ///   example, the "extendee" of an extension declaration appears at the
  ///   beginning of the "extend" block and is shared by all extensions within
  ///   the block.
  /// - Just because a location's span is a subset of some other location's span
  ///   does not mean that it is a descendant.  For example, a "group" defines
  ///   both a type and a field in a single declaration.  Thus, the locations
  ///   corresponding to the type and field and their components will overlap.
  /// - Code which tries to interpret locations should probably be designed to
  ///   ignore those that it doesn't understand, as more types of locations could
  ///   be recorded in the future.
  pub location: ::std::vec::Vec<SourceCodeInfo_Location>,
}
impl SourceCodeInfo {
  pub fn set_location(&mut self, v: ::std::vec::Vec<SourceCodeInfo_Location>) {
    self.location = v;
  }
  pub fn take_location(&mut self) -> ::std::vec::Vec<SourceCodeInfo_Location> {
    ::std::mem::replace(&mut self.location, ::std::vec::Vec::new())
  }
  pub fn get_location(&self) -> &[SourceCodeInfo_Location] {
    &self.location
  }
  pub fn mut_location(&mut self) -> &mut ::std::vec::Vec<SourceCodeInfo_Location> {
    &mut self.location
  }
}
impl ::std::default::Default for SourceCodeInfo {
  fn default() -> Self {
    SourceCodeInfo {
      location: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SourceCodeInfo_default: SourceCodeInfo = SourceCodeInfo::default();
}
impl ::pb::Message for SourceCodeInfo {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut location_size = 0;
    for val in &self.location {
      let l = ::pb::Message::compute_size(val);
      location_size += ::pb::wire_format::serialized_length(1);
      location_size += ::pb::varint::serialized_length(l as u64);
      location_size += l;
    }
    size += location_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.location {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.location {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "SourceCodeInfo", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: SourceCodeInfo_Location = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.location.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for SourceCodeInfo {
  const NAME: &'static str = "SourceCodeInfo";
  const FULL_NAME: &'static str = "google.protobuf.SourceCodeInfo";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceCodeInfo_Location {
  /// Identifies which part of the FileDescriptorProto was defined at this
  /// location.

  /// Each element is a field number or an index.  They form a path from
  /// the root FileDescriptorProto to the place where the definition.  For
  /// example, this path:
  ///   [ 4, 3, 2, 7, 1 ]
  /// refers to:
  ///   file.message_type(3)  // 4, 3
  ///       .field(7)         // 2, 7
  ///       .name()           // 1
  /// This is because FileDescriptorProto.message_type has field number 4:
  ///   repeated DescriptorProto message_type = 4;
  /// and DescriptorProto.field has field number 2:
  ///   repeated FieldDescriptorProto field = 2;
  /// and FieldDescriptorProto.name has field number 1:
  ///   optional string name = 1;

  /// Thus, the above path gives the location of a field name.  If we removed
  /// the last element:
  ///   [ 4, 3, 2, 7 ]
  /// this path refers to the whole field declaration (from the beginning
  /// of the label to the terminating semicolon).
  pub path: ::std::vec::Vec<i32>,
  /// Always has exactly three or four elements: start line, start column,
  /// end line (optional, otherwise assumed same as start line), end column.
  /// These are packed into a single field for efficiency.  Note that line
  /// and column numbers are zero-based -- typically you will want to add
  /// 1 to each before displaying to a user.
  pub span: ::std::vec::Vec<i32>,
  /// If this SourceCodeInfo represents a complete declaration, these are any
  /// comments appearing before and after the declaration which appear to be
  /// attached to the declaration.

  /// A series of line comments appearing on consecutive lines, with no other
  /// tokens appearing on those lines, will be treated as a single comment.

  /// leading_detached_comments will keep paragraphs of comments that appear
  /// before (but not connected to) the current element. Each paragraph,
  /// separated by empty lines, will be one comment element in the repeated
  /// field.

  /// Only the comment content is provided; comment markers (e.g. //) are
  /// stripped out.  For block comments, leading whitespace and an asterisk
  /// will be stripped from the beginning of each line other than the first.
  /// Newlines are included in the output.

  /// Examples:

  ///   optional int32 foo = 1;  // Comment attached to foo.
  ///   // Comment attached to bar.
  ///   optional int32 bar = 2;

  ///   optional string baz = 3;
  ///   // Comment attached to baz.
  ///   // Another line attached to baz.

  ///   // Comment attached to qux.
  ///   //
  ///   // Another line attached to qux.
  ///   optional double qux = 4;

  ///   // Detached comment for corge. This is not leading or trailing comments
  ///   // to qux or corge because there are blank lines separating it from
  ///   // both.

  ///   // Detached comment for corge paragraph 2.

  ///   optional string corge = 5;
  ///   /* Block comment attached
  ///    * to corge.  Leading asterisks
  ///    * will be removed. */
  ///   /* Block comment attached to
  ///    * grault. */
  ///   optional int32 grault = 6;

  ///   // ignored detached comments.
  pub leading_comments: ::std::option::Option<::std::string::String>,
  pub trailing_comments: ::std::option::Option<::std::string::String>,
  pub leading_detached_comments: ::std::vec::Vec<::std::string::String>,
}
impl SourceCodeInfo_Location {
  pub fn set_path(&mut self, v: ::std::vec::Vec<i32>) {
    self.path = v;
  }
  pub fn take_path(&mut self) -> ::std::vec::Vec<i32> {
    ::std::mem::replace(&mut self.path, ::std::vec::Vec::new())
  }
  pub fn get_path(&self) -> &[i32] {
    &self.path
  }
  pub fn mut_path(&mut self) -> &mut ::std::vec::Vec<i32> {
    &mut self.path
  }
  pub fn set_span(&mut self, v: ::std::vec::Vec<i32>) {
    self.span = v;
  }
  pub fn take_span(&mut self) -> ::std::vec::Vec<i32> {
    ::std::mem::replace(&mut self.span, ::std::vec::Vec::new())
  }
  pub fn get_span(&self) -> &[i32] {
    &self.span
  }
  pub fn mut_span(&mut self) -> &mut ::std::vec::Vec<i32> {
    &mut self.span
  }
  pub fn has_leading_comments(&self) -> bool {
    self.leading_comments.is_some()
  }
  pub fn set_leading_comments(&mut self, v: ::std::string::String) {
    self.leading_comments = Some(v);
  }
  pub fn take_leading_comments(&mut self) -> ::std::string::String {
    self.leading_comments.take().unwrap_or_default()
  }
  pub fn get_leading_comments(&self) -> &str {
    self.leading_comments.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_trailing_comments(&self) -> bool {
    self.trailing_comments.is_some()
  }
  pub fn set_trailing_comments(&mut self, v: ::std::string::String) {
    self.trailing_comments = Some(v);
  }
  pub fn take_trailing_comments(&mut self) -> ::std::string::String {
    self.trailing_comments.take().unwrap_or_default()
  }
  pub fn get_trailing_comments(&self) -> &str {
    self.trailing_comments.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn set_leading_detached_comments(&mut self, v: ::std::vec::Vec<::std::string::String>) {
    self.leading_detached_comments = v;
  }
  pub fn take_leading_detached_comments(&mut self) -> ::std::vec::Vec<::std::string::String> {
    ::std::mem::replace(&mut self.leading_detached_comments, ::std::vec::Vec::new())
  }
  pub fn get_leading_detached_comments(&self) -> &[::std::string::String] {
    &self.leading_detached_comments
  }
  pub fn mut_leading_detached_comments(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
    &mut self.leading_detached_comments
  }
}
impl ::std::default::Default for SourceCodeInfo_Location {
  fn default() -> Self {
    SourceCodeInfo_Location {
      path: ::std::default::Default::default(),
      span: ::std::default::Default::default(),
      leading_comments: ::std::default::Default::default(),
      trailing_comments: ::std::default::Default::default(),
      leading_detached_comments: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SourceCodeInfo_Location_default: SourceCodeInfo_Location = SourceCodeInfo_Location::default();
}
impl ::pb::Message for SourceCodeInfo_Location {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut path_size = 0;
    for val in &self.path {
      let l = ::pb::Message::compute_size(val);
      path_size += l;
    }
    if !self.path.is_empty() {
      size += ::pb::wire_format::serialized_length(1);
      size += ::pb::varint::serialized_length(path_size as u64);
    }
    size += path_size;
    let mut span_size = 0;
    for val in &self.span {
      let l = ::pb::Message::compute_size(val);
      span_size += l;
    }
    if !self.span.is_empty() {
      size += ::pb::wire_format::serialized_length(2);
      size += ::pb::varint::serialized_length(span_size as u64);
    }
    size += span_size;
    let mut leading_comments_size = 0;
    for val in &self.leading_comments {
      let l = ::pb::Message::compute_size(val);
      leading_comments_size += ::pb::wire_format::serialized_length(3);
      leading_comments_size += ::pb::varint::serialized_length(l as u64);
      leading_comments_size += l;
    }
    size += leading_comments_size;
    let mut trailing_comments_size = 0;
    for val in &self.trailing_comments {
      let l = ::pb::Message::compute_size(val);
      trailing_comments_size += ::pb::wire_format::serialized_length(4);
      trailing_comments_size += ::pb::varint::serialized_length(l as u64);
      trailing_comments_size += l;
    }
    size += trailing_comments_size;
    let mut leading_detached_comments_size = 0;
    for val in &self.leading_detached_comments {
      let l = ::pb::Message::compute_size(val);
      leading_detached_comments_size += ::pb::wire_format::serialized_length(6);
      leading_detached_comments_size += ::pb::varint::serialized_length(l as u64);
      leading_detached_comments_size += l;
    }
    size += leading_detached_comments_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.path {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.span {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.leading_comments {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.trailing_comments {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.leading_detached_comments {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.path.is_empty() {
      let mut size = 0;
      for val in &self.path {
        size += ::pb::Message::compute_size(val);
      }
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      ::pb::varint::write(size as u64, w)?;
    }
    for val in &self.path {
      ::pb::Message::serialize(val, w)?;
    }
    if !self.span.is_empty() {
      let mut size = 0;
      for val in &self.span {
        size += ::pb::Message::compute_size(val);
      }
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      ::pb::varint::write(size as u64, w)?;
    }
    for val in &self.span {
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.leading_comments {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.trailing_comments {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.leading_detached_comments {
      ::pb::wire_format::write(6, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          match typ {
            ::pb::wire_format::Type::LengthDelimited => {
              let len = ::pb::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: i32 = ::std::default::Default::default();
                ::pb::Message::deserialize(&mut val, &mut vals)?;
                self.path.push(val);
              }
            }
            _ => {
              ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "SourceCodeInfo_Location", 1)?;
              let mut val: i32 = ::std::default::Default::default();
              ::pb::Message::deserialize(&mut val, buf)?;
              self.path.push(val);
            }
          }
        }
        2 => {
          match typ {
            ::pb::wire_format::Type::LengthDelimited => {
              let len = ::pb::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: i32 = ::std::default::Default::default();
                ::pb::Message::deserialize(&mut val, &mut vals)?;
                self.span.push(val);
              }
            }
            _ => {
              ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "SourceCodeInfo_Location", 2)?;
              let mut val: i32 = ::std::default::Default::default();
              ::pb::Message::deserialize(&mut val, buf)?;
              self.span.push(val);
            }
          }
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "SourceCodeInfo_Location", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.leading_comments = Some(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "SourceCodeInfo_Location", 4)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.trailing_comments = Some(val);
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "SourceCodeInfo_Location", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.leading_detached_comments.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for SourceCodeInfo_Location {
  const NAME: &'static str = "SourceCodeInfo_Location";
  const FULL_NAME: &'static str = "google.protobuf.SourceCodeInfo_Location";
}

/// Describes the relationship between generated code and its original source
/// file. A GeneratedCodeInfo message is associated with only one generated
/// source file, but may contain references to different source .proto files.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeneratedCodeInfo {
  /// An Annotation connects some span of text in generated code to an element
  /// of its generating .proto file.
  pub annotation: ::std::vec::Vec<GeneratedCodeInfo_Annotation>,
}
impl GeneratedCodeInfo {
  pub fn set_annotation(&mut self, v: ::std::vec::Vec<GeneratedCodeInfo_Annotation>) {
    self.annotation = v;
  }
  pub fn take_annotation(&mut self) -> ::std::vec::Vec<GeneratedCodeInfo_Annotation> {
    ::std::mem::replace(&mut self.annotation, ::std::vec::Vec::new())
  }
  pub fn get_annotation(&self) -> &[GeneratedCodeInfo_Annotation] {
    &self.annotation
  }
  pub fn mut_annotation(&mut self) -> &mut ::std::vec::Vec<GeneratedCodeInfo_Annotation> {
    &mut self.annotation
  }
}
impl ::std::default::Default for GeneratedCodeInfo {
  fn default() -> Self {
    GeneratedCodeInfo {
      annotation: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref GeneratedCodeInfo_default: GeneratedCodeInfo = GeneratedCodeInfo::default();
}
impl ::pb::Message for GeneratedCodeInfo {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut annotation_size = 0;
    for val in &self.annotation {
      let l = ::pb::Message::compute_size(val);
      annotation_size += ::pb::wire_format::serialized_length(1);
      annotation_size += ::pb::varint::serialized_length(l as u64);
      annotation_size += l;
    }
    size += annotation_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.annotation {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.annotation {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "GeneratedCodeInfo", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: GeneratedCodeInfo_Annotation = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.annotation.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for GeneratedCodeInfo {
  const NAME: &'static str = "GeneratedCodeInfo";
  const FULL_NAME: &'static str = "google.protobuf.GeneratedCodeInfo";
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GeneratedCodeInfo_Annotation {
  /// Identifies the element in the original source .proto file. This field
  /// is formatted the same as SourceCodeInfo.Location.path.
  pub path: ::std::vec::Vec<i32>,
  /// Identifies the filesystem path to the original source .proto.
  pub source_file: ::std::option::Option<::std::string::String>,
  /// Identifies the starting offset in bytes in the generated code
  /// that relates to the identified object.
  pub begin: ::std::option::Option<i32>,
  /// Identifies the ending offset in bytes in the generated code that
  /// relates to the identified offset. The end offset should be one past
  /// the last relevant byte (so the length of the text = end - begin).
  pub end: ::std::option::Option<i32>,
}
impl GeneratedCodeInfo_Annotation {
  pub fn set_path(&mut self, v: ::std::vec::Vec<i32>) {
    self.path = v;
  }
  pub fn take_path(&mut self) -> ::std::vec::Vec<i32> {
    ::std::mem::replace(&mut self.path, ::std::vec::Vec::new())
  }
  pub fn get_path(&self) -> &[i32] {
    &self.path
  }
  pub fn mut_path(&mut self) -> &mut ::std::vec::Vec<i32> {
    &mut self.path
  }
  pub fn has_source_file(&self) -> bool {
    self.source_file.is_some()
  }
  pub fn set_source_file(&mut self, v: ::std::string::String) {
    self.source_file = Some(v);
  }
  pub fn take_source_file(&mut self) -> ::std::string::String {
    self.source_file.take().unwrap_or_default()
  }
  pub fn get_source_file(&self) -> &str {
    self.source_file.as_ref().map(|ref s| s.as_str()).unwrap_or("")
  }
  pub fn has_begin(&self) -> bool {
    self.begin.is_some()
  }
  pub fn set_begin(&mut self, v: i32) {
    self.begin = Some(v);
  }
  pub fn get_begin(&self) -> i32 {
    self.begin.unwrap_or(0)
  }
  pub fn has_end(&self) -> bool {
    self.end.is_some()
  }
  pub fn set_end(&mut self, v: i32) {
    self.end = Some(v);
  }
  pub fn get_end(&self) -> i32 {
    self.end.unwrap_or(0)
  }
}
impl ::std::default::Default for GeneratedCodeInfo_Annotation {
  fn default() -> Self {
    GeneratedCodeInfo_Annotation {
      path: ::std::default::Default::default(),
      source_file: ::std::default::Default::default(),
      begin: ::std::default::Default::default(),
      end: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref GeneratedCodeInfo_Annotation_default: GeneratedCodeInfo_Annotation = GeneratedCodeInfo_Annotation::default();
}
impl ::pb::Message for GeneratedCodeInfo_Annotation {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut path_size = 0;
    for val in &self.path {
      let l = ::pb::Message::compute_size(val);
      path_size += l;
    }
    if !self.path.is_empty() {
      size += ::pb::wire_format::serialized_length(1);
      size += ::pb::varint::serialized_length(path_size as u64);
    }
    size += path_size;
    let mut source_file_size = 0;
    for val in &self.source_file {
      let l = ::pb::Message::compute_size(val);
      source_file_size += ::pb::wire_format::serialized_length(2);
      source_file_size += ::pb::varint::serialized_length(l as u64);
      source_file_size += l;
    }
    size += source_file_size;
    let mut begin_size = 0;
    for val in &self.begin {
      let l = ::pb::Message::compute_size(val);
      begin_size += ::pb::wire_format::serialized_length(3);
      begin_size += l;
    }
    size += begin_size;
    let mut end_size = 0;
    for val in &self.end {
      let l = ::pb::Message::compute_size(val);
      end_size += ::pb::wire_format::serialized_length(4);
      end_size += l;
    }
    size += end_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.path {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.source_file {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.begin {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.end {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.path.is_empty() {
      let mut size = 0;
      for val in &self.path {
        size += ::pb::Message::compute_size(val);
      }
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      ::pb::varint::write(size as u64, w)?;
    }
    for val in &self.path {
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.source_file {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.begin {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.end {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          match typ {
            ::pb::wire_format::Type::LengthDelimited => {
              let len = ::pb::varint::ensure_read(&mut buf)?;
              let mut vals = ::pb::ensure_split(buf, len as usize)?;
              while vals.has_remaining() {
                let mut val: i32 = ::std::default::Default::default();
                ::pb::Message::deserialize(&mut val, &mut vals)?;
                self.path.push(val);
              }
            }
            _ => {
              ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "GeneratedCodeInfo_Annotation", 1)?;
              let mut val: i32 = ::std::default::Default::default();
              ::pb::Message::deserialize(&mut val, buf)?;
              self.path.push(val);
            }
          }
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "GeneratedCodeInfo_Annotation", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.source_file = Some(val);
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "GeneratedCodeInfo_Annotation", 3)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.begin = Some(val);
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "GeneratedCodeInfo_Annotation", 4)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.end = Some(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for GeneratedCodeInfo_Annotation {
  const NAME: &'static str = "GeneratedCodeInfo_Annotation";
  const FULL_NAME: &'static str = "google.protobuf.GeneratedCodeInfo_Annotation";
}

