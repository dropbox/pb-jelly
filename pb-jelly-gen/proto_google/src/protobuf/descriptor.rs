// @generated, do not edit
/// The full set of known editions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Edition(i32);
impl Edition {
  /// A placeholder for an unknown edition value.
  pub const EDITION_UNKNOWN: Edition = Edition(0);
  /// Legacy syntax "editions".  These pre-date editions, but behave much like
  /// distinct editions.  These can't be used to specify the edition of proto
  /// files, but feature definitions must supply proto2/proto3 defaults for
  /// backwards compatibility.
  pub const EDITION_PROTO2: Edition = Edition(998);
  pub const EDITION_PROTO3: Edition = Edition(999);
  /// Editions that have been released.  The specific values are arbitrary and
  /// should not be depended on, but they will always be time-ordered for easy
  /// comparison.
  pub const EDITION_2023: Edition = Edition(1000);
  /// Placeholder editions for testing feature resolution.  These should not be
  /// used or relyed on outside of tests.
  pub const EDITION_1_TEST_ONLY: Edition = Edition(1);
  pub const EDITION_2_TEST_ONLY: Edition = Edition(2);
  pub const EDITION_99997_TEST_ONLY: Edition = Edition(99997);
  pub const EDITION_99998_TEST_ONLY: Edition = Edition(99998);
  pub const EDITION_99999_TEST_ONLY: Edition = Edition(99999);
  pub const KNOWN_VARIANTS: [Edition; 9] = [Edition::EDITION_UNKNOWN, Edition::EDITION_PROTO2, Edition::EDITION_PROTO3, Edition::EDITION_2023, Edition::EDITION_1_TEST_ONLY, Edition::EDITION_2_TEST_ONLY, Edition::EDITION_99997_TEST_ONLY, Edition::EDITION_99998_TEST_ONLY, Edition::EDITION_99999_TEST_ONLY];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for Edition {
  fn default() -> Self {
    Edition::EDITION_UNKNOWN
  }
}
impl From<Edition> for i32 {
  fn from(v: Edition) -> i32 {
    v.0
  }
}
impl From<i32> for Edition {
  fn from(v: i32) -> Edition {
    Edition(v)
  }
}
impl From<Edition_Closed> for Edition {
  fn from(v: Edition_Closed) -> Edition {
    Edition(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for Edition {
}
impl ::pb_jelly::OpenProtoEnum for Edition {
  type Closed = Edition_Closed;
  fn into_known(self) -> ::std::option::Option<Edition_Closed> {
    match self {
      Edition::EDITION_UNKNOWN => Some(Edition_Closed::EDITION_UNKNOWN),
      Edition::EDITION_PROTO2 => Some(Edition_Closed::EDITION_PROTO2),
      Edition::EDITION_PROTO3 => Some(Edition_Closed::EDITION_PROTO3),
      Edition::EDITION_2023 => Some(Edition_Closed::EDITION_2023),
      Edition::EDITION_1_TEST_ONLY => Some(Edition_Closed::EDITION_1_TEST_ONLY),
      Edition::EDITION_2_TEST_ONLY => Some(Edition_Closed::EDITION_2_TEST_ONLY),
      Edition::EDITION_99997_TEST_ONLY => Some(Edition_Closed::EDITION_99997_TEST_ONLY),
      Edition::EDITION_99998_TEST_ONLY => Some(Edition_Closed::EDITION_99998_TEST_ONLY),
      Edition::EDITION_99999_TEST_ONLY => Some(Edition_Closed::EDITION_99999_TEST_ONLY),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for Edition {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// The full set of known editions.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum Edition_Closed {
  /// A placeholder for an unknown edition value.
  EDITION_UNKNOWN = 0,
  /// Legacy syntax "editions".  These pre-date editions, but behave much like
  /// distinct editions.  These can't be used to specify the edition of proto
  /// files, but feature definitions must supply proto2/proto3 defaults for
  /// backwards compatibility.
  EDITION_PROTO2 = 998,
  EDITION_PROTO3 = 999,
  /// Editions that have been released.  The specific values are arbitrary and
  /// should not be depended on, but they will always be time-ordered for easy
  /// comparison.
  EDITION_2023 = 1000,
  /// Placeholder editions for testing feature resolution.  These should not be
  /// used or relyed on outside of tests.
  EDITION_1_TEST_ONLY = 1,
  EDITION_2_TEST_ONLY = 2,
  EDITION_99997_TEST_ONLY = 99997,
  EDITION_99998_TEST_ONLY = 99998,
  EDITION_99999_TEST_ONLY = 99999,
}
impl Edition_Closed {
  pub const KNOWN_VARIANTS: [Edition_Closed; 9] = [Edition_Closed::EDITION_UNKNOWN, Edition_Closed::EDITION_PROTO2, Edition_Closed::EDITION_PROTO3, Edition_Closed::EDITION_2023, Edition_Closed::EDITION_1_TEST_ONLY, Edition_Closed::EDITION_2_TEST_ONLY, Edition_Closed::EDITION_99997_TEST_ONLY, Edition_Closed::EDITION_99998_TEST_ONLY, Edition_Closed::EDITION_99999_TEST_ONLY];
}
impl ::std::default::Default for Edition_Closed {
  fn default() -> Self {
    Edition_Closed::EDITION_UNKNOWN
  }
}
impl From<Edition_Closed> for i32 {
  fn from(v: Edition_Closed) -> i32 {
    match v {
      Edition_Closed::EDITION_UNKNOWN => 0,
      Edition_Closed::EDITION_PROTO2 => 998,
      Edition_Closed::EDITION_PROTO3 => 999,
      Edition_Closed::EDITION_2023 => 1000,
      Edition_Closed::EDITION_1_TEST_ONLY => 1,
      Edition_Closed::EDITION_2_TEST_ONLY => 2,
      Edition_Closed::EDITION_99997_TEST_ONLY => 99997,
      Edition_Closed::EDITION_99998_TEST_ONLY => 99998,
      Edition_Closed::EDITION_99999_TEST_ONLY => 99999,
    }
  }
}
impl ::std::convert::TryFrom<i32> for Edition_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(Edition_Closed::EDITION_UNKNOWN),
      998 => Ok(Edition_Closed::EDITION_PROTO2),
      999 => Ok(Edition_Closed::EDITION_PROTO3),
      1000 => Ok(Edition_Closed::EDITION_2023),
      1 => Ok(Edition_Closed::EDITION_1_TEST_ONLY),
      2 => Ok(Edition_Closed::EDITION_2_TEST_ONLY),
      99997 => Ok(Edition_Closed::EDITION_99997_TEST_ONLY),
      99998 => Ok(Edition_Closed::EDITION_99998_TEST_ONLY),
      99999 => Ok(Edition_Closed::EDITION_99999_TEST_ONLY),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for Edition_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for Edition_Closed {
  fn name(self) -> &'static str {
    match self {
      Edition_Closed::EDITION_UNKNOWN => "EDITION_UNKNOWN",
      Edition_Closed::EDITION_PROTO2 => "EDITION_PROTO2",
      Edition_Closed::EDITION_PROTO3 => "EDITION_PROTO3",
      Edition_Closed::EDITION_2023 => "EDITION_2023",
      Edition_Closed::EDITION_1_TEST_ONLY => "EDITION_1_TEST_ONLY",
      Edition_Closed::EDITION_2_TEST_ONLY => "EDITION_2_TEST_ONLY",
      Edition_Closed::EDITION_99997_TEST_ONLY => "EDITION_99997_TEST_ONLY",
      Edition_Closed::EDITION_99998_TEST_ONLY => "EDITION_99998_TEST_ONLY",
      Edition_Closed::EDITION_99999_TEST_ONLY => "EDITION_99999_TEST_ONLY",
    }
  }
}

/// The verification state of the extension range.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ExtensionRangeOptions_VerificationState(i32);
impl ExtensionRangeOptions_VerificationState {
  /// All the extensions of the range must be declared.
  pub const DECLARATION: ExtensionRangeOptions_VerificationState = ExtensionRangeOptions_VerificationState(0);
  pub const UNVERIFIED: ExtensionRangeOptions_VerificationState = ExtensionRangeOptions_VerificationState(1);
  pub const KNOWN_VARIANTS: [ExtensionRangeOptions_VerificationState; 2] = [ExtensionRangeOptions_VerificationState::DECLARATION, ExtensionRangeOptions_VerificationState::UNVERIFIED];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for ExtensionRangeOptions_VerificationState {
  fn default() -> Self {
    ExtensionRangeOptions_VerificationState::DECLARATION
  }
}
impl From<ExtensionRangeOptions_VerificationState> for i32 {
  fn from(v: ExtensionRangeOptions_VerificationState) -> i32 {
    v.0
  }
}
impl From<i32> for ExtensionRangeOptions_VerificationState {
  fn from(v: i32) -> ExtensionRangeOptions_VerificationState {
    ExtensionRangeOptions_VerificationState(v)
  }
}
impl From<ExtensionRangeOptions_VerificationState_Closed> for ExtensionRangeOptions_VerificationState {
  fn from(v: ExtensionRangeOptions_VerificationState_Closed) -> ExtensionRangeOptions_VerificationState {
    ExtensionRangeOptions_VerificationState(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for ExtensionRangeOptions_VerificationState {
}
impl ::pb_jelly::OpenProtoEnum for ExtensionRangeOptions_VerificationState {
  type Closed = ExtensionRangeOptions_VerificationState_Closed;
  fn into_known(self) -> ::std::option::Option<ExtensionRangeOptions_VerificationState_Closed> {
    match self {
      ExtensionRangeOptions_VerificationState::DECLARATION => Some(ExtensionRangeOptions_VerificationState_Closed::DECLARATION),
      ExtensionRangeOptions_VerificationState::UNVERIFIED => Some(ExtensionRangeOptions_VerificationState_Closed::UNVERIFIED),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for ExtensionRangeOptions_VerificationState {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// The verification state of the extension range.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum ExtensionRangeOptions_VerificationState_Closed {
  /// All the extensions of the range must be declared.
  DECLARATION = 0,
  UNVERIFIED = 1,
}
impl ExtensionRangeOptions_VerificationState_Closed {
  pub const KNOWN_VARIANTS: [ExtensionRangeOptions_VerificationState_Closed; 2] = [ExtensionRangeOptions_VerificationState_Closed::DECLARATION, ExtensionRangeOptions_VerificationState_Closed::UNVERIFIED];
}
impl ::std::default::Default for ExtensionRangeOptions_VerificationState_Closed {
  fn default() -> Self {
    ExtensionRangeOptions_VerificationState_Closed::DECLARATION
  }
}
impl From<ExtensionRangeOptions_VerificationState_Closed> for i32 {
  fn from(v: ExtensionRangeOptions_VerificationState_Closed) -> i32 {
    match v {
      ExtensionRangeOptions_VerificationState_Closed::DECLARATION => 0,
      ExtensionRangeOptions_VerificationState_Closed::UNVERIFIED => 1,
    }
  }
}
impl ::std::convert::TryFrom<i32> for ExtensionRangeOptions_VerificationState_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(ExtensionRangeOptions_VerificationState_Closed::DECLARATION),
      1 => Ok(ExtensionRangeOptions_VerificationState_Closed::UNVERIFIED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for ExtensionRangeOptions_VerificationState_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for ExtensionRangeOptions_VerificationState_Closed {
  fn name(self) -> &'static str {
    match self {
      ExtensionRangeOptions_VerificationState_Closed::DECLARATION => "DECLARATION",
      ExtensionRangeOptions_VerificationState_Closed::UNVERIFIED => "UNVERIFIED",
    }
  }
}

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
  /// Group type is deprecated and not supported after google.protobuf. However, Proto3
  /// implementations should still be able to parse the group wire format and
  /// treat group fields as unknown fields.  In Editions, the group wire format
  /// can be enabled via the `message_encoding` feature.
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
impl ::pb_jelly::ProtoEnum for FieldDescriptorProto_Type {
}
impl ::pb_jelly::OpenProtoEnum for FieldDescriptorProto_Type {
  type Closed = FieldDescriptorProto_Type_Closed;
  fn into_known(self) -> ::std::option::Option<FieldDescriptorProto_Type_Closed> {
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
impl ::std::fmt::Debug for FieldDescriptorProto_Type {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
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
  /// Group type is deprecated and not supported after google.protobuf. However, Proto3
  /// implementations should still be able to parse the group wire format and
  /// treat group fields as unknown fields.  In Editions, the group wire format
  /// can be enabled via the `message_encoding` feature.
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
impl ::pb_jelly::ProtoEnum for FieldDescriptorProto_Type_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FieldDescriptorProto_Type_Closed {
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
  pub const LABEL_REPEATED: FieldDescriptorProto_Label = FieldDescriptorProto_Label(3);
  /// The required label is only allowed in google.protobuf.  In proto3 and Editions
  /// it's explicitly prohibited.  In Editions, the `field_presence` feature
  /// can be used to get this behavior.
  pub const LABEL_REQUIRED: FieldDescriptorProto_Label = FieldDescriptorProto_Label(2);
  pub const KNOWN_VARIANTS: [FieldDescriptorProto_Label; 3] = [FieldDescriptorProto_Label::LABEL_OPTIONAL, FieldDescriptorProto_Label::LABEL_REPEATED, FieldDescriptorProto_Label::LABEL_REQUIRED];
  pub const fn value(self) -> i32 {
    self.0
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
impl ::pb_jelly::ProtoEnum for FieldDescriptorProto_Label {
}
impl ::pb_jelly::OpenProtoEnum for FieldDescriptorProto_Label {
  type Closed = FieldDescriptorProto_Label_Closed;
  fn into_known(self) -> ::std::option::Option<FieldDescriptorProto_Label_Closed> {
    match self {
      FieldDescriptorProto_Label::LABEL_OPTIONAL => Some(FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL),
      FieldDescriptorProto_Label::LABEL_REPEATED => Some(FieldDescriptorProto_Label_Closed::LABEL_REPEATED),
      FieldDescriptorProto_Label::LABEL_REQUIRED => Some(FieldDescriptorProto_Label_Closed::LABEL_REQUIRED),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FieldDescriptorProto_Label {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
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
  LABEL_REPEATED = 3,
  /// The required label is only allowed in google.protobuf.  In proto3 and Editions
  /// it's explicitly prohibited.  In Editions, the `field_presence` feature
  /// can be used to get this behavior.
  LABEL_REQUIRED = 2,
}
impl FieldDescriptorProto_Label_Closed {
  pub const KNOWN_VARIANTS: [FieldDescriptorProto_Label_Closed; 3] = [FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL, FieldDescriptorProto_Label_Closed::LABEL_REPEATED, FieldDescriptorProto_Label_Closed::LABEL_REQUIRED];
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
      FieldDescriptorProto_Label_Closed::LABEL_REPEATED => 3,
      FieldDescriptorProto_Label_Closed::LABEL_REQUIRED => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FieldDescriptorProto_Label_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      1 => Ok(FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL),
      3 => Ok(FieldDescriptorProto_Label_Closed::LABEL_REPEATED),
      2 => Ok(FieldDescriptorProto_Label_Closed::LABEL_REQUIRED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FieldDescriptorProto_Label_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FieldDescriptorProto_Label_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldDescriptorProto_Label_Closed::LABEL_OPTIONAL => "LABEL_OPTIONAL",
      FieldDescriptorProto_Label_Closed::LABEL_REPEATED => "LABEL_REPEATED",
      FieldDescriptorProto_Label_Closed::LABEL_REQUIRED => "LABEL_REQUIRED",
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
impl ::pb_jelly::ProtoEnum for FileOptions_OptimizeMode {
}
impl ::pb_jelly::OpenProtoEnum for FileOptions_OptimizeMode {
  type Closed = FileOptions_OptimizeMode_Closed;
  fn into_known(self) -> ::std::option::Option<FileOptions_OptimizeMode_Closed> {
    match self {
      FileOptions_OptimizeMode::SPEED => Some(FileOptions_OptimizeMode_Closed::SPEED),
      FileOptions_OptimizeMode::CODE_SIZE => Some(FileOptions_OptimizeMode_Closed::CODE_SIZE),
      FileOptions_OptimizeMode::LITE_RUNTIME => Some(FileOptions_OptimizeMode_Closed::LITE_RUNTIME),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FileOptions_OptimizeMode {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
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
impl ::pb_jelly::ProtoEnum for FileOptions_OptimizeMode_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FileOptions_OptimizeMode_Closed {
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
  /// The option [ctype=CORD] may be applied to a non-repeated field of type
  /// "bytes". It indicates that in C++, the data should be stored in a Cord
  /// instead of a string.  For very large strings, this may reduce memory
  /// fragmentation. It may also allow better performance when parsing from a
  /// Cord, or when parsing with aliasing enabled, as the parsed Cord may then
  /// alias the original buffer.
  pub const CORD: FieldOptions_CType = FieldOptions_CType(1);
  pub const STRING_PIECE: FieldOptions_CType = FieldOptions_CType(2);
  pub const KNOWN_VARIANTS: [FieldOptions_CType; 3] = [FieldOptions_CType::STRING, FieldOptions_CType::CORD, FieldOptions_CType::STRING_PIECE];
  pub const fn value(self) -> i32 {
    self.0
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
impl ::pb_jelly::ProtoEnum for FieldOptions_CType {
}
impl ::pb_jelly::OpenProtoEnum for FieldOptions_CType {
  type Closed = FieldOptions_CType_Closed;
  fn into_known(self) -> ::std::option::Option<FieldOptions_CType_Closed> {
    match self {
      FieldOptions_CType::STRING => Some(FieldOptions_CType_Closed::STRING),
      FieldOptions_CType::CORD => Some(FieldOptions_CType_Closed::CORD),
      FieldOptions_CType::STRING_PIECE => Some(FieldOptions_CType_Closed::STRING_PIECE),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FieldOptions_CType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
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
  /// The option [ctype=CORD] may be applied to a non-repeated field of type
  /// "bytes". It indicates that in C++, the data should be stored in a Cord
  /// instead of a string.  For very large strings, this may reduce memory
  /// fragmentation. It may also allow better performance when parsing from a
  /// Cord, or when parsing with aliasing enabled, as the parsed Cord may then
  /// alias the original buffer.
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
impl ::pb_jelly::ProtoEnum for FieldOptions_CType_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FieldOptions_CType_Closed {
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
impl ::pb_jelly::ProtoEnum for FieldOptions_JSType {
}
impl ::pb_jelly::OpenProtoEnum for FieldOptions_JSType {
  type Closed = FieldOptions_JSType_Closed;
  fn into_known(self) -> ::std::option::Option<FieldOptions_JSType_Closed> {
    match self {
      FieldOptions_JSType::JS_NORMAL => Some(FieldOptions_JSType_Closed::JS_NORMAL),
      FieldOptions_JSType::JS_STRING => Some(FieldOptions_JSType_Closed::JS_STRING),
      FieldOptions_JSType::JS_NUMBER => Some(FieldOptions_JSType_Closed::JS_NUMBER),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FieldOptions_JSType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
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
impl ::pb_jelly::ProtoEnum for FieldOptions_JSType_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FieldOptions_JSType_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldOptions_JSType_Closed::JS_NORMAL => "JS_NORMAL",
      FieldOptions_JSType_Closed::JS_STRING => "JS_STRING",
      FieldOptions_JSType_Closed::JS_NUMBER => "JS_NUMBER",
    }
  }
}

/// If set to RETENTION_SOURCE, the option will be omitted from the binary.
/// Note: as of January 2023, support for this is in progress and does not yet
/// have an effect (b/264593489).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldOptions_OptionRetention(i32);
impl FieldOptions_OptionRetention {
  pub const RETENTION_UNKNOWN: FieldOptions_OptionRetention = FieldOptions_OptionRetention(0);
  pub const RETENTION_RUNTIME: FieldOptions_OptionRetention = FieldOptions_OptionRetention(1);
  pub const RETENTION_SOURCE: FieldOptions_OptionRetention = FieldOptions_OptionRetention(2);
  pub const KNOWN_VARIANTS: [FieldOptions_OptionRetention; 3] = [FieldOptions_OptionRetention::RETENTION_UNKNOWN, FieldOptions_OptionRetention::RETENTION_RUNTIME, FieldOptions_OptionRetention::RETENTION_SOURCE];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FieldOptions_OptionRetention {
  fn default() -> Self {
    FieldOptions_OptionRetention::RETENTION_UNKNOWN
  }
}
impl From<FieldOptions_OptionRetention> for i32 {
  fn from(v: FieldOptions_OptionRetention) -> i32 {
    v.0
  }
}
impl From<i32> for FieldOptions_OptionRetention {
  fn from(v: i32) -> FieldOptions_OptionRetention {
    FieldOptions_OptionRetention(v)
  }
}
impl From<FieldOptions_OptionRetention_Closed> for FieldOptions_OptionRetention {
  fn from(v: FieldOptions_OptionRetention_Closed) -> FieldOptions_OptionRetention {
    FieldOptions_OptionRetention(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FieldOptions_OptionRetention {
}
impl ::pb_jelly::OpenProtoEnum for FieldOptions_OptionRetention {
  type Closed = FieldOptions_OptionRetention_Closed;
  fn into_known(self) -> ::std::option::Option<FieldOptions_OptionRetention_Closed> {
    match self {
      FieldOptions_OptionRetention::RETENTION_UNKNOWN => Some(FieldOptions_OptionRetention_Closed::RETENTION_UNKNOWN),
      FieldOptions_OptionRetention::RETENTION_RUNTIME => Some(FieldOptions_OptionRetention_Closed::RETENTION_RUNTIME),
      FieldOptions_OptionRetention::RETENTION_SOURCE => Some(FieldOptions_OptionRetention_Closed::RETENTION_SOURCE),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FieldOptions_OptionRetention {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// If set to RETENTION_SOURCE, the option will be omitted from the binary.
/// Note: as of January 2023, support for this is in progress and does not yet
/// have an effect (b/264593489).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FieldOptions_OptionRetention_Closed {
  RETENTION_UNKNOWN = 0,
  RETENTION_RUNTIME = 1,
  RETENTION_SOURCE = 2,
}
impl FieldOptions_OptionRetention_Closed {
  pub const KNOWN_VARIANTS: [FieldOptions_OptionRetention_Closed; 3] = [FieldOptions_OptionRetention_Closed::RETENTION_UNKNOWN, FieldOptions_OptionRetention_Closed::RETENTION_RUNTIME, FieldOptions_OptionRetention_Closed::RETENTION_SOURCE];
}
impl ::std::default::Default for FieldOptions_OptionRetention_Closed {
  fn default() -> Self {
    FieldOptions_OptionRetention_Closed::RETENTION_UNKNOWN
  }
}
impl From<FieldOptions_OptionRetention_Closed> for i32 {
  fn from(v: FieldOptions_OptionRetention_Closed) -> i32 {
    match v {
      FieldOptions_OptionRetention_Closed::RETENTION_UNKNOWN => 0,
      FieldOptions_OptionRetention_Closed::RETENTION_RUNTIME => 1,
      FieldOptions_OptionRetention_Closed::RETENTION_SOURCE => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FieldOptions_OptionRetention_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FieldOptions_OptionRetention_Closed::RETENTION_UNKNOWN),
      1 => Ok(FieldOptions_OptionRetention_Closed::RETENTION_RUNTIME),
      2 => Ok(FieldOptions_OptionRetention_Closed::RETENTION_SOURCE),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FieldOptions_OptionRetention_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FieldOptions_OptionRetention_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldOptions_OptionRetention_Closed::RETENTION_UNKNOWN => "RETENTION_UNKNOWN",
      FieldOptions_OptionRetention_Closed::RETENTION_RUNTIME => "RETENTION_RUNTIME",
      FieldOptions_OptionRetention_Closed::RETENTION_SOURCE => "RETENTION_SOURCE",
    }
  }
}

/// This indicates the types of entities that the field may apply to when used
/// as an option. If it is unset, then the field may be freely used as an
/// option on any kind of entity. Note: as of January 2023, support for this is
/// in progress and does not yet have an effect (b/264593489).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FieldOptions_OptionTargetType(i32);
impl FieldOptions_OptionTargetType {
  pub const TARGET_TYPE_UNKNOWN: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(0);
  pub const TARGET_TYPE_FILE: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(1);
  pub const TARGET_TYPE_EXTENSION_RANGE: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(2);
  pub const TARGET_TYPE_MESSAGE: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(3);
  pub const TARGET_TYPE_FIELD: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(4);
  pub const TARGET_TYPE_ONEOF: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(5);
  pub const TARGET_TYPE_ENUM: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(6);
  pub const TARGET_TYPE_ENUM_ENTRY: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(7);
  pub const TARGET_TYPE_SERVICE: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(8);
  pub const TARGET_TYPE_METHOD: FieldOptions_OptionTargetType = FieldOptions_OptionTargetType(9);
  pub const KNOWN_VARIANTS: [FieldOptions_OptionTargetType; 10] = [FieldOptions_OptionTargetType::TARGET_TYPE_UNKNOWN, FieldOptions_OptionTargetType::TARGET_TYPE_FILE, FieldOptions_OptionTargetType::TARGET_TYPE_EXTENSION_RANGE, FieldOptions_OptionTargetType::TARGET_TYPE_MESSAGE, FieldOptions_OptionTargetType::TARGET_TYPE_FIELD, FieldOptions_OptionTargetType::TARGET_TYPE_ONEOF, FieldOptions_OptionTargetType::TARGET_TYPE_ENUM, FieldOptions_OptionTargetType::TARGET_TYPE_ENUM_ENTRY, FieldOptions_OptionTargetType::TARGET_TYPE_SERVICE, FieldOptions_OptionTargetType::TARGET_TYPE_METHOD];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FieldOptions_OptionTargetType {
  fn default() -> Self {
    FieldOptions_OptionTargetType::TARGET_TYPE_UNKNOWN
  }
}
impl From<FieldOptions_OptionTargetType> for i32 {
  fn from(v: FieldOptions_OptionTargetType) -> i32 {
    v.0
  }
}
impl From<i32> for FieldOptions_OptionTargetType {
  fn from(v: i32) -> FieldOptions_OptionTargetType {
    FieldOptions_OptionTargetType(v)
  }
}
impl From<FieldOptions_OptionTargetType_Closed> for FieldOptions_OptionTargetType {
  fn from(v: FieldOptions_OptionTargetType_Closed) -> FieldOptions_OptionTargetType {
    FieldOptions_OptionTargetType(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FieldOptions_OptionTargetType {
}
impl ::pb_jelly::OpenProtoEnum for FieldOptions_OptionTargetType {
  type Closed = FieldOptions_OptionTargetType_Closed;
  fn into_known(self) -> ::std::option::Option<FieldOptions_OptionTargetType_Closed> {
    match self {
      FieldOptions_OptionTargetType::TARGET_TYPE_UNKNOWN => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_UNKNOWN),
      FieldOptions_OptionTargetType::TARGET_TYPE_FILE => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FILE),
      FieldOptions_OptionTargetType::TARGET_TYPE_EXTENSION_RANGE => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_EXTENSION_RANGE),
      FieldOptions_OptionTargetType::TARGET_TYPE_MESSAGE => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_MESSAGE),
      FieldOptions_OptionTargetType::TARGET_TYPE_FIELD => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FIELD),
      FieldOptions_OptionTargetType::TARGET_TYPE_ONEOF => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ONEOF),
      FieldOptions_OptionTargetType::TARGET_TYPE_ENUM => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM),
      FieldOptions_OptionTargetType::TARGET_TYPE_ENUM_ENTRY => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM_ENTRY),
      FieldOptions_OptionTargetType::TARGET_TYPE_SERVICE => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_SERVICE),
      FieldOptions_OptionTargetType::TARGET_TYPE_METHOD => Some(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_METHOD),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FieldOptions_OptionTargetType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// This indicates the types of entities that the field may apply to when used
/// as an option. If it is unset, then the field may be freely used as an
/// option on any kind of entity. Note: as of January 2023, support for this is
/// in progress and does not yet have an effect (b/264593489).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FieldOptions_OptionTargetType_Closed {
  TARGET_TYPE_UNKNOWN = 0,
  TARGET_TYPE_FILE = 1,
  TARGET_TYPE_EXTENSION_RANGE = 2,
  TARGET_TYPE_MESSAGE = 3,
  TARGET_TYPE_FIELD = 4,
  TARGET_TYPE_ONEOF = 5,
  TARGET_TYPE_ENUM = 6,
  TARGET_TYPE_ENUM_ENTRY = 7,
  TARGET_TYPE_SERVICE = 8,
  TARGET_TYPE_METHOD = 9,
}
impl FieldOptions_OptionTargetType_Closed {
  pub const KNOWN_VARIANTS: [FieldOptions_OptionTargetType_Closed; 10] = [FieldOptions_OptionTargetType_Closed::TARGET_TYPE_UNKNOWN, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FILE, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_EXTENSION_RANGE, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_MESSAGE, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FIELD, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ONEOF, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM_ENTRY, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_SERVICE, FieldOptions_OptionTargetType_Closed::TARGET_TYPE_METHOD];
}
impl ::std::default::Default for FieldOptions_OptionTargetType_Closed {
  fn default() -> Self {
    FieldOptions_OptionTargetType_Closed::TARGET_TYPE_UNKNOWN
  }
}
impl From<FieldOptions_OptionTargetType_Closed> for i32 {
  fn from(v: FieldOptions_OptionTargetType_Closed) -> i32 {
    match v {
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_UNKNOWN => 0,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FILE => 1,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_EXTENSION_RANGE => 2,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_MESSAGE => 3,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FIELD => 4,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ONEOF => 5,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM => 6,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM_ENTRY => 7,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_SERVICE => 8,
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_METHOD => 9,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FieldOptions_OptionTargetType_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_UNKNOWN),
      1 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FILE),
      2 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_EXTENSION_RANGE),
      3 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_MESSAGE),
      4 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FIELD),
      5 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ONEOF),
      6 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM),
      7 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM_ENTRY),
      8 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_SERVICE),
      9 => Ok(FieldOptions_OptionTargetType_Closed::TARGET_TYPE_METHOD),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FieldOptions_OptionTargetType_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FieldOptions_OptionTargetType_Closed {
  fn name(self) -> &'static str {
    match self {
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_UNKNOWN => "TARGET_TYPE_UNKNOWN",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FILE => "TARGET_TYPE_FILE",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_EXTENSION_RANGE => "TARGET_TYPE_EXTENSION_RANGE",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_MESSAGE => "TARGET_TYPE_MESSAGE",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_FIELD => "TARGET_TYPE_FIELD",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ONEOF => "TARGET_TYPE_ONEOF",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM => "TARGET_TYPE_ENUM",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_ENUM_ENTRY => "TARGET_TYPE_ENUM_ENTRY",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_SERVICE => "TARGET_TYPE_SERVICE",
      FieldOptions_OptionTargetType_Closed::TARGET_TYPE_METHOD => "TARGET_TYPE_METHOD",
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
impl ::pb_jelly::ProtoEnum for MethodOptions_IdempotencyLevel {
}
impl ::pb_jelly::OpenProtoEnum for MethodOptions_IdempotencyLevel {
  type Closed = MethodOptions_IdempotencyLevel_Closed;
  fn into_known(self) -> ::std::option::Option<MethodOptions_IdempotencyLevel_Closed> {
    match self {
      MethodOptions_IdempotencyLevel::IDEMPOTENCY_UNKNOWN => Some(MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN),
      MethodOptions_IdempotencyLevel::NO_SIDE_EFFECTS => Some(MethodOptions_IdempotencyLevel_Closed::NO_SIDE_EFFECTS),
      MethodOptions_IdempotencyLevel::IDEMPOTENT => Some(MethodOptions_IdempotencyLevel_Closed::IDEMPOTENT),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for MethodOptions_IdempotencyLevel {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
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
impl ::pb_jelly::ProtoEnum for MethodOptions_IdempotencyLevel_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for MethodOptions_IdempotencyLevel_Closed {
  fn name(self) -> &'static str {
    match self {
      MethodOptions_IdempotencyLevel_Closed::IDEMPOTENCY_UNKNOWN => "IDEMPOTENCY_UNKNOWN",
      MethodOptions_IdempotencyLevel_Closed::NO_SIDE_EFFECTS => "NO_SIDE_EFFECTS",
      MethodOptions_IdempotencyLevel_Closed::IDEMPOTENT => "IDEMPOTENT",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FeatureSet_FieldPresence(i32);
impl FeatureSet_FieldPresence {
  pub const FIELD_PRESENCE_UNKNOWN: FeatureSet_FieldPresence = FeatureSet_FieldPresence(0);
  pub const EXPLICIT: FeatureSet_FieldPresence = FeatureSet_FieldPresence(1);
  pub const IMPLICIT: FeatureSet_FieldPresence = FeatureSet_FieldPresence(2);
  pub const LEGACY_REQUIRED: FeatureSet_FieldPresence = FeatureSet_FieldPresence(3);
  pub const KNOWN_VARIANTS: [FeatureSet_FieldPresence; 4] = [FeatureSet_FieldPresence::FIELD_PRESENCE_UNKNOWN, FeatureSet_FieldPresence::EXPLICIT, FeatureSet_FieldPresence::IMPLICIT, FeatureSet_FieldPresence::LEGACY_REQUIRED];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FeatureSet_FieldPresence {
  fn default() -> Self {
    FeatureSet_FieldPresence::FIELD_PRESENCE_UNKNOWN
  }
}
impl From<FeatureSet_FieldPresence> for i32 {
  fn from(v: FeatureSet_FieldPresence) -> i32 {
    v.0
  }
}
impl From<i32> for FeatureSet_FieldPresence {
  fn from(v: i32) -> FeatureSet_FieldPresence {
    FeatureSet_FieldPresence(v)
  }
}
impl From<FeatureSet_FieldPresence_Closed> for FeatureSet_FieldPresence {
  fn from(v: FeatureSet_FieldPresence_Closed) -> FeatureSet_FieldPresence {
    FeatureSet_FieldPresence(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_FieldPresence {
}
impl ::pb_jelly::OpenProtoEnum for FeatureSet_FieldPresence {
  type Closed = FeatureSet_FieldPresence_Closed;
  fn into_known(self) -> ::std::option::Option<FeatureSet_FieldPresence_Closed> {
    match self {
      FeatureSet_FieldPresence::FIELD_PRESENCE_UNKNOWN => Some(FeatureSet_FieldPresence_Closed::FIELD_PRESENCE_UNKNOWN),
      FeatureSet_FieldPresence::EXPLICIT => Some(FeatureSet_FieldPresence_Closed::EXPLICIT),
      FeatureSet_FieldPresence::IMPLICIT => Some(FeatureSet_FieldPresence_Closed::IMPLICIT),
      FeatureSet_FieldPresence::LEGACY_REQUIRED => Some(FeatureSet_FieldPresence_Closed::LEGACY_REQUIRED),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FeatureSet_FieldPresence {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FeatureSet_FieldPresence_Closed {
  FIELD_PRESENCE_UNKNOWN = 0,
  EXPLICIT = 1,
  IMPLICIT = 2,
  LEGACY_REQUIRED = 3,
}
impl FeatureSet_FieldPresence_Closed {
  pub const KNOWN_VARIANTS: [FeatureSet_FieldPresence_Closed; 4] = [FeatureSet_FieldPresence_Closed::FIELD_PRESENCE_UNKNOWN, FeatureSet_FieldPresence_Closed::EXPLICIT, FeatureSet_FieldPresence_Closed::IMPLICIT, FeatureSet_FieldPresence_Closed::LEGACY_REQUIRED];
}
impl ::std::default::Default for FeatureSet_FieldPresence_Closed {
  fn default() -> Self {
    FeatureSet_FieldPresence_Closed::FIELD_PRESENCE_UNKNOWN
  }
}
impl From<FeatureSet_FieldPresence_Closed> for i32 {
  fn from(v: FeatureSet_FieldPresence_Closed) -> i32 {
    match v {
      FeatureSet_FieldPresence_Closed::FIELD_PRESENCE_UNKNOWN => 0,
      FeatureSet_FieldPresence_Closed::EXPLICIT => 1,
      FeatureSet_FieldPresence_Closed::IMPLICIT => 2,
      FeatureSet_FieldPresence_Closed::LEGACY_REQUIRED => 3,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FeatureSet_FieldPresence_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FeatureSet_FieldPresence_Closed::FIELD_PRESENCE_UNKNOWN),
      1 => Ok(FeatureSet_FieldPresence_Closed::EXPLICIT),
      2 => Ok(FeatureSet_FieldPresence_Closed::IMPLICIT),
      3 => Ok(FeatureSet_FieldPresence_Closed::LEGACY_REQUIRED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_FieldPresence_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FeatureSet_FieldPresence_Closed {
  fn name(self) -> &'static str {
    match self {
      FeatureSet_FieldPresence_Closed::FIELD_PRESENCE_UNKNOWN => "FIELD_PRESENCE_UNKNOWN",
      FeatureSet_FieldPresence_Closed::EXPLICIT => "EXPLICIT",
      FeatureSet_FieldPresence_Closed::IMPLICIT => "IMPLICIT",
      FeatureSet_FieldPresence_Closed::LEGACY_REQUIRED => "LEGACY_REQUIRED",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FeatureSet_EnumType(i32);
impl FeatureSet_EnumType {
  pub const ENUM_TYPE_UNKNOWN: FeatureSet_EnumType = FeatureSet_EnumType(0);
  pub const OPEN: FeatureSet_EnumType = FeatureSet_EnumType(1);
  pub const CLOSED: FeatureSet_EnumType = FeatureSet_EnumType(2);
  pub const KNOWN_VARIANTS: [FeatureSet_EnumType; 3] = [FeatureSet_EnumType::ENUM_TYPE_UNKNOWN, FeatureSet_EnumType::OPEN, FeatureSet_EnumType::CLOSED];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FeatureSet_EnumType {
  fn default() -> Self {
    FeatureSet_EnumType::ENUM_TYPE_UNKNOWN
  }
}
impl From<FeatureSet_EnumType> for i32 {
  fn from(v: FeatureSet_EnumType) -> i32 {
    v.0
  }
}
impl From<i32> for FeatureSet_EnumType {
  fn from(v: i32) -> FeatureSet_EnumType {
    FeatureSet_EnumType(v)
  }
}
impl From<FeatureSet_EnumType_Closed> for FeatureSet_EnumType {
  fn from(v: FeatureSet_EnumType_Closed) -> FeatureSet_EnumType {
    FeatureSet_EnumType(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_EnumType {
}
impl ::pb_jelly::OpenProtoEnum for FeatureSet_EnumType {
  type Closed = FeatureSet_EnumType_Closed;
  fn into_known(self) -> ::std::option::Option<FeatureSet_EnumType_Closed> {
    match self {
      FeatureSet_EnumType::ENUM_TYPE_UNKNOWN => Some(FeatureSet_EnumType_Closed::ENUM_TYPE_UNKNOWN),
      FeatureSet_EnumType::OPEN => Some(FeatureSet_EnumType_Closed::OPEN),
      FeatureSet_EnumType::CLOSED => Some(FeatureSet_EnumType_Closed::CLOSED),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FeatureSet_EnumType {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FeatureSet_EnumType_Closed {
  ENUM_TYPE_UNKNOWN = 0,
  OPEN = 1,
  CLOSED = 2,
}
impl FeatureSet_EnumType_Closed {
  pub const KNOWN_VARIANTS: [FeatureSet_EnumType_Closed; 3] = [FeatureSet_EnumType_Closed::ENUM_TYPE_UNKNOWN, FeatureSet_EnumType_Closed::OPEN, FeatureSet_EnumType_Closed::CLOSED];
}
impl ::std::default::Default for FeatureSet_EnumType_Closed {
  fn default() -> Self {
    FeatureSet_EnumType_Closed::ENUM_TYPE_UNKNOWN
  }
}
impl From<FeatureSet_EnumType_Closed> for i32 {
  fn from(v: FeatureSet_EnumType_Closed) -> i32 {
    match v {
      FeatureSet_EnumType_Closed::ENUM_TYPE_UNKNOWN => 0,
      FeatureSet_EnumType_Closed::OPEN => 1,
      FeatureSet_EnumType_Closed::CLOSED => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FeatureSet_EnumType_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FeatureSet_EnumType_Closed::ENUM_TYPE_UNKNOWN),
      1 => Ok(FeatureSet_EnumType_Closed::OPEN),
      2 => Ok(FeatureSet_EnumType_Closed::CLOSED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_EnumType_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FeatureSet_EnumType_Closed {
  fn name(self) -> &'static str {
    match self {
      FeatureSet_EnumType_Closed::ENUM_TYPE_UNKNOWN => "ENUM_TYPE_UNKNOWN",
      FeatureSet_EnumType_Closed::OPEN => "OPEN",
      FeatureSet_EnumType_Closed::CLOSED => "CLOSED",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FeatureSet_RepeatedFieldEncoding(i32);
impl FeatureSet_RepeatedFieldEncoding {
  pub const REPEATED_FIELD_ENCODING_UNKNOWN: FeatureSet_RepeatedFieldEncoding = FeatureSet_RepeatedFieldEncoding(0);
  pub const PACKED: FeatureSet_RepeatedFieldEncoding = FeatureSet_RepeatedFieldEncoding(1);
  pub const EXPANDED: FeatureSet_RepeatedFieldEncoding = FeatureSet_RepeatedFieldEncoding(2);
  pub const KNOWN_VARIANTS: [FeatureSet_RepeatedFieldEncoding; 3] = [FeatureSet_RepeatedFieldEncoding::REPEATED_FIELD_ENCODING_UNKNOWN, FeatureSet_RepeatedFieldEncoding::PACKED, FeatureSet_RepeatedFieldEncoding::EXPANDED];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FeatureSet_RepeatedFieldEncoding {
  fn default() -> Self {
    FeatureSet_RepeatedFieldEncoding::REPEATED_FIELD_ENCODING_UNKNOWN
  }
}
impl From<FeatureSet_RepeatedFieldEncoding> for i32 {
  fn from(v: FeatureSet_RepeatedFieldEncoding) -> i32 {
    v.0
  }
}
impl From<i32> for FeatureSet_RepeatedFieldEncoding {
  fn from(v: i32) -> FeatureSet_RepeatedFieldEncoding {
    FeatureSet_RepeatedFieldEncoding(v)
  }
}
impl From<FeatureSet_RepeatedFieldEncoding_Closed> for FeatureSet_RepeatedFieldEncoding {
  fn from(v: FeatureSet_RepeatedFieldEncoding_Closed) -> FeatureSet_RepeatedFieldEncoding {
    FeatureSet_RepeatedFieldEncoding(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_RepeatedFieldEncoding {
}
impl ::pb_jelly::OpenProtoEnum for FeatureSet_RepeatedFieldEncoding {
  type Closed = FeatureSet_RepeatedFieldEncoding_Closed;
  fn into_known(self) -> ::std::option::Option<FeatureSet_RepeatedFieldEncoding_Closed> {
    match self {
      FeatureSet_RepeatedFieldEncoding::REPEATED_FIELD_ENCODING_UNKNOWN => Some(FeatureSet_RepeatedFieldEncoding_Closed::REPEATED_FIELD_ENCODING_UNKNOWN),
      FeatureSet_RepeatedFieldEncoding::PACKED => Some(FeatureSet_RepeatedFieldEncoding_Closed::PACKED),
      FeatureSet_RepeatedFieldEncoding::EXPANDED => Some(FeatureSet_RepeatedFieldEncoding_Closed::EXPANDED),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FeatureSet_RepeatedFieldEncoding {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FeatureSet_RepeatedFieldEncoding_Closed {
  REPEATED_FIELD_ENCODING_UNKNOWN = 0,
  PACKED = 1,
  EXPANDED = 2,
}
impl FeatureSet_RepeatedFieldEncoding_Closed {
  pub const KNOWN_VARIANTS: [FeatureSet_RepeatedFieldEncoding_Closed; 3] = [FeatureSet_RepeatedFieldEncoding_Closed::REPEATED_FIELD_ENCODING_UNKNOWN, FeatureSet_RepeatedFieldEncoding_Closed::PACKED, FeatureSet_RepeatedFieldEncoding_Closed::EXPANDED];
}
impl ::std::default::Default for FeatureSet_RepeatedFieldEncoding_Closed {
  fn default() -> Self {
    FeatureSet_RepeatedFieldEncoding_Closed::REPEATED_FIELD_ENCODING_UNKNOWN
  }
}
impl From<FeatureSet_RepeatedFieldEncoding_Closed> for i32 {
  fn from(v: FeatureSet_RepeatedFieldEncoding_Closed) -> i32 {
    match v {
      FeatureSet_RepeatedFieldEncoding_Closed::REPEATED_FIELD_ENCODING_UNKNOWN => 0,
      FeatureSet_RepeatedFieldEncoding_Closed::PACKED => 1,
      FeatureSet_RepeatedFieldEncoding_Closed::EXPANDED => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FeatureSet_RepeatedFieldEncoding_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FeatureSet_RepeatedFieldEncoding_Closed::REPEATED_FIELD_ENCODING_UNKNOWN),
      1 => Ok(FeatureSet_RepeatedFieldEncoding_Closed::PACKED),
      2 => Ok(FeatureSet_RepeatedFieldEncoding_Closed::EXPANDED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_RepeatedFieldEncoding_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FeatureSet_RepeatedFieldEncoding_Closed {
  fn name(self) -> &'static str {
    match self {
      FeatureSet_RepeatedFieldEncoding_Closed::REPEATED_FIELD_ENCODING_UNKNOWN => "REPEATED_FIELD_ENCODING_UNKNOWN",
      FeatureSet_RepeatedFieldEncoding_Closed::PACKED => "PACKED",
      FeatureSet_RepeatedFieldEncoding_Closed::EXPANDED => "EXPANDED",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FeatureSet_Utf8Validation(i32);
impl FeatureSet_Utf8Validation {
  pub const UTF8_VALIDATION_UNKNOWN: FeatureSet_Utf8Validation = FeatureSet_Utf8Validation(0);
  pub const NONE: FeatureSet_Utf8Validation = FeatureSet_Utf8Validation(1);
  pub const VERIFY: FeatureSet_Utf8Validation = FeatureSet_Utf8Validation(2);
  pub const KNOWN_VARIANTS: [FeatureSet_Utf8Validation; 3] = [FeatureSet_Utf8Validation::UTF8_VALIDATION_UNKNOWN, FeatureSet_Utf8Validation::NONE, FeatureSet_Utf8Validation::VERIFY];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FeatureSet_Utf8Validation {
  fn default() -> Self {
    FeatureSet_Utf8Validation::UTF8_VALIDATION_UNKNOWN
  }
}
impl From<FeatureSet_Utf8Validation> for i32 {
  fn from(v: FeatureSet_Utf8Validation) -> i32 {
    v.0
  }
}
impl From<i32> for FeatureSet_Utf8Validation {
  fn from(v: i32) -> FeatureSet_Utf8Validation {
    FeatureSet_Utf8Validation(v)
  }
}
impl From<FeatureSet_Utf8Validation_Closed> for FeatureSet_Utf8Validation {
  fn from(v: FeatureSet_Utf8Validation_Closed) -> FeatureSet_Utf8Validation {
    FeatureSet_Utf8Validation(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_Utf8Validation {
}
impl ::pb_jelly::OpenProtoEnum for FeatureSet_Utf8Validation {
  type Closed = FeatureSet_Utf8Validation_Closed;
  fn into_known(self) -> ::std::option::Option<FeatureSet_Utf8Validation_Closed> {
    match self {
      FeatureSet_Utf8Validation::UTF8_VALIDATION_UNKNOWN => Some(FeatureSet_Utf8Validation_Closed::UTF8_VALIDATION_UNKNOWN),
      FeatureSet_Utf8Validation::NONE => Some(FeatureSet_Utf8Validation_Closed::NONE),
      FeatureSet_Utf8Validation::VERIFY => Some(FeatureSet_Utf8Validation_Closed::VERIFY),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FeatureSet_Utf8Validation {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FeatureSet_Utf8Validation_Closed {
  UTF8_VALIDATION_UNKNOWN = 0,
  NONE = 1,
  VERIFY = 2,
}
impl FeatureSet_Utf8Validation_Closed {
  pub const KNOWN_VARIANTS: [FeatureSet_Utf8Validation_Closed; 3] = [FeatureSet_Utf8Validation_Closed::UTF8_VALIDATION_UNKNOWN, FeatureSet_Utf8Validation_Closed::NONE, FeatureSet_Utf8Validation_Closed::VERIFY];
}
impl ::std::default::Default for FeatureSet_Utf8Validation_Closed {
  fn default() -> Self {
    FeatureSet_Utf8Validation_Closed::UTF8_VALIDATION_UNKNOWN
  }
}
impl From<FeatureSet_Utf8Validation_Closed> for i32 {
  fn from(v: FeatureSet_Utf8Validation_Closed) -> i32 {
    match v {
      FeatureSet_Utf8Validation_Closed::UTF8_VALIDATION_UNKNOWN => 0,
      FeatureSet_Utf8Validation_Closed::NONE => 1,
      FeatureSet_Utf8Validation_Closed::VERIFY => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FeatureSet_Utf8Validation_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FeatureSet_Utf8Validation_Closed::UTF8_VALIDATION_UNKNOWN),
      1 => Ok(FeatureSet_Utf8Validation_Closed::NONE),
      2 => Ok(FeatureSet_Utf8Validation_Closed::VERIFY),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_Utf8Validation_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FeatureSet_Utf8Validation_Closed {
  fn name(self) -> &'static str {
    match self {
      FeatureSet_Utf8Validation_Closed::UTF8_VALIDATION_UNKNOWN => "UTF8_VALIDATION_UNKNOWN",
      FeatureSet_Utf8Validation_Closed::NONE => "NONE",
      FeatureSet_Utf8Validation_Closed::VERIFY => "VERIFY",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FeatureSet_MessageEncoding(i32);
impl FeatureSet_MessageEncoding {
  pub const MESSAGE_ENCODING_UNKNOWN: FeatureSet_MessageEncoding = FeatureSet_MessageEncoding(0);
  pub const LENGTH_PREFIXED: FeatureSet_MessageEncoding = FeatureSet_MessageEncoding(1);
  pub const DELIMITED: FeatureSet_MessageEncoding = FeatureSet_MessageEncoding(2);
  pub const KNOWN_VARIANTS: [FeatureSet_MessageEncoding; 3] = [FeatureSet_MessageEncoding::MESSAGE_ENCODING_UNKNOWN, FeatureSet_MessageEncoding::LENGTH_PREFIXED, FeatureSet_MessageEncoding::DELIMITED];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FeatureSet_MessageEncoding {
  fn default() -> Self {
    FeatureSet_MessageEncoding::MESSAGE_ENCODING_UNKNOWN
  }
}
impl From<FeatureSet_MessageEncoding> for i32 {
  fn from(v: FeatureSet_MessageEncoding) -> i32 {
    v.0
  }
}
impl From<i32> for FeatureSet_MessageEncoding {
  fn from(v: i32) -> FeatureSet_MessageEncoding {
    FeatureSet_MessageEncoding(v)
  }
}
impl From<FeatureSet_MessageEncoding_Closed> for FeatureSet_MessageEncoding {
  fn from(v: FeatureSet_MessageEncoding_Closed) -> FeatureSet_MessageEncoding {
    FeatureSet_MessageEncoding(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_MessageEncoding {
}
impl ::pb_jelly::OpenProtoEnum for FeatureSet_MessageEncoding {
  type Closed = FeatureSet_MessageEncoding_Closed;
  fn into_known(self) -> ::std::option::Option<FeatureSet_MessageEncoding_Closed> {
    match self {
      FeatureSet_MessageEncoding::MESSAGE_ENCODING_UNKNOWN => Some(FeatureSet_MessageEncoding_Closed::MESSAGE_ENCODING_UNKNOWN),
      FeatureSet_MessageEncoding::LENGTH_PREFIXED => Some(FeatureSet_MessageEncoding_Closed::LENGTH_PREFIXED),
      FeatureSet_MessageEncoding::DELIMITED => Some(FeatureSet_MessageEncoding_Closed::DELIMITED),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FeatureSet_MessageEncoding {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FeatureSet_MessageEncoding_Closed {
  MESSAGE_ENCODING_UNKNOWN = 0,
  LENGTH_PREFIXED = 1,
  DELIMITED = 2,
}
impl FeatureSet_MessageEncoding_Closed {
  pub const KNOWN_VARIANTS: [FeatureSet_MessageEncoding_Closed; 3] = [FeatureSet_MessageEncoding_Closed::MESSAGE_ENCODING_UNKNOWN, FeatureSet_MessageEncoding_Closed::LENGTH_PREFIXED, FeatureSet_MessageEncoding_Closed::DELIMITED];
}
impl ::std::default::Default for FeatureSet_MessageEncoding_Closed {
  fn default() -> Self {
    FeatureSet_MessageEncoding_Closed::MESSAGE_ENCODING_UNKNOWN
  }
}
impl From<FeatureSet_MessageEncoding_Closed> for i32 {
  fn from(v: FeatureSet_MessageEncoding_Closed) -> i32 {
    match v {
      FeatureSet_MessageEncoding_Closed::MESSAGE_ENCODING_UNKNOWN => 0,
      FeatureSet_MessageEncoding_Closed::LENGTH_PREFIXED => 1,
      FeatureSet_MessageEncoding_Closed::DELIMITED => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FeatureSet_MessageEncoding_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FeatureSet_MessageEncoding_Closed::MESSAGE_ENCODING_UNKNOWN),
      1 => Ok(FeatureSet_MessageEncoding_Closed::LENGTH_PREFIXED),
      2 => Ok(FeatureSet_MessageEncoding_Closed::DELIMITED),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_MessageEncoding_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FeatureSet_MessageEncoding_Closed {
  fn name(self) -> &'static str {
    match self {
      FeatureSet_MessageEncoding_Closed::MESSAGE_ENCODING_UNKNOWN => "MESSAGE_ENCODING_UNKNOWN",
      FeatureSet_MessageEncoding_Closed::LENGTH_PREFIXED => "LENGTH_PREFIXED",
      FeatureSet_MessageEncoding_Closed::DELIMITED => "DELIMITED",
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FeatureSet_JsonFormat(i32);
impl FeatureSet_JsonFormat {
  pub const JSON_FORMAT_UNKNOWN: FeatureSet_JsonFormat = FeatureSet_JsonFormat(0);
  pub const ALLOW: FeatureSet_JsonFormat = FeatureSet_JsonFormat(1);
  pub const LEGACY_BEST_EFFORT: FeatureSet_JsonFormat = FeatureSet_JsonFormat(2);
  pub const KNOWN_VARIANTS: [FeatureSet_JsonFormat; 3] = [FeatureSet_JsonFormat::JSON_FORMAT_UNKNOWN, FeatureSet_JsonFormat::ALLOW, FeatureSet_JsonFormat::LEGACY_BEST_EFFORT];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for FeatureSet_JsonFormat {
  fn default() -> Self {
    FeatureSet_JsonFormat::JSON_FORMAT_UNKNOWN
  }
}
impl From<FeatureSet_JsonFormat> for i32 {
  fn from(v: FeatureSet_JsonFormat) -> i32 {
    v.0
  }
}
impl From<i32> for FeatureSet_JsonFormat {
  fn from(v: i32) -> FeatureSet_JsonFormat {
    FeatureSet_JsonFormat(v)
  }
}
impl From<FeatureSet_JsonFormat_Closed> for FeatureSet_JsonFormat {
  fn from(v: FeatureSet_JsonFormat_Closed) -> FeatureSet_JsonFormat {
    FeatureSet_JsonFormat(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_JsonFormat {
}
impl ::pb_jelly::OpenProtoEnum for FeatureSet_JsonFormat {
  type Closed = FeatureSet_JsonFormat_Closed;
  fn into_known(self) -> ::std::option::Option<FeatureSet_JsonFormat_Closed> {
    match self {
      FeatureSet_JsonFormat::JSON_FORMAT_UNKNOWN => Some(FeatureSet_JsonFormat_Closed::JSON_FORMAT_UNKNOWN),
      FeatureSet_JsonFormat::ALLOW => Some(FeatureSet_JsonFormat_Closed::ALLOW),
      FeatureSet_JsonFormat::LEGACY_BEST_EFFORT => Some(FeatureSet_JsonFormat_Closed::LEGACY_BEST_EFFORT),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for FeatureSet_JsonFormat {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum FeatureSet_JsonFormat_Closed {
  JSON_FORMAT_UNKNOWN = 0,
  ALLOW = 1,
  LEGACY_BEST_EFFORT = 2,
}
impl FeatureSet_JsonFormat_Closed {
  pub const KNOWN_VARIANTS: [FeatureSet_JsonFormat_Closed; 3] = [FeatureSet_JsonFormat_Closed::JSON_FORMAT_UNKNOWN, FeatureSet_JsonFormat_Closed::ALLOW, FeatureSet_JsonFormat_Closed::LEGACY_BEST_EFFORT];
}
impl ::std::default::Default for FeatureSet_JsonFormat_Closed {
  fn default() -> Self {
    FeatureSet_JsonFormat_Closed::JSON_FORMAT_UNKNOWN
  }
}
impl From<FeatureSet_JsonFormat_Closed> for i32 {
  fn from(v: FeatureSet_JsonFormat_Closed) -> i32 {
    match v {
      FeatureSet_JsonFormat_Closed::JSON_FORMAT_UNKNOWN => 0,
      FeatureSet_JsonFormat_Closed::ALLOW => 1,
      FeatureSet_JsonFormat_Closed::LEGACY_BEST_EFFORT => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for FeatureSet_JsonFormat_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(FeatureSet_JsonFormat_Closed::JSON_FORMAT_UNKNOWN),
      1 => Ok(FeatureSet_JsonFormat_Closed::ALLOW),
      2 => Ok(FeatureSet_JsonFormat_Closed::LEGACY_BEST_EFFORT),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for FeatureSet_JsonFormat_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for FeatureSet_JsonFormat_Closed {
  fn name(self) -> &'static str {
    match self {
      FeatureSet_JsonFormat_Closed::JSON_FORMAT_UNKNOWN => "JSON_FORMAT_UNKNOWN",
      FeatureSet_JsonFormat_Closed::ALLOW => "ALLOW",
      FeatureSet_JsonFormat_Closed::LEGACY_BEST_EFFORT => "LEGACY_BEST_EFFORT",
    }
  }
}

/// Represents the identified object's effect on the element in the original
/// .proto file.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct GeneratedCodeInfo_Annotation_Semantic(i32);
impl GeneratedCodeInfo_Annotation_Semantic {
  /// There is no effect or the effect is indescribable.
  pub const NONE: GeneratedCodeInfo_Annotation_Semantic = GeneratedCodeInfo_Annotation_Semantic(0);
  /// The element is set or otherwise mutated.
  pub const SET: GeneratedCodeInfo_Annotation_Semantic = GeneratedCodeInfo_Annotation_Semantic(1);
  /// An alias to the element is returned.
  pub const ALIAS: GeneratedCodeInfo_Annotation_Semantic = GeneratedCodeInfo_Annotation_Semantic(2);
  pub const KNOWN_VARIANTS: [GeneratedCodeInfo_Annotation_Semantic; 3] = [GeneratedCodeInfo_Annotation_Semantic::NONE, GeneratedCodeInfo_Annotation_Semantic::SET, GeneratedCodeInfo_Annotation_Semantic::ALIAS];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for GeneratedCodeInfo_Annotation_Semantic {
  fn default() -> Self {
    GeneratedCodeInfo_Annotation_Semantic::NONE
  }
}
impl From<GeneratedCodeInfo_Annotation_Semantic> for i32 {
  fn from(v: GeneratedCodeInfo_Annotation_Semantic) -> i32 {
    v.0
  }
}
impl From<i32> for GeneratedCodeInfo_Annotation_Semantic {
  fn from(v: i32) -> GeneratedCodeInfo_Annotation_Semantic {
    GeneratedCodeInfo_Annotation_Semantic(v)
  }
}
impl From<GeneratedCodeInfo_Annotation_Semantic_Closed> for GeneratedCodeInfo_Annotation_Semantic {
  fn from(v: GeneratedCodeInfo_Annotation_Semantic_Closed) -> GeneratedCodeInfo_Annotation_Semantic {
    GeneratedCodeInfo_Annotation_Semantic(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for GeneratedCodeInfo_Annotation_Semantic {
}
impl ::pb_jelly::OpenProtoEnum for GeneratedCodeInfo_Annotation_Semantic {
  type Closed = GeneratedCodeInfo_Annotation_Semantic_Closed;
  fn into_known(self) -> ::std::option::Option<GeneratedCodeInfo_Annotation_Semantic_Closed> {
    match self {
      GeneratedCodeInfo_Annotation_Semantic::NONE => Some(GeneratedCodeInfo_Annotation_Semantic_Closed::NONE),
      GeneratedCodeInfo_Annotation_Semantic::SET => Some(GeneratedCodeInfo_Annotation_Semantic_Closed::SET),
      GeneratedCodeInfo_Annotation_Semantic::ALIAS => Some(GeneratedCodeInfo_Annotation_Semantic_Closed::ALIAS),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for GeneratedCodeInfo_Annotation_Semantic {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// Represents the identified object's effect on the element in the original
/// .proto file.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum GeneratedCodeInfo_Annotation_Semantic_Closed {
  /// There is no effect or the effect is indescribable.
  NONE = 0,
  /// The element is set or otherwise mutated.
  SET = 1,
  /// An alias to the element is returned.
  ALIAS = 2,
}
impl GeneratedCodeInfo_Annotation_Semantic_Closed {
  pub const KNOWN_VARIANTS: [GeneratedCodeInfo_Annotation_Semantic_Closed; 3] = [GeneratedCodeInfo_Annotation_Semantic_Closed::NONE, GeneratedCodeInfo_Annotation_Semantic_Closed::SET, GeneratedCodeInfo_Annotation_Semantic_Closed::ALIAS];
}
impl ::std::default::Default for GeneratedCodeInfo_Annotation_Semantic_Closed {
  fn default() -> Self {
    GeneratedCodeInfo_Annotation_Semantic_Closed::NONE
  }
}
impl From<GeneratedCodeInfo_Annotation_Semantic_Closed> for i32 {
  fn from(v: GeneratedCodeInfo_Annotation_Semantic_Closed) -> i32 {
    match v {
      GeneratedCodeInfo_Annotation_Semantic_Closed::NONE => 0,
      GeneratedCodeInfo_Annotation_Semantic_Closed::SET => 1,
      GeneratedCodeInfo_Annotation_Semantic_Closed::ALIAS => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for GeneratedCodeInfo_Annotation_Semantic_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(GeneratedCodeInfo_Annotation_Semantic_Closed::NONE),
      1 => Ok(GeneratedCodeInfo_Annotation_Semantic_Closed::SET),
      2 => Ok(GeneratedCodeInfo_Annotation_Semantic_Closed::ALIAS),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for GeneratedCodeInfo_Annotation_Semantic_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for GeneratedCodeInfo_Annotation_Semantic_Closed {
  fn name(self) -> &'static str {
    match self {
      GeneratedCodeInfo_Annotation_Semantic_Closed::NONE => "NONE",
      GeneratedCodeInfo_Annotation_Semantic_Closed::SET => "SET",
      GeneratedCodeInfo_Annotation_Semantic_Closed::ALIAS => "ALIAS",
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
    ::std::mem::take(&mut self.file)
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
impl ::pb_jelly::Message for FileDescriptorSet {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FileDescriptorSet",
      full_name: "google.protobuf.FileDescriptorSet",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "file",
          full_name: "google.protobuf.FileDescriptorSet.file",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut file_size = 0;
    for val in &self.file {
      let l = ::pb_jelly::Message::compute_size(val);
      file_size += ::pb_jelly::wire_format::serialized_length(1);
      file_size += ::pb_jelly::varint::serialized_length(l as u64);
      file_size += l;
    }
    size += file_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.file {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FileDescriptorProto>(buf, typ, "FileDescriptorSet", 1)?;
          self.file.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FileDescriptorSet {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "file" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
  /// The supported values are "proto2", "proto3", and "editions".

  /// If `edition` is present, this value must be "editions".
  pub syntax: ::std::option::Option<::std::string::String>,
  /// The edition of the proto file.
  pub edition: ::std::option::Option<Edition>,
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
    self.name.as_deref().unwrap_or("")
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
    self.package.as_deref().unwrap_or("")
  }
  pub fn set_dependency(&mut self, v: ::std::vec::Vec<::std::string::String>) {
    self.dependency = v;
  }
  pub fn take_dependency(&mut self) -> ::std::vec::Vec<::std::string::String> {
    ::std::mem::take(&mut self.dependency)
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
    ::std::mem::take(&mut self.public_dependency)
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
    ::std::mem::take(&mut self.weak_dependency)
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
    ::std::mem::take(&mut self.message_type)
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
    ::std::mem::take(&mut self.enum_type)
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
    ::std::mem::take(&mut self.service)
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
    ::std::mem::take(&mut self.extension)
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
    self.syntax.as_deref().unwrap_or("")
  }
  pub fn has_edition(&self) -> bool {
    self.edition.is_some()
  }
  pub fn set_edition(&mut self, v: Edition) {
    self.edition = Some(v);
  }
  pub fn get_edition(&self) -> Edition {
    self.edition.unwrap_or_default()
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
      edition: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FileDescriptorProto_default: FileDescriptorProto = FileDescriptorProto::default();
}
impl ::pb_jelly::Message for FileDescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FileDescriptorProto",
      full_name: "google.protobuf.FileDescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.FileDescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "package",
          full_name: "google.protobuf.FileDescriptorProto.package",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "dependency",
          full_name: "google.protobuf.FileDescriptorProto.dependency",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "public_dependency",
          full_name: "google.protobuf.FileDescriptorProto.public_dependency",
          index: 3,
          number: 10,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "weak_dependency",
          full_name: "google.protobuf.FileDescriptorProto.weak_dependency",
          index: 4,
          number: 11,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "message_type",
          full_name: "google.protobuf.FileDescriptorProto.message_type",
          index: 5,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "enum_type",
          full_name: "google.protobuf.FileDescriptorProto.enum_type",
          index: 6,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "service",
          full_name: "google.protobuf.FileDescriptorProto.service",
          index: 7,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "extension",
          full_name: "google.protobuf.FileDescriptorProto.extension",
          index: 8,
          number: 7,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.FileDescriptorProto.options",
          index: 9,
          number: 8,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "source_code_info",
          full_name: "google.protobuf.FileDescriptorProto.source_code_info",
          index: 10,
          number: 9,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "syntax",
          full_name: "google.protobuf.FileDescriptorProto.syntax",
          index: 11,
          number: 12,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "edition",
          full_name: "google.protobuf.FileDescriptorProto.edition",
          index: 12,
          number: 14,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut package_size = 0;
    if let Some(ref val) = self.package {
      let l = ::pb_jelly::Message::compute_size(val);
      package_size += ::pb_jelly::wire_format::serialized_length(2);
      package_size += ::pb_jelly::varint::serialized_length(l as u64);
      package_size += l;
    }
    size += package_size;
    let mut dependency_size = 0;
    for val in &self.dependency {
      let l = ::pb_jelly::Message::compute_size(val);
      dependency_size += ::pb_jelly::wire_format::serialized_length(3);
      dependency_size += ::pb_jelly::varint::serialized_length(l as u64);
      dependency_size += l;
    }
    size += dependency_size;
    let mut public_dependency_size = 0;
    for val in &self.public_dependency {
      let l = ::pb_jelly::Message::compute_size(val);
      public_dependency_size += ::pb_jelly::wire_format::serialized_length(10);
      public_dependency_size += l;
    }
    size += public_dependency_size;
    let mut weak_dependency_size = 0;
    for val in &self.weak_dependency {
      let l = ::pb_jelly::Message::compute_size(val);
      weak_dependency_size += ::pb_jelly::wire_format::serialized_length(11);
      weak_dependency_size += l;
    }
    size += weak_dependency_size;
    let mut message_type_size = 0;
    for val in &self.message_type {
      let l = ::pb_jelly::Message::compute_size(val);
      message_type_size += ::pb_jelly::wire_format::serialized_length(4);
      message_type_size += ::pb_jelly::varint::serialized_length(l as u64);
      message_type_size += l;
    }
    size += message_type_size;
    let mut enum_type_size = 0;
    for val in &self.enum_type {
      let l = ::pb_jelly::Message::compute_size(val);
      enum_type_size += ::pb_jelly::wire_format::serialized_length(5);
      enum_type_size += ::pb_jelly::varint::serialized_length(l as u64);
      enum_type_size += l;
    }
    size += enum_type_size;
    let mut service_size = 0;
    for val in &self.service {
      let l = ::pb_jelly::Message::compute_size(val);
      service_size += ::pb_jelly::wire_format::serialized_length(6);
      service_size += ::pb_jelly::varint::serialized_length(l as u64);
      service_size += l;
    }
    size += service_size;
    let mut extension_size = 0;
    for val in &self.extension {
      let l = ::pb_jelly::Message::compute_size(val);
      extension_size += ::pb_jelly::wire_format::serialized_length(7);
      extension_size += ::pb_jelly::varint::serialized_length(l as u64);
      extension_size += l;
    }
    size += extension_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(8);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut source_code_info_size = 0;
    if let Some(ref val) = self.source_code_info {
      let l = ::pb_jelly::Message::compute_size(val);
      source_code_info_size += ::pb_jelly::wire_format::serialized_length(9);
      source_code_info_size += ::pb_jelly::varint::serialized_length(l as u64);
      source_code_info_size += l;
    }
    size += source_code_info_size;
    let mut syntax_size = 0;
    if let Some(ref val) = self.syntax {
      let l = ::pb_jelly::Message::compute_size(val);
      syntax_size += ::pb_jelly::wire_format::serialized_length(12);
      syntax_size += ::pb_jelly::varint::serialized_length(l as u64);
      syntax_size += l;
    }
    size += syntax_size;
    let mut edition_size = 0;
    if let Some(ref val) = self.edition {
      let l = ::pb_jelly::Message::compute_size(val);
      edition_size += ::pb_jelly::wire_format::serialized_length(14);
      edition_size += l;
    }
    size += edition_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.package {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.dependency {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.message_type {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.enum_type {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.service {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.extension {
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.source_code_info {
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.public_dependency {
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.weak_dependency {
      ::pb_jelly::wire_format::write(11, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.syntax {
      ::pb_jelly::wire_format::write(12, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.edition {
      ::pb_jelly::wire_format::write(14, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileDescriptorProto", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileDescriptorProto", 2)?;
          self.package = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileDescriptorProto", 3)?;
          self.dependency.push(val);
        }
        10 => {
          ::pb_jelly::helpers::deserialize_packed::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileDescriptorProto", 10, &mut self.public_dependency)?;
        }
        11 => {
          ::pb_jelly::helpers::deserialize_packed::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileDescriptorProto", 11, &mut self.weak_dependency)?;
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, DescriptorProto>(buf, typ, "FileDescriptorProto", 4)?;
          self.message_type.push(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, EnumDescriptorProto>(buf, typ, "FileDescriptorProto", 5)?;
          self.enum_type.push(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ServiceDescriptorProto>(buf, typ, "FileDescriptorProto", 6)?;
          self.service.push(val);
        }
        7 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FieldDescriptorProto>(buf, typ, "FileDescriptorProto", 7)?;
          self.extension.push(val);
        }
        8 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FileOptions>(buf, typ, "FileDescriptorProto", 8)?;
          self.options = Some(val);
        }
        9 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, SourceCodeInfo>(buf, typ, "FileDescriptorProto", 9)?;
          self.source_code_info = Some(val);
        }
        12 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileDescriptorProto", 12)?;
          self.syntax = Some(val);
        }
        14 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, Edition>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileDescriptorProto", 14)?;
          self.edition = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FileDescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "package" => {
        ::pb_jelly::reflection::FieldMut::Value(self.package.get_or_insert_with(::std::default::Default::default))
      }
      "dependency" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "public_dependency" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "weak_dependency" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "message_type" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "enum_type" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "service" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "extension" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      "source_code_info" => {
        ::pb_jelly::reflection::FieldMut::Value(self.source_code_info.get_or_insert_with(::std::default::Default::default))
      }
      "syntax" => {
        ::pb_jelly::reflection::FieldMut::Value(self.syntax.get_or_insert_with(::std::default::Default::default))
      }
      "edition" => {
        ::pb_jelly::reflection::FieldMut::Value(self.edition.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    self.name.as_deref().unwrap_or("")
  }
  pub fn set_field(&mut self, v: ::std::vec::Vec<FieldDescriptorProto>) {
    self.field = v;
  }
  pub fn take_field(&mut self) -> ::std::vec::Vec<FieldDescriptorProto> {
    ::std::mem::take(&mut self.field)
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
    ::std::mem::take(&mut self.extension)
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
    ::std::mem::take(&mut self.nested_type)
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
    ::std::mem::take(&mut self.enum_type)
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
    ::std::mem::take(&mut self.extension_range)
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
    ::std::mem::take(&mut self.oneof_decl)
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
    ::std::mem::take(&mut self.reserved_range)
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
    ::std::mem::take(&mut self.reserved_name)
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
impl ::pb_jelly::Message for DescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "DescriptorProto",
      full_name: "google.protobuf.DescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.DescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "field",
          full_name: "google.protobuf.DescriptorProto.field",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "extension",
          full_name: "google.protobuf.DescriptorProto.extension",
          index: 2,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "nested_type",
          full_name: "google.protobuf.DescriptorProto.nested_type",
          index: 3,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "enum_type",
          full_name: "google.protobuf.DescriptorProto.enum_type",
          index: 4,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "extension_range",
          full_name: "google.protobuf.DescriptorProto.extension_range",
          index: 5,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "oneof_decl",
          full_name: "google.protobuf.DescriptorProto.oneof_decl",
          index: 6,
          number: 8,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.DescriptorProto.options",
          index: 7,
          number: 7,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "reserved_range",
          full_name: "google.protobuf.DescriptorProto.reserved_range",
          index: 8,
          number: 9,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "reserved_name",
          full_name: "google.protobuf.DescriptorProto.reserved_name",
          index: 9,
          number: 10,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut field_size = 0;
    for val in &self.field {
      let l = ::pb_jelly::Message::compute_size(val);
      field_size += ::pb_jelly::wire_format::serialized_length(2);
      field_size += ::pb_jelly::varint::serialized_length(l as u64);
      field_size += l;
    }
    size += field_size;
    let mut extension_size = 0;
    for val in &self.extension {
      let l = ::pb_jelly::Message::compute_size(val);
      extension_size += ::pb_jelly::wire_format::serialized_length(6);
      extension_size += ::pb_jelly::varint::serialized_length(l as u64);
      extension_size += l;
    }
    size += extension_size;
    let mut nested_type_size = 0;
    for val in &self.nested_type {
      let l = ::pb_jelly::Message::compute_size(val);
      nested_type_size += ::pb_jelly::wire_format::serialized_length(3);
      nested_type_size += ::pb_jelly::varint::serialized_length(l as u64);
      nested_type_size += l;
    }
    size += nested_type_size;
    let mut enum_type_size = 0;
    for val in &self.enum_type {
      let l = ::pb_jelly::Message::compute_size(val);
      enum_type_size += ::pb_jelly::wire_format::serialized_length(4);
      enum_type_size += ::pb_jelly::varint::serialized_length(l as u64);
      enum_type_size += l;
    }
    size += enum_type_size;
    let mut extension_range_size = 0;
    for val in &self.extension_range {
      let l = ::pb_jelly::Message::compute_size(val);
      extension_range_size += ::pb_jelly::wire_format::serialized_length(5);
      extension_range_size += ::pb_jelly::varint::serialized_length(l as u64);
      extension_range_size += l;
    }
    size += extension_range_size;
    let mut oneof_decl_size = 0;
    for val in &self.oneof_decl {
      let l = ::pb_jelly::Message::compute_size(val);
      oneof_decl_size += ::pb_jelly::wire_format::serialized_length(8);
      oneof_decl_size += ::pb_jelly::varint::serialized_length(l as u64);
      oneof_decl_size += l;
    }
    size += oneof_decl_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(7);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut reserved_range_size = 0;
    for val in &self.reserved_range {
      let l = ::pb_jelly::Message::compute_size(val);
      reserved_range_size += ::pb_jelly::wire_format::serialized_length(9);
      reserved_range_size += ::pb_jelly::varint::serialized_length(l as u64);
      reserved_range_size += l;
    }
    size += reserved_range_size;
    let mut reserved_name_size = 0;
    for val in &self.reserved_name {
      let l = ::pb_jelly::Message::compute_size(val);
      reserved_name_size += ::pb_jelly::wire_format::serialized_length(10);
      reserved_name_size += ::pb_jelly::varint::serialized_length(l as u64);
      reserved_name_size += l;
    }
    size += reserved_name_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.field {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.nested_type {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.enum_type {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.extension_range {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.extension {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.oneof_decl {
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.reserved_range {
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.reserved_name {
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "DescriptorProto", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FieldDescriptorProto>(buf, typ, "DescriptorProto", 2)?;
          self.field.push(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FieldDescriptorProto>(buf, typ, "DescriptorProto", 6)?;
          self.extension.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, DescriptorProto>(buf, typ, "DescriptorProto", 3)?;
          self.nested_type.push(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, EnumDescriptorProto>(buf, typ, "DescriptorProto", 4)?;
          self.enum_type.push(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, DescriptorProto_ExtensionRange>(buf, typ, "DescriptorProto", 5)?;
          self.extension_range.push(val);
        }
        8 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, OneofDescriptorProto>(buf, typ, "DescriptorProto", 8)?;
          self.oneof_decl.push(val);
        }
        7 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, MessageOptions>(buf, typ, "DescriptorProto", 7)?;
          self.options = Some(val);
        }
        9 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, DescriptorProto_ReservedRange>(buf, typ, "DescriptorProto", 9)?;
          self.reserved_range.push(val);
        }
        10 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "DescriptorProto", 10)?;
          self.reserved_name.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for DescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "field" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "extension" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "nested_type" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "enum_type" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "extension_range" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "oneof_decl" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      "reserved_range" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "reserved_name" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
impl ::pb_jelly::Message for DescriptorProto_ExtensionRange {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "DescriptorProto_ExtensionRange",
      full_name: "google.protobuf.DescriptorProto_ExtensionRange",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "start",
          full_name: "google.protobuf.DescriptorProto_ExtensionRange.start",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "end",
          full_name: "google.protobuf.DescriptorProto_ExtensionRange.end",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.DescriptorProto_ExtensionRange.options",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut start_size = 0;
    if let Some(ref val) = self.start {
      let l = ::pb_jelly::Message::compute_size(val);
      start_size += ::pb_jelly::wire_format::serialized_length(1);
      start_size += l;
    }
    size += start_size;
    let mut end_size = 0;
    if let Some(ref val) = self.end {
      let l = ::pb_jelly::Message::compute_size(val);
      end_size += ::pb_jelly::wire_format::serialized_length(2);
      end_size += l;
    }
    size += end_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(3);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.start {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.end {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "DescriptorProto_ExtensionRange", 1)?;
          self.start = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "DescriptorProto_ExtensionRange", 2)?;
          self.end = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ExtensionRangeOptions>(buf, typ, "DescriptorProto_ExtensionRange", 3)?;
          self.options = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for DescriptorProto_ExtensionRange {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "start" => {
        ::pb_jelly::reflection::FieldMut::Value(self.start.get_or_insert_with(::std::default::Default::default))
      }
      "end" => {
        ::pb_jelly::reflection::FieldMut::Value(self.end.get_or_insert_with(::std::default::Default::default))
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
impl ::pb_jelly::Message for DescriptorProto_ReservedRange {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "DescriptorProto_ReservedRange",
      full_name: "google.protobuf.DescriptorProto_ReservedRange",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "start",
          full_name: "google.protobuf.DescriptorProto_ReservedRange.start",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "end",
          full_name: "google.protobuf.DescriptorProto_ReservedRange.end",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut start_size = 0;
    if let Some(ref val) = self.start {
      let l = ::pb_jelly::Message::compute_size(val);
      start_size += ::pb_jelly::wire_format::serialized_length(1);
      start_size += l;
    }
    size += start_size;
    let mut end_size = 0;
    if let Some(ref val) = self.end {
      let l = ::pb_jelly::Message::compute_size(val);
      end_size += ::pb_jelly::wire_format::serialized_length(2);
      end_size += l;
    }
    size += end_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.start {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.end {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "DescriptorProto_ReservedRange", 1)?;
          self.start = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "DescriptorProto_ReservedRange", 2)?;
          self.end = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for DescriptorProto_ReservedRange {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "start" => {
        ::pb_jelly::reflection::FieldMut::Value(self.start.get_or_insert_with(::std::default::Default::default))
      }
      "end" => {
        ::pb_jelly::reflection::FieldMut::Value(self.end.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExtensionRangeOptions {
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  /// For external users: DO NOT USE. We are in the process of open sourcing
  /// extension declaration and executing internal cleanups before it can be
  /// used externally.
  pub declaration: ::std::vec::Vec<ExtensionRangeOptions_Declaration>,
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// The verification state of the range.
  /// TODO: flip the default to DECLARATION once all empty ranges
  /// are marked as UNVERIFIED.
  pub verification: ::std::option::Option<ExtensionRangeOptions_VerificationState>,
  pub _extensions: ::pb_jelly::Unrecognized,
}
impl ExtensionRangeOptions {
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
  }
  pub fn get_uninterpreted_option(&self) -> &[UninterpretedOption] {
    &self.uninterpreted_option
  }
  pub fn mut_uninterpreted_option(&mut self) -> &mut ::std::vec::Vec<UninterpretedOption> {
    &mut self.uninterpreted_option
  }
  pub fn set_declaration(&mut self, v: ::std::vec::Vec<ExtensionRangeOptions_Declaration>) {
    self.declaration = v;
  }
  pub fn take_declaration(&mut self) -> ::std::vec::Vec<ExtensionRangeOptions_Declaration> {
    ::std::mem::take(&mut self.declaration)
  }
  pub fn get_declaration(&self) -> &[ExtensionRangeOptions_Declaration] {
    &self.declaration
  }
  pub fn mut_declaration(&mut self) -> &mut ::std::vec::Vec<ExtensionRangeOptions_Declaration> {
    &mut self.declaration
  }
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn has_verification(&self) -> bool {
    self.verification.is_some()
  }
  pub fn set_verification(&mut self, v: ExtensionRangeOptions_VerificationState) {
    self.verification = Some(v);
  }
  pub fn get_verification(&self) -> ExtensionRangeOptions_VerificationState {
    self.verification.unwrap_or_default()
  }
}
impl ::std::default::Default for ExtensionRangeOptions {
  fn default() -> Self {
    ExtensionRangeOptions {
      uninterpreted_option: ::std::default::Default::default(),
      declaration: ::std::default::Default::default(),
      features: ::std::default::Default::default(),
      verification: Some(ExtensionRangeOptions_VerificationState::UNVERIFIED),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref ExtensionRangeOptions_default: ExtensionRangeOptions = ExtensionRangeOptions::default();
}
impl ::pb_jelly::Message for ExtensionRangeOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "ExtensionRangeOptions",
      full_name: "google.protobuf.ExtensionRangeOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.ExtensionRangeOptions.uninterpreted_option",
          index: 0,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "declaration",
          full_name: "google.protobuf.ExtensionRangeOptions.declaration",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.ExtensionRangeOptions.features",
          index: 2,
          number: 50,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "verification",
          full_name: "google.protobuf.ExtensionRangeOptions.verification",
          index: 3,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    let mut declaration_size = 0;
    for val in &self.declaration {
      let l = ::pb_jelly::Message::compute_size(val);
      declaration_size += ::pb_jelly::wire_format::serialized_length(2);
      declaration_size += ::pb_jelly::varint::serialized_length(l as u64);
      declaration_size += l;
    }
    size += declaration_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(50);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut verification_size = 0;
    if let Some(ref val) = self.verification {
      let l = ::pb_jelly::Message::compute_size(val);
      verification_size += ::pb_jelly::wire_format::serialized_length(3);
      verification_size += l;
    }
    size += verification_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.declaration {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.verification {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(50, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "ExtensionRangeOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ExtensionRangeOptions_Declaration>(buf, typ, "ExtensionRangeOptions", 2)?;
          self.declaration.push(val);
        }
        50 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "ExtensionRangeOptions", 50)?;
          self.features = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, ExtensionRangeOptions_VerificationState>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "ExtensionRangeOptions", 3)?;
          self.verification = Some(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for ExtensionRangeOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "declaration" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "verification" => {
        ::pb_jelly::reflection::FieldMut::Value(self.verification.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for ExtensionRangeOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExtensionRangeOptions_Declaration {
  /// The extension number declared within the extension range.
  pub number: ::std::option::Option<i32>,
  /// The fully-qualified name of the extension field. There must be a leading
  /// dot in front of the full name.
  pub full_name: ::std::option::Option<::std::string::String>,
  /// The fully-qualified type name of the extension field. Unlike
  /// Metadata.type, Declaration.type must have a leading dot for messages
  /// and enums.
  pub r#type: ::std::option::Option<::std::string::String>,
  /// If true, indicates that the number is reserved in the extension range,
  /// and any extension field with the number will fail to compile. Set this
  /// when a declared extension field is deleted.
  pub reserved: ::std::option::Option<bool>,
  /// If true, indicates that the extension must be defined as repeated.
  /// Otherwise the extension must be defined as optional.
  pub repeated: ::std::option::Option<bool>,
}
impl ExtensionRangeOptions_Declaration {
  pub fn has_number(&self) -> bool {
    self.number.is_some()
  }
  pub fn set_number(&mut self, v: i32) {
    self.number = Some(v);
  }
  pub fn get_number(&self) -> i32 {
    self.number.unwrap_or(0)
  }
  pub fn has_full_name(&self) -> bool {
    self.full_name.is_some()
  }
  pub fn set_full_name(&mut self, v: ::std::string::String) {
    self.full_name = Some(v);
  }
  pub fn take_full_name(&mut self) -> ::std::string::String {
    self.full_name.take().unwrap_or_default()
  }
  pub fn get_full_name(&self) -> &str {
    self.full_name.as_deref().unwrap_or("")
  }
  pub fn has_type(&self) -> bool {
    self.r#type.is_some()
  }
  pub fn set_type(&mut self, v: ::std::string::String) {
    self.r#type = Some(v);
  }
  pub fn take_type(&mut self) -> ::std::string::String {
    self.r#type.take().unwrap_or_default()
  }
  pub fn get_type(&self) -> &str {
    self.r#type.as_deref().unwrap_or("")
  }
  pub fn has_reserved(&self) -> bool {
    self.reserved.is_some()
  }
  pub fn set_reserved(&mut self, v: bool) {
    self.reserved = Some(v);
  }
  pub fn get_reserved(&self) -> bool {
    self.reserved.unwrap_or(false)
  }
  pub fn has_repeated(&self) -> bool {
    self.repeated.is_some()
  }
  pub fn set_repeated(&mut self, v: bool) {
    self.repeated = Some(v);
  }
  pub fn get_repeated(&self) -> bool {
    self.repeated.unwrap_or(false)
  }
}
impl ::std::default::Default for ExtensionRangeOptions_Declaration {
  fn default() -> Self {
    ExtensionRangeOptions_Declaration {
      number: ::std::default::Default::default(),
      full_name: ::std::default::Default::default(),
      r#type: ::std::default::Default::default(),
      reserved: ::std::default::Default::default(),
      repeated: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ExtensionRangeOptions_Declaration_default: ExtensionRangeOptions_Declaration = ExtensionRangeOptions_Declaration::default();
}
impl ::pb_jelly::Message for ExtensionRangeOptions_Declaration {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "ExtensionRangeOptions_Declaration",
      full_name: "google.protobuf.ExtensionRangeOptions_Declaration",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "number",
          full_name: "google.protobuf.ExtensionRangeOptions_Declaration.number",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "full_name",
          full_name: "google.protobuf.ExtensionRangeOptions_Declaration.full_name",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "type",
          full_name: "google.protobuf.ExtensionRangeOptions_Declaration.type",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "reserved",
          full_name: "google.protobuf.ExtensionRangeOptions_Declaration.reserved",
          index: 3,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "repeated",
          full_name: "google.protobuf.ExtensionRangeOptions_Declaration.repeated",
          index: 4,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut number_size = 0;
    if let Some(ref val) = self.number {
      let l = ::pb_jelly::Message::compute_size(val);
      number_size += ::pb_jelly::wire_format::serialized_length(1);
      number_size += l;
    }
    size += number_size;
    let mut full_name_size = 0;
    if let Some(ref val) = self.full_name {
      let l = ::pb_jelly::Message::compute_size(val);
      full_name_size += ::pb_jelly::wire_format::serialized_length(2);
      full_name_size += ::pb_jelly::varint::serialized_length(l as u64);
      full_name_size += l;
    }
    size += full_name_size;
    let mut type_size = 0;
    if let Some(ref val) = self.r#type {
      let l = ::pb_jelly::Message::compute_size(val);
      type_size += ::pb_jelly::wire_format::serialized_length(3);
      type_size += ::pb_jelly::varint::serialized_length(l as u64);
      type_size += l;
    }
    size += type_size;
    let mut reserved_size = 0;
    if let Some(ref val) = self.reserved {
      let l = ::pb_jelly::Message::compute_size(val);
      reserved_size += ::pb_jelly::wire_format::serialized_length(5);
      reserved_size += l;
    }
    size += reserved_size;
    let mut repeated_size = 0;
    if let Some(ref val) = self.repeated {
      let l = ::pb_jelly::Message::compute_size(val);
      repeated_size += ::pb_jelly::wire_format::serialized_length(6);
      repeated_size += l;
    }
    size += repeated_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.number {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.full_name {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.r#type {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.reserved {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.repeated {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "ExtensionRangeOptions_Declaration", 1)?;
          self.number = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "ExtensionRangeOptions_Declaration", 2)?;
          self.full_name = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "ExtensionRangeOptions_Declaration", 3)?;
          self.r#type = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "ExtensionRangeOptions_Declaration", 5)?;
          self.reserved = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "ExtensionRangeOptions_Declaration", 6)?;
          self.repeated = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for ExtensionRangeOptions_Declaration {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "number" => {
        ::pb_jelly::reflection::FieldMut::Value(self.number.get_or_insert_with(::std::default::Default::default))
      }
      "full_name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.full_name.get_or_insert_with(::std::default::Default::default))
      }
      "type" => {
        ::pb_jelly::reflection::FieldMut::Value(self.r#type.get_or_insert_with(::std::default::Default::default))
      }
      "reserved" => {
        ::pb_jelly::reflection::FieldMut::Value(self.reserved.get_or_insert_with(::std::default::Default::default))
      }
      "repeated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.repeated.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// Describes a field within a message.
#[derive(Clone, Debug, PartialEq)]
pub struct FieldDescriptorProto {
  pub name: ::std::option::Option<::std::string::String>,
  pub number: ::std::option::Option<i32>,
  pub label: ::std::option::Option<FieldDescriptorProto_Label>,
  /// If type_name is set, this need not be set.  If both this and type_name
  /// are set, this must be one of TYPE_ENUM, TYPE_MESSAGE or TYPE_GROUP.
  pub r#type: ::std::option::Option<FieldDescriptorProto_Type>,
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
    self.name.as_deref().unwrap_or("")
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
  pub fn has_type(&self) -> bool {
    self.r#type.is_some()
  }
  pub fn set_type(&mut self, v: FieldDescriptorProto_Type) {
    self.r#type = Some(v);
  }
  pub fn get_type(&self) -> FieldDescriptorProto_Type {
    self.r#type.unwrap_or_default()
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
    self.type_name.as_deref().unwrap_or("")
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
    self.extendee.as_deref().unwrap_or("")
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
    self.default_value.as_deref().unwrap_or("")
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
    self.json_name.as_deref().unwrap_or("")
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
      r#type: ::std::default::Default::default(),
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
impl ::pb_jelly::Message for FieldDescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FieldDescriptorProto",
      full_name: "google.protobuf.FieldDescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.FieldDescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "number",
          full_name: "google.protobuf.FieldDescriptorProto.number",
          index: 1,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "label",
          full_name: "google.protobuf.FieldDescriptorProto.label",
          index: 2,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "type",
          full_name: "google.protobuf.FieldDescriptorProto.type",
          index: 3,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "type_name",
          full_name: "google.protobuf.FieldDescriptorProto.type_name",
          index: 4,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "extendee",
          full_name: "google.protobuf.FieldDescriptorProto.extendee",
          index: 5,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "default_value",
          full_name: "google.protobuf.FieldDescriptorProto.default_value",
          index: 6,
          number: 7,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "oneof_index",
          full_name: "google.protobuf.FieldDescriptorProto.oneof_index",
          index: 7,
          number: 9,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "json_name",
          full_name: "google.protobuf.FieldDescriptorProto.json_name",
          index: 8,
          number: 10,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.FieldDescriptorProto.options",
          index: 9,
          number: 8,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "proto3_optional",
          full_name: "google.protobuf.FieldDescriptorProto.proto3_optional",
          index: 10,
          number: 17,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut number_size = 0;
    if let Some(ref val) = self.number {
      let l = ::pb_jelly::Message::compute_size(val);
      number_size += ::pb_jelly::wire_format::serialized_length(3);
      number_size += l;
    }
    size += number_size;
    let mut label_size = 0;
    if let Some(ref val) = self.label {
      let l = ::pb_jelly::Message::compute_size(val);
      label_size += ::pb_jelly::wire_format::serialized_length(4);
      label_size += l;
    }
    size += label_size;
    let mut type_size = 0;
    if let Some(ref val) = self.r#type {
      let l = ::pb_jelly::Message::compute_size(val);
      type_size += ::pb_jelly::wire_format::serialized_length(5);
      type_size += l;
    }
    size += type_size;
    let mut type_name_size = 0;
    if let Some(ref val) = self.type_name {
      let l = ::pb_jelly::Message::compute_size(val);
      type_name_size += ::pb_jelly::wire_format::serialized_length(6);
      type_name_size += ::pb_jelly::varint::serialized_length(l as u64);
      type_name_size += l;
    }
    size += type_name_size;
    let mut extendee_size = 0;
    if let Some(ref val) = self.extendee {
      let l = ::pb_jelly::Message::compute_size(val);
      extendee_size += ::pb_jelly::wire_format::serialized_length(2);
      extendee_size += ::pb_jelly::varint::serialized_length(l as u64);
      extendee_size += l;
    }
    size += extendee_size;
    let mut default_value_size = 0;
    if let Some(ref val) = self.default_value {
      let l = ::pb_jelly::Message::compute_size(val);
      default_value_size += ::pb_jelly::wire_format::serialized_length(7);
      default_value_size += ::pb_jelly::varint::serialized_length(l as u64);
      default_value_size += l;
    }
    size += default_value_size;
    let mut oneof_index_size = 0;
    if let Some(ref val) = self.oneof_index {
      let l = ::pb_jelly::Message::compute_size(val);
      oneof_index_size += ::pb_jelly::wire_format::serialized_length(9);
      oneof_index_size += l;
    }
    size += oneof_index_size;
    let mut json_name_size = 0;
    if let Some(ref val) = self.json_name {
      let l = ::pb_jelly::Message::compute_size(val);
      json_name_size += ::pb_jelly::wire_format::serialized_length(10);
      json_name_size += ::pb_jelly::varint::serialized_length(l as u64);
      json_name_size += l;
    }
    size += json_name_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(8);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut proto3_optional_size = 0;
    if let Some(ref val) = self.proto3_optional {
      let l = ::pb_jelly::Message::compute_size(val);
      proto3_optional_size += ::pb_jelly::wire_format::serialized_length(17);
      proto3_optional_size += l;
    }
    size += proto3_optional_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.extendee {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.number {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.label {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.r#type {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.type_name {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.default_value {
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.oneof_index {
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.json_name {
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.proto3_optional {
      ::pb_jelly::wire_format::write(17, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FieldDescriptorProto", 1)?;
          self.name = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldDescriptorProto", 3)?;
          self.number = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FieldDescriptorProto_Label>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldDescriptorProto", 4)?;
          self.label = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FieldDescriptorProto_Type>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldDescriptorProto", 5)?;
          self.r#type = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FieldDescriptorProto", 6)?;
          self.type_name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FieldDescriptorProto", 2)?;
          self.extendee = Some(val);
        }
        7 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FieldDescriptorProto", 7)?;
          self.default_value = Some(val);
        }
        9 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldDescriptorProto", 9)?;
          self.oneof_index = Some(val);
        }
        10 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FieldDescriptorProto", 10)?;
          self.json_name = Some(val);
        }
        8 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FieldOptions>(buf, typ, "FieldDescriptorProto", 8)?;
          self.options = Some(val);
        }
        17 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldDescriptorProto", 17)?;
          self.proto3_optional = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FieldDescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "number" => {
        ::pb_jelly::reflection::FieldMut::Value(self.number.get_or_insert_with(::std::default::Default::default))
      }
      "label" => {
        ::pb_jelly::reflection::FieldMut::Value(self.label.get_or_insert_with(::std::default::Default::default))
      }
      "type" => {
        ::pb_jelly::reflection::FieldMut::Value(self.r#type.get_or_insert_with(::std::default::Default::default))
      }
      "type_name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.type_name.get_or_insert_with(::std::default::Default::default))
      }
      "extendee" => {
        ::pb_jelly::reflection::FieldMut::Value(self.extendee.get_or_insert_with(::std::default::Default::default))
      }
      "default_value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.default_value.get_or_insert_with(::std::default::Default::default))
      }
      "oneof_index" => {
        ::pb_jelly::reflection::FieldMut::Value(self.oneof_index.get_or_insert_with(::std::default::Default::default))
      }
      "json_name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.json_name.get_or_insert_with(::std::default::Default::default))
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      "proto3_optional" => {
        ::pb_jelly::reflection::FieldMut::Value(self.proto3_optional.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    self.name.as_deref().unwrap_or("")
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
impl ::pb_jelly::Message for OneofDescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "OneofDescriptorProto",
      full_name: "google.protobuf.OneofDescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.OneofDescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.OneofDescriptorProto.options",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(2);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "OneofDescriptorProto", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, OneofOptions>(buf, typ, "OneofDescriptorProto", 2)?;
          self.options = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for OneofDescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    self.name.as_deref().unwrap_or("")
  }
  pub fn set_value(&mut self, v: ::std::vec::Vec<EnumValueDescriptorProto>) {
    self.value = v;
  }
  pub fn take_value(&mut self) -> ::std::vec::Vec<EnumValueDescriptorProto> {
    ::std::mem::take(&mut self.value)
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
    ::std::mem::take(&mut self.reserved_range)
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
    ::std::mem::take(&mut self.reserved_name)
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
impl ::pb_jelly::Message for EnumDescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "EnumDescriptorProto",
      full_name: "google.protobuf.EnumDescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.EnumDescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "value",
          full_name: "google.protobuf.EnumDescriptorProto.value",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.EnumDescriptorProto.options",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "reserved_range",
          full_name: "google.protobuf.EnumDescriptorProto.reserved_range",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "reserved_name",
          full_name: "google.protobuf.EnumDescriptorProto.reserved_name",
          index: 4,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut value_size = 0;
    for val in &self.value {
      let l = ::pb_jelly::Message::compute_size(val);
      value_size += ::pb_jelly::wire_format::serialized_length(2);
      value_size += ::pb_jelly::varint::serialized_length(l as u64);
      value_size += l;
    }
    size += value_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(3);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut reserved_range_size = 0;
    for val in &self.reserved_range {
      let l = ::pb_jelly::Message::compute_size(val);
      reserved_range_size += ::pb_jelly::wire_format::serialized_length(4);
      reserved_range_size += ::pb_jelly::varint::serialized_length(l as u64);
      reserved_range_size += l;
    }
    size += reserved_range_size;
    let mut reserved_name_size = 0;
    for val in &self.reserved_name {
      let l = ::pb_jelly::Message::compute_size(val);
      reserved_name_size += ::pb_jelly::wire_format::serialized_length(5);
      reserved_name_size += ::pb_jelly::varint::serialized_length(l as u64);
      reserved_name_size += l;
    }
    size += reserved_name_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.value {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.reserved_range {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.reserved_name {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "EnumDescriptorProto", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, EnumValueDescriptorProto>(buf, typ, "EnumDescriptorProto", 2)?;
          self.value.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, EnumOptions>(buf, typ, "EnumDescriptorProto", 3)?;
          self.options = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, EnumDescriptorProto_EnumReservedRange>(buf, typ, "EnumDescriptorProto", 4)?;
          self.reserved_range.push(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "EnumDescriptorProto", 5)?;
          self.reserved_name.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for EnumDescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "value" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      "reserved_range" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "reserved_name" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
impl ::pb_jelly::Message for EnumDescriptorProto_EnumReservedRange {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "EnumDescriptorProto_EnumReservedRange",
      full_name: "google.protobuf.EnumDescriptorProto_EnumReservedRange",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "start",
          full_name: "google.protobuf.EnumDescriptorProto_EnumReservedRange.start",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "end",
          full_name: "google.protobuf.EnumDescriptorProto_EnumReservedRange.end",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut start_size = 0;
    if let Some(ref val) = self.start {
      let l = ::pb_jelly::Message::compute_size(val);
      start_size += ::pb_jelly::wire_format::serialized_length(1);
      start_size += l;
    }
    size += start_size;
    let mut end_size = 0;
    if let Some(ref val) = self.end {
      let l = ::pb_jelly::Message::compute_size(val);
      end_size += ::pb_jelly::wire_format::serialized_length(2);
      end_size += l;
    }
    size += end_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.start {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.end {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumDescriptorProto_EnumReservedRange", 1)?;
          self.start = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumDescriptorProto_EnumReservedRange", 2)?;
          self.end = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for EnumDescriptorProto_EnumReservedRange {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "start" => {
        ::pb_jelly::reflection::FieldMut::Value(self.start.get_or_insert_with(::std::default::Default::default))
      }
      "end" => {
        ::pb_jelly::reflection::FieldMut::Value(self.end.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    self.name.as_deref().unwrap_or("")
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
impl ::pb_jelly::Message for EnumValueDescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "EnumValueDescriptorProto",
      full_name: "google.protobuf.EnumValueDescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.EnumValueDescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "number",
          full_name: "google.protobuf.EnumValueDescriptorProto.number",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.EnumValueDescriptorProto.options",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut number_size = 0;
    if let Some(ref val) = self.number {
      let l = ::pb_jelly::Message::compute_size(val);
      number_size += ::pb_jelly::wire_format::serialized_length(2);
      number_size += l;
    }
    size += number_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(3);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.number {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "EnumValueDescriptorProto", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumValueDescriptorProto", 2)?;
          self.number = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, EnumValueOptions>(buf, typ, "EnumValueDescriptorProto", 3)?;
          self.options = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for EnumValueDescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "number" => {
        ::pb_jelly::reflection::FieldMut::Value(self.number.get_or_insert_with(::std::default::Default::default))
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    self.name.as_deref().unwrap_or("")
  }
  pub fn set_method(&mut self, v: ::std::vec::Vec<MethodDescriptorProto>) {
    self.method = v;
  }
  pub fn take_method(&mut self) -> ::std::vec::Vec<MethodDescriptorProto> {
    ::std::mem::take(&mut self.method)
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
impl ::pb_jelly::Message for ServiceDescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "ServiceDescriptorProto",
      full_name: "google.protobuf.ServiceDescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.ServiceDescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "method",
          full_name: "google.protobuf.ServiceDescriptorProto.method",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.ServiceDescriptorProto.options",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut method_size = 0;
    for val in &self.method {
      let l = ::pb_jelly::Message::compute_size(val);
      method_size += ::pb_jelly::wire_format::serialized_length(2);
      method_size += ::pb_jelly::varint::serialized_length(l as u64);
      method_size += l;
    }
    size += method_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(3);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.method {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "ServiceDescriptorProto", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, MethodDescriptorProto>(buf, typ, "ServiceDescriptorProto", 2)?;
          self.method.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ServiceOptions>(buf, typ, "ServiceDescriptorProto", 3)?;
          self.options = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for ServiceDescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "method" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    self.name.as_deref().unwrap_or("")
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
    self.input_type.as_deref().unwrap_or("")
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
    self.output_type.as_deref().unwrap_or("")
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
impl ::pb_jelly::Message for MethodDescriptorProto {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "MethodDescriptorProto",
      full_name: "google.protobuf.MethodDescriptorProto",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.MethodDescriptorProto.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "input_type",
          full_name: "google.protobuf.MethodDescriptorProto.input_type",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "output_type",
          full_name: "google.protobuf.MethodDescriptorProto.output_type",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "options",
          full_name: "google.protobuf.MethodDescriptorProto.options",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "client_streaming",
          full_name: "google.protobuf.MethodDescriptorProto.client_streaming",
          index: 4,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "server_streaming",
          full_name: "google.protobuf.MethodDescriptorProto.server_streaming",
          index: 5,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    if let Some(ref val) = self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(1);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut input_type_size = 0;
    if let Some(ref val) = self.input_type {
      let l = ::pb_jelly::Message::compute_size(val);
      input_type_size += ::pb_jelly::wire_format::serialized_length(2);
      input_type_size += ::pb_jelly::varint::serialized_length(l as u64);
      input_type_size += l;
    }
    size += input_type_size;
    let mut output_type_size = 0;
    if let Some(ref val) = self.output_type {
      let l = ::pb_jelly::Message::compute_size(val);
      output_type_size += ::pb_jelly::wire_format::serialized_length(3);
      output_type_size += ::pb_jelly::varint::serialized_length(l as u64);
      output_type_size += l;
    }
    size += output_type_size;
    let mut options_size = 0;
    if let Some(ref val) = self.options {
      let l = ::pb_jelly::Message::compute_size(val);
      options_size += ::pb_jelly::wire_format::serialized_length(4);
      options_size += ::pb_jelly::varint::serialized_length(l as u64);
      options_size += l;
    }
    size += options_size;
    let mut client_streaming_size = 0;
    if let Some(ref val) = self.client_streaming {
      let l = ::pb_jelly::Message::compute_size(val);
      client_streaming_size += ::pb_jelly::wire_format::serialized_length(5);
      client_streaming_size += l;
    }
    size += client_streaming_size;
    let mut server_streaming_size = 0;
    if let Some(ref val) = self.server_streaming {
      let l = ::pb_jelly::Message::compute_size(val);
      server_streaming_size += ::pb_jelly::wire_format::serialized_length(6);
      server_streaming_size += l;
    }
    size += server_streaming_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.input_type {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.output_type {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.options {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.client_streaming {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.server_streaming {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "MethodDescriptorProto", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "MethodDescriptorProto", 2)?;
          self.input_type = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "MethodDescriptorProto", 3)?;
          self.output_type = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, MethodOptions>(buf, typ, "MethodDescriptorProto", 4)?;
          self.options = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MethodDescriptorProto", 5)?;
          self.client_streaming = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MethodDescriptorProto", 6)?;
          self.server_streaming = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for MethodDescriptorProto {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name.get_or_insert_with(::std::default::Default::default))
      }
      "input_type" => {
        ::pb_jelly::reflection::FieldMut::Value(self.input_type.get_or_insert_with(::std::default::Default::default))
      }
      "output_type" => {
        ::pb_jelly::reflection::FieldMut::Value(self.output_type.get_or_insert_with(::std::default::Default::default))
      }
      "options" => {
        ::pb_jelly::reflection::FieldMut::Value(self.options.get_or_insert_with(::std::default::Default::default))
      }
      "client_streaming" => {
        ::pb_jelly::reflection::FieldMut::Value(self.client_streaming.get_or_insert_with(::std::default::Default::default))
      }
      "server_streaming" => {
        ::pb_jelly::reflection::FieldMut::Value(self.server_streaming.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
  /// Controls the name of the wrapper Java class generated for the .proto file.
  /// That class will always contain the .proto file's getDescriptor() method as
  /// well as any top-level extensions defined in the .proto file.
  /// If java_multiple_files is disabled, then all the other classes from the
  /// .proto file will be nested inside the single wrapper outer class.
  pub java_outer_classname: ::std::option::Option<::std::string::String>,
  /// If enabled, then the Java code generator will generate a separate .java
  /// file for each top-level message, enum, and service defined in the .proto
  /// file.  Thus, these types will *not* be nested inside the wrapper class
  /// named by java_outer_classname.  However, the wrapper class will still be
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
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// The parser stores options it doesn't recognize here.
  /// See the documentation for the "Options" section above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  pub _extensions: ::pb_jelly::Unrecognized,
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
    self.java_package.as_deref().unwrap_or("")
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
    self.java_outer_classname.as_deref().unwrap_or("")
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
    self.go_package.as_deref().unwrap_or("")
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
    self.objc_class_prefix.as_deref().unwrap_or("")
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
    self.csharp_namespace.as_deref().unwrap_or("")
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
    self.swift_prefix.as_deref().unwrap_or("")
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
    self.php_class_prefix.as_deref().unwrap_or("")
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
    self.php_namespace.as_deref().unwrap_or("")
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
    self.php_metadata_namespace.as_deref().unwrap_or("")
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
    self.ruby_package.as_deref().unwrap_or("")
  }
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
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
      features: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref FileOptions_default: FileOptions = FileOptions::default();
}
impl ::pb_jelly::Message for FileOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FileOptions",
      full_name: "google.protobuf.FileOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "java_package",
          full_name: "google.protobuf.FileOptions.java_package",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "java_outer_classname",
          full_name: "google.protobuf.FileOptions.java_outer_classname",
          index: 1,
          number: 8,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "java_multiple_files",
          full_name: "google.protobuf.FileOptions.java_multiple_files",
          index: 2,
          number: 10,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "java_generate_equals_and_hash",
          full_name: "google.protobuf.FileOptions.java_generate_equals_and_hash",
          index: 3,
          number: 20,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "java_string_check_utf8",
          full_name: "google.protobuf.FileOptions.java_string_check_utf8",
          index: 4,
          number: 27,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "optimize_for",
          full_name: "google.protobuf.FileOptions.optimize_for",
          index: 5,
          number: 9,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "go_package",
          full_name: "google.protobuf.FileOptions.go_package",
          index: 6,
          number: 11,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "cc_generic_services",
          full_name: "google.protobuf.FileOptions.cc_generic_services",
          index: 7,
          number: 16,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "java_generic_services",
          full_name: "google.protobuf.FileOptions.java_generic_services",
          index: 8,
          number: 17,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "py_generic_services",
          full_name: "google.protobuf.FileOptions.py_generic_services",
          index: 9,
          number: 18,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "php_generic_services",
          full_name: "google.protobuf.FileOptions.php_generic_services",
          index: 10,
          number: 42,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "deprecated",
          full_name: "google.protobuf.FileOptions.deprecated",
          index: 11,
          number: 23,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "cc_enable_arenas",
          full_name: "google.protobuf.FileOptions.cc_enable_arenas",
          index: 12,
          number: 31,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "objc_class_prefix",
          full_name: "google.protobuf.FileOptions.objc_class_prefix",
          index: 13,
          number: 36,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "csharp_namespace",
          full_name: "google.protobuf.FileOptions.csharp_namespace",
          index: 14,
          number: 37,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "swift_prefix",
          full_name: "google.protobuf.FileOptions.swift_prefix",
          index: 15,
          number: 39,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "php_class_prefix",
          full_name: "google.protobuf.FileOptions.php_class_prefix",
          index: 16,
          number: 40,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "php_namespace",
          full_name: "google.protobuf.FileOptions.php_namespace",
          index: 17,
          number: 41,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "php_metadata_namespace",
          full_name: "google.protobuf.FileOptions.php_metadata_namespace",
          index: 18,
          number: 44,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "ruby_package",
          full_name: "google.protobuf.FileOptions.ruby_package",
          index: 19,
          number: 45,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.FileOptions.features",
          index: 20,
          number: 50,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.FileOptions.uninterpreted_option",
          index: 21,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut java_package_size = 0;
    if let Some(ref val) = self.java_package {
      let l = ::pb_jelly::Message::compute_size(val);
      java_package_size += ::pb_jelly::wire_format::serialized_length(1);
      java_package_size += ::pb_jelly::varint::serialized_length(l as u64);
      java_package_size += l;
    }
    size += java_package_size;
    let mut java_outer_classname_size = 0;
    if let Some(ref val) = self.java_outer_classname {
      let l = ::pb_jelly::Message::compute_size(val);
      java_outer_classname_size += ::pb_jelly::wire_format::serialized_length(8);
      java_outer_classname_size += ::pb_jelly::varint::serialized_length(l as u64);
      java_outer_classname_size += l;
    }
    size += java_outer_classname_size;
    let mut java_multiple_files_size = 0;
    if let Some(ref val) = self.java_multiple_files {
      let l = ::pb_jelly::Message::compute_size(val);
      java_multiple_files_size += ::pb_jelly::wire_format::serialized_length(10);
      java_multiple_files_size += l;
    }
    size += java_multiple_files_size;
    let mut java_generate_equals_and_hash_size = 0;
    if let Some(ref val) = self.java_generate_equals_and_hash {
      let l = ::pb_jelly::Message::compute_size(val);
      java_generate_equals_and_hash_size += ::pb_jelly::wire_format::serialized_length(20);
      java_generate_equals_and_hash_size += l;
    }
    size += java_generate_equals_and_hash_size;
    let mut java_string_check_utf8_size = 0;
    if let Some(ref val) = self.java_string_check_utf8 {
      let l = ::pb_jelly::Message::compute_size(val);
      java_string_check_utf8_size += ::pb_jelly::wire_format::serialized_length(27);
      java_string_check_utf8_size += l;
    }
    size += java_string_check_utf8_size;
    let mut optimize_for_size = 0;
    if let Some(ref val) = self.optimize_for {
      let l = ::pb_jelly::Message::compute_size(val);
      optimize_for_size += ::pb_jelly::wire_format::serialized_length(9);
      optimize_for_size += l;
    }
    size += optimize_for_size;
    let mut go_package_size = 0;
    if let Some(ref val) = self.go_package {
      let l = ::pb_jelly::Message::compute_size(val);
      go_package_size += ::pb_jelly::wire_format::serialized_length(11);
      go_package_size += ::pb_jelly::varint::serialized_length(l as u64);
      go_package_size += l;
    }
    size += go_package_size;
    let mut cc_generic_services_size = 0;
    if let Some(ref val) = self.cc_generic_services {
      let l = ::pb_jelly::Message::compute_size(val);
      cc_generic_services_size += ::pb_jelly::wire_format::serialized_length(16);
      cc_generic_services_size += l;
    }
    size += cc_generic_services_size;
    let mut java_generic_services_size = 0;
    if let Some(ref val) = self.java_generic_services {
      let l = ::pb_jelly::Message::compute_size(val);
      java_generic_services_size += ::pb_jelly::wire_format::serialized_length(17);
      java_generic_services_size += l;
    }
    size += java_generic_services_size;
    let mut py_generic_services_size = 0;
    if let Some(ref val) = self.py_generic_services {
      let l = ::pb_jelly::Message::compute_size(val);
      py_generic_services_size += ::pb_jelly::wire_format::serialized_length(18);
      py_generic_services_size += l;
    }
    size += py_generic_services_size;
    let mut php_generic_services_size = 0;
    if let Some(ref val) = self.php_generic_services {
      let l = ::pb_jelly::Message::compute_size(val);
      php_generic_services_size += ::pb_jelly::wire_format::serialized_length(42);
      php_generic_services_size += l;
    }
    size += php_generic_services_size;
    let mut deprecated_size = 0;
    if let Some(ref val) = self.deprecated {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_size += ::pb_jelly::wire_format::serialized_length(23);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut cc_enable_arenas_size = 0;
    if let Some(ref val) = self.cc_enable_arenas {
      let l = ::pb_jelly::Message::compute_size(val);
      cc_enable_arenas_size += ::pb_jelly::wire_format::serialized_length(31);
      cc_enable_arenas_size += l;
    }
    size += cc_enable_arenas_size;
    let mut objc_class_prefix_size = 0;
    if let Some(ref val) = self.objc_class_prefix {
      let l = ::pb_jelly::Message::compute_size(val);
      objc_class_prefix_size += ::pb_jelly::wire_format::serialized_length(36);
      objc_class_prefix_size += ::pb_jelly::varint::serialized_length(l as u64);
      objc_class_prefix_size += l;
    }
    size += objc_class_prefix_size;
    let mut csharp_namespace_size = 0;
    if let Some(ref val) = self.csharp_namespace {
      let l = ::pb_jelly::Message::compute_size(val);
      csharp_namespace_size += ::pb_jelly::wire_format::serialized_length(37);
      csharp_namespace_size += ::pb_jelly::varint::serialized_length(l as u64);
      csharp_namespace_size += l;
    }
    size += csharp_namespace_size;
    let mut swift_prefix_size = 0;
    if let Some(ref val) = self.swift_prefix {
      let l = ::pb_jelly::Message::compute_size(val);
      swift_prefix_size += ::pb_jelly::wire_format::serialized_length(39);
      swift_prefix_size += ::pb_jelly::varint::serialized_length(l as u64);
      swift_prefix_size += l;
    }
    size += swift_prefix_size;
    let mut php_class_prefix_size = 0;
    if let Some(ref val) = self.php_class_prefix {
      let l = ::pb_jelly::Message::compute_size(val);
      php_class_prefix_size += ::pb_jelly::wire_format::serialized_length(40);
      php_class_prefix_size += ::pb_jelly::varint::serialized_length(l as u64);
      php_class_prefix_size += l;
    }
    size += php_class_prefix_size;
    let mut php_namespace_size = 0;
    if let Some(ref val) = self.php_namespace {
      let l = ::pb_jelly::Message::compute_size(val);
      php_namespace_size += ::pb_jelly::wire_format::serialized_length(41);
      php_namespace_size += ::pb_jelly::varint::serialized_length(l as u64);
      php_namespace_size += l;
    }
    size += php_namespace_size;
    let mut php_metadata_namespace_size = 0;
    if let Some(ref val) = self.php_metadata_namespace {
      let l = ::pb_jelly::Message::compute_size(val);
      php_metadata_namespace_size += ::pb_jelly::wire_format::serialized_length(44);
      php_metadata_namespace_size += ::pb_jelly::varint::serialized_length(l as u64);
      php_metadata_namespace_size += l;
    }
    size += php_metadata_namespace_size;
    let mut ruby_package_size = 0;
    if let Some(ref val) = self.ruby_package {
      let l = ::pb_jelly::Message::compute_size(val);
      ruby_package_size += ::pb_jelly::wire_format::serialized_length(45);
      ruby_package_size += ::pb_jelly::varint::serialized_length(l as u64);
      ruby_package_size += l;
    }
    size += ruby_package_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(50);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.java_package {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.java_outer_classname {
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.optimize_for {
      ::pb_jelly::wire_format::write(9, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.java_multiple_files {
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.go_package {
      ::pb_jelly::wire_format::write(11, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.cc_generic_services {
      ::pb_jelly::wire_format::write(16, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.java_generic_services {
      ::pb_jelly::wire_format::write(17, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.py_generic_services {
      ::pb_jelly::wire_format::write(18, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.java_generate_equals_and_hash {
      ::pb_jelly::wire_format::write(20, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.deprecated {
      ::pb_jelly::wire_format::write(23, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.java_string_check_utf8 {
      ::pb_jelly::wire_format::write(27, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.cc_enable_arenas {
      ::pb_jelly::wire_format::write(31, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.objc_class_prefix {
      ::pb_jelly::wire_format::write(36, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.csharp_namespace {
      ::pb_jelly::wire_format::write(37, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.swift_prefix {
      ::pb_jelly::wire_format::write(39, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.php_class_prefix {
      ::pb_jelly::wire_format::write(40, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.php_namespace {
      ::pb_jelly::wire_format::write(41, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.php_generic_services {
      ::pb_jelly::wire_format::write(42, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.php_metadata_namespace {
      ::pb_jelly::wire_format::write(44, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.ruby_package {
      ::pb_jelly::wire_format::write(45, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(50, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 1)?;
          self.java_package = Some(val);
        }
        8 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 8)?;
          self.java_outer_classname = Some(val);
        }
        10 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 10)?;
          self.java_multiple_files = Some(val);
        }
        20 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 20)?;
          self.java_generate_equals_and_hash = Some(val);
        }
        27 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 27)?;
          self.java_string_check_utf8 = Some(val);
        }
        9 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FileOptions_OptimizeMode>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 9)?;
          self.optimize_for = Some(val);
        }
        11 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 11)?;
          self.go_package = Some(val);
        }
        16 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 16)?;
          self.cc_generic_services = Some(val);
        }
        17 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 17)?;
          self.java_generic_services = Some(val);
        }
        18 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 18)?;
          self.py_generic_services = Some(val);
        }
        42 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 42)?;
          self.php_generic_services = Some(val);
        }
        23 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 23)?;
          self.deprecated = Some(val);
        }
        31 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FileOptions", 31)?;
          self.cc_enable_arenas = Some(val);
        }
        36 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 36)?;
          self.objc_class_prefix = Some(val);
        }
        37 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 37)?;
          self.csharp_namespace = Some(val);
        }
        39 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 39)?;
          self.swift_prefix = Some(val);
        }
        40 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 40)?;
          self.php_class_prefix = Some(val);
        }
        41 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 41)?;
          self.php_namespace = Some(val);
        }
        44 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 44)?;
          self.php_metadata_namespace = Some(val);
        }
        45 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FileOptions", 45)?;
          self.ruby_package = Some(val);
        }
        50 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "FileOptions", 50)?;
          self.features = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "FileOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FileOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "java_package" => {
        ::pb_jelly::reflection::FieldMut::Value(self.java_package.get_or_insert_with(::std::default::Default::default))
      }
      "java_outer_classname" => {
        ::pb_jelly::reflection::FieldMut::Value(self.java_outer_classname.get_or_insert_with(::std::default::Default::default))
      }
      "java_multiple_files" => {
        ::pb_jelly::reflection::FieldMut::Value(self.java_multiple_files.get_or_insert_with(::std::default::Default::default))
      }
      "java_generate_equals_and_hash" => {
        ::pb_jelly::reflection::FieldMut::Value(self.java_generate_equals_and_hash.get_or_insert_with(::std::default::Default::default))
      }
      "java_string_check_utf8" => {
        ::pb_jelly::reflection::FieldMut::Value(self.java_string_check_utf8.get_or_insert_with(::std::default::Default::default))
      }
      "optimize_for" => {
        ::pb_jelly::reflection::FieldMut::Value(self.optimize_for.get_or_insert_with(::std::default::Default::default))
      }
      "go_package" => {
        ::pb_jelly::reflection::FieldMut::Value(self.go_package.get_or_insert_with(::std::default::Default::default))
      }
      "cc_generic_services" => {
        ::pb_jelly::reflection::FieldMut::Value(self.cc_generic_services.get_or_insert_with(::std::default::Default::default))
      }
      "java_generic_services" => {
        ::pb_jelly::reflection::FieldMut::Value(self.java_generic_services.get_or_insert_with(::std::default::Default::default))
      }
      "py_generic_services" => {
        ::pb_jelly::reflection::FieldMut::Value(self.py_generic_services.get_or_insert_with(::std::default::Default::default))
      }
      "php_generic_services" => {
        ::pb_jelly::reflection::FieldMut::Value(self.php_generic_services.get_or_insert_with(::std::default::Default::default))
      }
      "deprecated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated.get_or_insert_with(::std::default::Default::default))
      }
      "cc_enable_arenas" => {
        ::pb_jelly::reflection::FieldMut::Value(self.cc_enable_arenas.get_or_insert_with(::std::default::Default::default))
      }
      "objc_class_prefix" => {
        ::pb_jelly::reflection::FieldMut::Value(self.objc_class_prefix.get_or_insert_with(::std::default::Default::default))
      }
      "csharp_namespace" => {
        ::pb_jelly::reflection::FieldMut::Value(self.csharp_namespace.get_or_insert_with(::std::default::Default::default))
      }
      "swift_prefix" => {
        ::pb_jelly::reflection::FieldMut::Value(self.swift_prefix.get_or_insert_with(::std::default::Default::default))
      }
      "php_class_prefix" => {
        ::pb_jelly::reflection::FieldMut::Value(self.php_class_prefix.get_or_insert_with(::std::default::Default::default))
      }
      "php_namespace" => {
        ::pb_jelly::reflection::FieldMut::Value(self.php_namespace.get_or_insert_with(::std::default::Default::default))
      }
      "php_metadata_namespace" => {
        ::pb_jelly::reflection::FieldMut::Value(self.php_metadata_namespace.get_or_insert_with(::std::default::Default::default))
      }
      "ruby_package" => {
        ::pb_jelly::reflection::FieldMut::Value(self.ruby_package.get_or_insert_with(::std::default::Default::default))
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for FileOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
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
  /// NOTE: Do not set the option in .proto files. Always use the maps syntax
  /// instead. The option should only be implicitly set by the proto compiler
  /// parser.

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
  pub map_entry: ::std::option::Option<bool>,
  /// Enable the legacy handling of JSON field name conflicts.  This lowercases
  /// and strips underscored from the fields before comparison in proto3 only.
  /// The new behavior takes `json_name` into account and applies to proto2 as
  /// well.

  /// This should only be used as a temporary measure against broken builds due
  /// to the change in behavior for JSON field name conflicts.

  /// TODO This is legacy behavior we plan to remove once downstream
  /// teams have had time to migrate.
  pub deprecated_legacy_json_field_conflicts: ::std::option::Option<bool>,
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  pub _extensions: ::pb_jelly::Unrecognized,
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
  pub fn has_deprecated_legacy_json_field_conflicts(&self) -> bool {
    self.deprecated_legacy_json_field_conflicts.is_some()
  }
  pub fn set_deprecated_legacy_json_field_conflicts(&mut self, v: bool) {
    self.deprecated_legacy_json_field_conflicts = Some(v);
  }
  pub fn get_deprecated_legacy_json_field_conflicts(&self) -> bool {
    self.deprecated_legacy_json_field_conflicts.unwrap_or(false)
  }
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
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
      deprecated_legacy_json_field_conflicts: ::std::default::Default::default(),
      features: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref MessageOptions_default: MessageOptions = MessageOptions::default();
}
impl ::pb_jelly::Message for MessageOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "MessageOptions",
      full_name: "google.protobuf.MessageOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "message_set_wire_format",
          full_name: "google.protobuf.MessageOptions.message_set_wire_format",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "no_standard_descriptor_accessor",
          full_name: "google.protobuf.MessageOptions.no_standard_descriptor_accessor",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "deprecated",
          full_name: "google.protobuf.MessageOptions.deprecated",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "map_entry",
          full_name: "google.protobuf.MessageOptions.map_entry",
          index: 3,
          number: 7,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "deprecated_legacy_json_field_conflicts",
          full_name: "google.protobuf.MessageOptions.deprecated_legacy_json_field_conflicts",
          index: 4,
          number: 11,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.MessageOptions.features",
          index: 5,
          number: 12,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.MessageOptions.uninterpreted_option",
          index: 6,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut message_set_wire_format_size = 0;
    if let Some(ref val) = self.message_set_wire_format {
      let l = ::pb_jelly::Message::compute_size(val);
      message_set_wire_format_size += ::pb_jelly::wire_format::serialized_length(1);
      message_set_wire_format_size += l;
    }
    size += message_set_wire_format_size;
    let mut no_standard_descriptor_accessor_size = 0;
    if let Some(ref val) = self.no_standard_descriptor_accessor {
      let l = ::pb_jelly::Message::compute_size(val);
      no_standard_descriptor_accessor_size += ::pb_jelly::wire_format::serialized_length(2);
      no_standard_descriptor_accessor_size += l;
    }
    size += no_standard_descriptor_accessor_size;
    let mut deprecated_size = 0;
    if let Some(ref val) = self.deprecated {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_size += ::pb_jelly::wire_format::serialized_length(3);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut map_entry_size = 0;
    if let Some(ref val) = self.map_entry {
      let l = ::pb_jelly::Message::compute_size(val);
      map_entry_size += ::pb_jelly::wire_format::serialized_length(7);
      map_entry_size += l;
    }
    size += map_entry_size;
    let mut deprecated_legacy_json_field_conflicts_size = 0;
    if let Some(ref val) = self.deprecated_legacy_json_field_conflicts {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_legacy_json_field_conflicts_size += ::pb_jelly::wire_format::serialized_length(11);
      deprecated_legacy_json_field_conflicts_size += l;
    }
    size += deprecated_legacy_json_field_conflicts_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(12);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.message_set_wire_format {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.no_standard_descriptor_accessor {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.deprecated {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.map_entry {
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.deprecated_legacy_json_field_conflicts {
      ::pb_jelly::wire_format::write(11, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(12, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MessageOptions", 1)?;
          self.message_set_wire_format = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MessageOptions", 2)?;
          self.no_standard_descriptor_accessor = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MessageOptions", 3)?;
          self.deprecated = Some(val);
        }
        7 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MessageOptions", 7)?;
          self.map_entry = Some(val);
        }
        11 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MessageOptions", 11)?;
          self.deprecated_legacy_json_field_conflicts = Some(val);
        }
        12 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "MessageOptions", 12)?;
          self.features = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "MessageOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for MessageOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "message_set_wire_format" => {
        ::pb_jelly::reflection::FieldMut::Value(self.message_set_wire_format.get_or_insert_with(::std::default::Default::default))
      }
      "no_standard_descriptor_accessor" => {
        ::pb_jelly::reflection::FieldMut::Value(self.no_standard_descriptor_accessor.get_or_insert_with(::std::default::Default::default))
      }
      "deprecated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated.get_or_insert_with(::std::default::Default::default))
      }
      "map_entry" => {
        ::pb_jelly::reflection::FieldMut::Value(self.map_entry.get_or_insert_with(::std::default::Default::default))
      }
      "deprecated_legacy_json_field_conflicts" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated_legacy_json_field_conflicts.get_or_insert_with(::std::default::Default::default))
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for MessageOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOptions {
  /// The ctype option instructs the C++ code generator to use a different
  /// representation of the field than it normally would.  See the specific
  /// options below.  This option is only implemented to support use of
  /// [ctype=CORD] and [ctype=STRING] (the default) on non-repeated fields of
  /// type "bytes" in the open source release -- sorry, we'll try to include
  /// other types in a future version!
  pub ctype: ::std::option::Option<FieldOptions_CType>,
  /// The packed option can be enabled for repeated primitive fields to enable
  /// a more efficient representation on the wire. Rather than repeatedly
  /// writing the tag and type for each element, the entire array is encoded as
  /// a single length-delimited blob. In proto3, only explicit setting it to
  /// false will avoid using packed encoding.  This option is prohibited in
  /// Editions, but the `repeated_field_encoding` feature can be used to control
  /// the behavior.
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

  /// As of May 2022, lazy verifies the contents of the byte stream during
  /// parsing.  An invalid byte stream will cause the overall parsing to fail.
  pub lazy: ::std::option::Option<bool>,
  /// unverified_lazy does no correctness checks on the byte stream. This should
  /// only be used where lazy with verification is prohibitive for performance
  /// reasons.
  pub unverified_lazy: ::std::option::Option<bool>,
  /// Is this field deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for accessors, or it will be completely ignored; in the very least, this
  /// is a formalization for deprecating fields.
  pub deprecated: ::std::option::Option<bool>,
  /// For Google-internal migration only. Do not use.
  pub weak: ::std::option::Option<bool>,
  /// Indicate that the field value should not be printed out when using debug
  /// formats, e.g. when the field contains sensitive credentials.
  pub debug_redact: ::std::option::Option<bool>,
  pub retention: ::std::option::Option<FieldOptions_OptionRetention>,
  pub targets: ::std::vec::Vec<FieldOptions_OptionTargetType>,
  pub edition_defaults: ::std::vec::Vec<FieldOptions_EditionDefault>,
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  pub _extensions: ::pb_jelly::Unrecognized,
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
  pub fn has_unverified_lazy(&self) -> bool {
    self.unverified_lazy.is_some()
  }
  pub fn set_unverified_lazy(&mut self, v: bool) {
    self.unverified_lazy = Some(v);
  }
  pub fn get_unverified_lazy(&self) -> bool {
    self.unverified_lazy.unwrap_or(false)
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
  pub fn has_debug_redact(&self) -> bool {
    self.debug_redact.is_some()
  }
  pub fn set_debug_redact(&mut self, v: bool) {
    self.debug_redact = Some(v);
  }
  pub fn get_debug_redact(&self) -> bool {
    self.debug_redact.unwrap_or(false)
  }
  pub fn has_retention(&self) -> bool {
    self.retention.is_some()
  }
  pub fn set_retention(&mut self, v: FieldOptions_OptionRetention) {
    self.retention = Some(v);
  }
  pub fn get_retention(&self) -> FieldOptions_OptionRetention {
    self.retention.unwrap_or_default()
  }
  pub fn set_targets(&mut self, v: ::std::vec::Vec<FieldOptions_OptionTargetType>) {
    self.targets = v;
  }
  pub fn take_targets(&mut self) -> ::std::vec::Vec<FieldOptions_OptionTargetType> {
    ::std::mem::take(&mut self.targets)
  }
  pub fn get_targets(&self) -> &[FieldOptions_OptionTargetType] {
    &self.targets
  }
  pub fn mut_targets(&mut self) -> &mut ::std::vec::Vec<FieldOptions_OptionTargetType> {
    &mut self.targets
  }
  pub fn set_edition_defaults(&mut self, v: ::std::vec::Vec<FieldOptions_EditionDefault>) {
    self.edition_defaults = v;
  }
  pub fn take_edition_defaults(&mut self) -> ::std::vec::Vec<FieldOptions_EditionDefault> {
    ::std::mem::take(&mut self.edition_defaults)
  }
  pub fn get_edition_defaults(&self) -> &[FieldOptions_EditionDefault] {
    &self.edition_defaults
  }
  pub fn mut_edition_defaults(&mut self) -> &mut ::std::vec::Vec<FieldOptions_EditionDefault> {
    &mut self.edition_defaults
  }
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
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
      unverified_lazy: Some(false),
      deprecated: Some(false),
      weak: Some(false),
      debug_redact: Some(false),
      retention: ::std::default::Default::default(),
      targets: ::std::default::Default::default(),
      edition_defaults: ::std::default::Default::default(),
      features: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref FieldOptions_default: FieldOptions = FieldOptions::default();
}
impl ::pb_jelly::Message for FieldOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FieldOptions",
      full_name: "google.protobuf.FieldOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "ctype",
          full_name: "google.protobuf.FieldOptions.ctype",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "packed",
          full_name: "google.protobuf.FieldOptions.packed",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "jstype",
          full_name: "google.protobuf.FieldOptions.jstype",
          index: 2,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "lazy",
          full_name: "google.protobuf.FieldOptions.lazy",
          index: 3,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "unverified_lazy",
          full_name: "google.protobuf.FieldOptions.unverified_lazy",
          index: 4,
          number: 15,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "deprecated",
          full_name: "google.protobuf.FieldOptions.deprecated",
          index: 5,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "weak",
          full_name: "google.protobuf.FieldOptions.weak",
          index: 6,
          number: 10,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "debug_redact",
          full_name: "google.protobuf.FieldOptions.debug_redact",
          index: 7,
          number: 16,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "retention",
          full_name: "google.protobuf.FieldOptions.retention",
          index: 8,
          number: 17,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "targets",
          full_name: "google.protobuf.FieldOptions.targets",
          index: 9,
          number: 19,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "edition_defaults",
          full_name: "google.protobuf.FieldOptions.edition_defaults",
          index: 10,
          number: 20,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.FieldOptions.features",
          index: 11,
          number: 21,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.FieldOptions.uninterpreted_option",
          index: 12,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut ctype_size = 0;
    if let Some(ref val) = self.ctype {
      let l = ::pb_jelly::Message::compute_size(val);
      ctype_size += ::pb_jelly::wire_format::serialized_length(1);
      ctype_size += l;
    }
    size += ctype_size;
    let mut packed_size = 0;
    if let Some(ref val) = self.packed {
      let l = ::pb_jelly::Message::compute_size(val);
      packed_size += ::pb_jelly::wire_format::serialized_length(2);
      packed_size += l;
    }
    size += packed_size;
    let mut jstype_size = 0;
    if let Some(ref val) = self.jstype {
      let l = ::pb_jelly::Message::compute_size(val);
      jstype_size += ::pb_jelly::wire_format::serialized_length(6);
      jstype_size += l;
    }
    size += jstype_size;
    let mut lazy_size = 0;
    if let Some(ref val) = self.lazy {
      let l = ::pb_jelly::Message::compute_size(val);
      lazy_size += ::pb_jelly::wire_format::serialized_length(5);
      lazy_size += l;
    }
    size += lazy_size;
    let mut unverified_lazy_size = 0;
    if let Some(ref val) = self.unverified_lazy {
      let l = ::pb_jelly::Message::compute_size(val);
      unverified_lazy_size += ::pb_jelly::wire_format::serialized_length(15);
      unverified_lazy_size += l;
    }
    size += unverified_lazy_size;
    let mut deprecated_size = 0;
    if let Some(ref val) = self.deprecated {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_size += ::pb_jelly::wire_format::serialized_length(3);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut weak_size = 0;
    if let Some(ref val) = self.weak {
      let l = ::pb_jelly::Message::compute_size(val);
      weak_size += ::pb_jelly::wire_format::serialized_length(10);
      weak_size += l;
    }
    size += weak_size;
    let mut debug_redact_size = 0;
    if let Some(ref val) = self.debug_redact {
      let l = ::pb_jelly::Message::compute_size(val);
      debug_redact_size += ::pb_jelly::wire_format::serialized_length(16);
      debug_redact_size += l;
    }
    size += debug_redact_size;
    let mut retention_size = 0;
    if let Some(ref val) = self.retention {
      let l = ::pb_jelly::Message::compute_size(val);
      retention_size += ::pb_jelly::wire_format::serialized_length(17);
      retention_size += l;
    }
    size += retention_size;
    let mut targets_size = 0;
    for val in &self.targets {
      let l = ::pb_jelly::Message::compute_size(val);
      targets_size += ::pb_jelly::wire_format::serialized_length(19);
      targets_size += l;
    }
    size += targets_size;
    let mut edition_defaults_size = 0;
    for val in &self.edition_defaults {
      let l = ::pb_jelly::Message::compute_size(val);
      edition_defaults_size += ::pb_jelly::wire_format::serialized_length(20);
      edition_defaults_size += ::pb_jelly::varint::serialized_length(l as u64);
      edition_defaults_size += l;
    }
    size += edition_defaults_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(21);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.ctype {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.packed {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.deprecated {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.lazy {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.jstype {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.weak {
      ::pb_jelly::wire_format::write(10, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.unverified_lazy {
      ::pb_jelly::wire_format::write(15, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.debug_redact {
      ::pb_jelly::wire_format::write(16, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.retention {
      ::pb_jelly::wire_format::write(17, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.targets {
      ::pb_jelly::wire_format::write(19, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.edition_defaults {
      ::pb_jelly::wire_format::write(20, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(21, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FieldOptions_CType>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 1)?;
          self.ctype = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 2)?;
          self.packed = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FieldOptions_JSType>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 6)?;
          self.jstype = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 5)?;
          self.lazy = Some(val);
        }
        15 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 15)?;
          self.unverified_lazy = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 3)?;
          self.deprecated = Some(val);
        }
        10 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 10)?;
          self.weak = Some(val);
        }
        16 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 16)?;
          self.debug_redact = Some(val);
        }
        17 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FieldOptions_OptionRetention>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 17)?;
          self.retention = Some(val);
        }
        19 => {
          ::pb_jelly::helpers::deserialize_packed::<B, FieldOptions_OptionTargetType>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions", 19, &mut self.targets)?;
        }
        20 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FieldOptions_EditionDefault>(buf, typ, "FieldOptions", 20)?;
          self.edition_defaults.push(val);
        }
        21 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "FieldOptions", 21)?;
          self.features = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "FieldOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FieldOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "ctype" => {
        ::pb_jelly::reflection::FieldMut::Value(self.ctype.get_or_insert_with(::std::default::Default::default))
      }
      "packed" => {
        ::pb_jelly::reflection::FieldMut::Value(self.packed.get_or_insert_with(::std::default::Default::default))
      }
      "jstype" => {
        ::pb_jelly::reflection::FieldMut::Value(self.jstype.get_or_insert_with(::std::default::Default::default))
      }
      "lazy" => {
        ::pb_jelly::reflection::FieldMut::Value(self.lazy.get_or_insert_with(::std::default::Default::default))
      }
      "unverified_lazy" => {
        ::pb_jelly::reflection::FieldMut::Value(self.unverified_lazy.get_or_insert_with(::std::default::Default::default))
      }
      "deprecated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated.get_or_insert_with(::std::default::Default::default))
      }
      "weak" => {
        ::pb_jelly::reflection::FieldMut::Value(self.weak.get_or_insert_with(::std::default::Default::default))
      }
      "debug_redact" => {
        ::pb_jelly::reflection::FieldMut::Value(self.debug_redact.get_or_insert_with(::std::default::Default::default))
      }
      "retention" => {
        ::pb_jelly::reflection::FieldMut::Value(self.retention.get_or_insert_with(::std::default::Default::default))
      }
      "targets" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "edition_defaults" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for FieldOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FieldOptions_EditionDefault {
  pub edition: ::std::option::Option<Edition>,
  /// Textproto value.
  pub value: ::std::option::Option<::std::string::String>,
}
impl FieldOptions_EditionDefault {
  pub fn has_edition(&self) -> bool {
    self.edition.is_some()
  }
  pub fn set_edition(&mut self, v: Edition) {
    self.edition = Some(v);
  }
  pub fn get_edition(&self) -> Edition {
    self.edition.unwrap_or_default()
  }
  pub fn has_value(&self) -> bool {
    self.value.is_some()
  }
  pub fn set_value(&mut self, v: ::std::string::String) {
    self.value = Some(v);
  }
  pub fn take_value(&mut self) -> ::std::string::String {
    self.value.take().unwrap_or_default()
  }
  pub fn get_value(&self) -> &str {
    self.value.as_deref().unwrap_or("")
  }
}
impl ::std::default::Default for FieldOptions_EditionDefault {
  fn default() -> Self {
    FieldOptions_EditionDefault {
      edition: ::std::default::Default::default(),
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FieldOptions_EditionDefault_default: FieldOptions_EditionDefault = FieldOptions_EditionDefault::default();
}
impl ::pb_jelly::Message for FieldOptions_EditionDefault {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FieldOptions_EditionDefault",
      full_name: "google.protobuf.FieldOptions_EditionDefault",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "edition",
          full_name: "google.protobuf.FieldOptions_EditionDefault.edition",
          index: 0,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "value",
          full_name: "google.protobuf.FieldOptions_EditionDefault.value",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut edition_size = 0;
    if let Some(ref val) = self.edition {
      let l = ::pb_jelly::Message::compute_size(val);
      edition_size += ::pb_jelly::wire_format::serialized_length(3);
      edition_size += l;
    }
    size += edition_size;
    let mut value_size = 0;
    if let Some(ref val) = self.value {
      let l = ::pb_jelly::Message::compute_size(val);
      value_size += ::pb_jelly::wire_format::serialized_length(2);
      value_size += ::pb_jelly::varint::serialized_length(l as u64);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.value {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.edition {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, Edition>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FieldOptions_EditionDefault", 3)?;
          self.edition = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "FieldOptions_EditionDefault", 2)?;
          self.value = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FieldOptions_EditionDefault {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "edition" => {
        ::pb_jelly::reflection::FieldMut::Value(self.edition.get_or_insert_with(::std::default::Default::default))
      }
      "value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.value.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct OneofOptions {
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  pub _extensions: ::pb_jelly::Unrecognized,
}
impl OneofOptions {
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
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
      features: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref OneofOptions_default: OneofOptions = OneofOptions::default();
}
impl ::pb_jelly::Message for OneofOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "OneofOptions",
      full_name: "google.protobuf.OneofOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.OneofOptions.features",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.OneofOptions.uninterpreted_option",
          index: 1,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(1);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "OneofOptions", 1)?;
          self.features = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "OneofOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for OneofOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for OneofOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
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
  /// Enable the legacy handling of JSON field name conflicts.  This lowercases
  /// and strips underscored from the fields before comparison in proto3 only.
  /// The new behavior takes `json_name` into account and applies to proto2 as
  /// well.
  /// TODO Remove this legacy behavior once downstream teams have
  /// had time to migrate.
  pub deprecated_legacy_json_field_conflicts: ::std::option::Option<bool>,
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  pub _extensions: ::pb_jelly::Unrecognized,
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
  pub fn has_deprecated_legacy_json_field_conflicts(&self) -> bool {
    self.deprecated_legacy_json_field_conflicts.is_some()
  }
  pub fn set_deprecated_legacy_json_field_conflicts(&mut self, v: bool) {
    self.deprecated_legacy_json_field_conflicts = Some(v);
  }
  pub fn get_deprecated_legacy_json_field_conflicts(&self) -> bool {
    self.deprecated_legacy_json_field_conflicts.unwrap_or(false)
  }
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
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
      deprecated_legacy_json_field_conflicts: ::std::default::Default::default(),
      features: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumOptions_default: EnumOptions = EnumOptions::default();
}
impl ::pb_jelly::Message for EnumOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "EnumOptions",
      full_name: "google.protobuf.EnumOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "allow_alias",
          full_name: "google.protobuf.EnumOptions.allow_alias",
          index: 0,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "deprecated",
          full_name: "google.protobuf.EnumOptions.deprecated",
          index: 1,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "deprecated_legacy_json_field_conflicts",
          full_name: "google.protobuf.EnumOptions.deprecated_legacy_json_field_conflicts",
          index: 2,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.EnumOptions.features",
          index: 3,
          number: 7,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.EnumOptions.uninterpreted_option",
          index: 4,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut allow_alias_size = 0;
    if let Some(ref val) = self.allow_alias {
      let l = ::pb_jelly::Message::compute_size(val);
      allow_alias_size += ::pb_jelly::wire_format::serialized_length(2);
      allow_alias_size += l;
    }
    size += allow_alias_size;
    let mut deprecated_size = 0;
    if let Some(ref val) = self.deprecated {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_size += ::pb_jelly::wire_format::serialized_length(3);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut deprecated_legacy_json_field_conflicts_size = 0;
    if let Some(ref val) = self.deprecated_legacy_json_field_conflicts {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_legacy_json_field_conflicts_size += ::pb_jelly::wire_format::serialized_length(6);
      deprecated_legacy_json_field_conflicts_size += l;
    }
    size += deprecated_legacy_json_field_conflicts_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(7);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.allow_alias {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.deprecated {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.deprecated_legacy_json_field_conflicts {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumOptions", 2)?;
          self.allow_alias = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumOptions", 3)?;
          self.deprecated = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumOptions", 6)?;
          self.deprecated_legacy_json_field_conflicts = Some(val);
        }
        7 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "EnumOptions", 7)?;
          self.features = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "EnumOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for EnumOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "allow_alias" => {
        ::pb_jelly::reflection::FieldMut::Value(self.allow_alias.get_or_insert_with(::std::default::Default::default))
      }
      "deprecated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated.get_or_insert_with(::std::default::Default::default))
      }
      "deprecated_legacy_json_field_conflicts" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated_legacy_json_field_conflicts.get_or_insert_with(::std::default::Default::default))
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for EnumOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EnumValueOptions {
  /// Is this enum value deprecated?
  /// Depending on the target platform, this can emit Deprecated annotations
  /// for the enum value, or it will be completely ignored; in the very least,
  /// this is a formalization for deprecating enum values.
  pub deprecated: ::std::option::Option<bool>,
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// Indicate that fields annotated with this enum value should not be printed
  /// out when using debug formats, e.g. when the field contains sensitive
  /// credentials.
  pub debug_redact: ::std::option::Option<bool>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  pub _extensions: ::pb_jelly::Unrecognized,
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
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn has_debug_redact(&self) -> bool {
    self.debug_redact.is_some()
  }
  pub fn set_debug_redact(&mut self, v: bool) {
    self.debug_redact = Some(v);
  }
  pub fn get_debug_redact(&self) -> bool {
    self.debug_redact.unwrap_or(false)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
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
      features: ::std::default::Default::default(),
      debug_redact: Some(false),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref EnumValueOptions_default: EnumValueOptions = EnumValueOptions::default();
}
impl ::pb_jelly::Message for EnumValueOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "EnumValueOptions",
      full_name: "google.protobuf.EnumValueOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "deprecated",
          full_name: "google.protobuf.EnumValueOptions.deprecated",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.EnumValueOptions.features",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "debug_redact",
          full_name: "google.protobuf.EnumValueOptions.debug_redact",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.EnumValueOptions.uninterpreted_option",
          index: 3,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut deprecated_size = 0;
    if let Some(ref val) = self.deprecated {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_size += ::pb_jelly::wire_format::serialized_length(1);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(2);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut debug_redact_size = 0;
    if let Some(ref val) = self.debug_redact {
      let l = ::pb_jelly::Message::compute_size(val);
      debug_redact_size += ::pb_jelly::wire_format::serialized_length(3);
      debug_redact_size += l;
    }
    size += debug_redact_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.deprecated {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.debug_redact {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumValueOptions", 1)?;
          self.deprecated = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "EnumValueOptions", 2)?;
          self.features = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "EnumValueOptions", 3)?;
          self.debug_redact = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "EnumValueOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for EnumValueOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "deprecated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated.get_or_insert_with(::std::default::Default::default))
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "debug_redact" => {
        ::pb_jelly::reflection::FieldMut::Value(self.debug_redact.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for EnumValueOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ServiceOptions {
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
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
  pub _extensions: ::pb_jelly::Unrecognized,
}
impl ServiceOptions {
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
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
    ::std::mem::take(&mut self.uninterpreted_option)
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
      features: ::std::default::Default::default(),
      deprecated: Some(false),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref ServiceOptions_default: ServiceOptions = ServiceOptions::default();
}
impl ::pb_jelly::Message for ServiceOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "ServiceOptions",
      full_name: "google.protobuf.ServiceOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.ServiceOptions.features",
          index: 0,
          number: 34,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "deprecated",
          full_name: "google.protobuf.ServiceOptions.deprecated",
          index: 1,
          number: 33,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.ServiceOptions.uninterpreted_option",
          index: 2,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(34);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut deprecated_size = 0;
    if let Some(ref val) = self.deprecated {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_size += ::pb_jelly::wire_format::serialized_length(33);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.deprecated {
      ::pb_jelly::wire_format::write(33, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(34, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        34 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "ServiceOptions", 34)?;
          self.features = Some(val);
        }
        33 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "ServiceOptions", 33)?;
          self.deprecated = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "ServiceOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for ServiceOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "deprecated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for ServiceOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
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
  /// Any features defined in the specific edition.
  pub features: ::std::option::Option<FeatureSet>,
  /// The parser stores options it doesn't recognize here. See above.
  pub uninterpreted_option: ::std::vec::Vec<UninterpretedOption>,
  pub _extensions: ::pb_jelly::Unrecognized,
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
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
  pub fn set_uninterpreted_option(&mut self, v: ::std::vec::Vec<UninterpretedOption>) {
    self.uninterpreted_option = v;
  }
  pub fn take_uninterpreted_option(&mut self) -> ::std::vec::Vec<UninterpretedOption> {
    ::std::mem::take(&mut self.uninterpreted_option)
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
      features: ::std::default::Default::default(),
      uninterpreted_option: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref MethodOptions_default: MethodOptions = MethodOptions::default();
}
impl ::pb_jelly::Message for MethodOptions {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "MethodOptions",
      full_name: "google.protobuf.MethodOptions",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "deprecated",
          full_name: "google.protobuf.MethodOptions.deprecated",
          index: 0,
          number: 33,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "idempotency_level",
          full_name: "google.protobuf.MethodOptions.idempotency_level",
          index: 1,
          number: 34,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.MethodOptions.features",
          index: 2,
          number: 35,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "uninterpreted_option",
          full_name: "google.protobuf.MethodOptions.uninterpreted_option",
          index: 3,
          number: 999,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut deprecated_size = 0;
    if let Some(ref val) = self.deprecated {
      let l = ::pb_jelly::Message::compute_size(val);
      deprecated_size += ::pb_jelly::wire_format::serialized_length(33);
      deprecated_size += l;
    }
    size += deprecated_size;
    let mut idempotency_level_size = 0;
    if let Some(ref val) = self.idempotency_level {
      let l = ::pb_jelly::Message::compute_size(val);
      idempotency_level_size += ::pb_jelly::wire_format::serialized_length(34);
      idempotency_level_size += l;
    }
    size += idempotency_level_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(35);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    let mut uninterpreted_option_size = 0;
    for val in &self.uninterpreted_option {
      let l = ::pb_jelly::Message::compute_size(val);
      uninterpreted_option_size += ::pb_jelly::wire_format::serialized_length(999);
      uninterpreted_option_size += ::pb_jelly::varint::serialized_length(l as u64);
      uninterpreted_option_size += l;
    }
    size += uninterpreted_option_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.deprecated {
      ::pb_jelly::wire_format::write(33, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.idempotency_level {
      ::pb_jelly::wire_format::write(34, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(35, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.uninterpreted_option {
      ::pb_jelly::wire_format::write(999, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        33 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MethodOptions", 33)?;
          self.deprecated = Some(val);
        }
        34 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, MethodOptions_IdempotencyLevel>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "MethodOptions", 34)?;
          self.idempotency_level = Some(val);
        }
        35 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "MethodOptions", 35)?;
          self.features = Some(val);
        }
        999 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption>(buf, typ, "MethodOptions", 999)?;
          self.uninterpreted_option.push(val);
        }
        1000..=536870911 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for MethodOptions {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "deprecated" => {
        ::pb_jelly::reflection::FieldMut::Value(self.deprecated.get_or_insert_with(::std::default::Default::default))
      }
      "idempotency_level" => {
        ::pb_jelly::reflection::FieldMut::Value(self.idempotency_level.get_or_insert_with(::std::default::Default::default))
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      "uninterpreted_option" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for MethodOptions {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
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
    ::std::mem::take(&mut self.name)
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
    self.identifier_value.as_deref().unwrap_or("")
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
    self.string_value.as_deref().unwrap_or(&[])
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
    self.aggregate_value.as_deref().unwrap_or("")
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
impl ::pb_jelly::Message for UninterpretedOption {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "UninterpretedOption",
      full_name: "google.protobuf.UninterpretedOption",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.UninterpretedOption.name",
          index: 0,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "identifier_value",
          full_name: "google.protobuf.UninterpretedOption.identifier_value",
          index: 1,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "positive_int_value",
          full_name: "google.protobuf.UninterpretedOption.positive_int_value",
          index: 2,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "negative_int_value",
          full_name: "google.protobuf.UninterpretedOption.negative_int_value",
          index: 3,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "double_value",
          full_name: "google.protobuf.UninterpretedOption.double_value",
          index: 4,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::Fixed64,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "string_value",
          full_name: "google.protobuf.UninterpretedOption.string_value",
          index: 5,
          number: 7,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "aggregate_value",
          full_name: "google.protobuf.UninterpretedOption.aggregate_value",
          index: 6,
          number: 8,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_size = 0;
    for val in &self.name {
      let l = ::pb_jelly::Message::compute_size(val);
      name_size += ::pb_jelly::wire_format::serialized_length(2);
      name_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_size += l;
    }
    size += name_size;
    let mut identifier_value_size = 0;
    if let Some(ref val) = self.identifier_value {
      let l = ::pb_jelly::Message::compute_size(val);
      identifier_value_size += ::pb_jelly::wire_format::serialized_length(3);
      identifier_value_size += ::pb_jelly::varint::serialized_length(l as u64);
      identifier_value_size += l;
    }
    size += identifier_value_size;
    let mut positive_int_value_size = 0;
    if let Some(ref val) = self.positive_int_value {
      let l = ::pb_jelly::Message::compute_size(val);
      positive_int_value_size += ::pb_jelly::wire_format::serialized_length(4);
      positive_int_value_size += l;
    }
    size += positive_int_value_size;
    let mut negative_int_value_size = 0;
    if let Some(ref val) = self.negative_int_value {
      let l = ::pb_jelly::Message::compute_size(val);
      negative_int_value_size += ::pb_jelly::wire_format::serialized_length(5);
      negative_int_value_size += l;
    }
    size += negative_int_value_size;
    let mut double_value_size = 0;
    if let Some(ref val) = self.double_value {
      let l = ::pb_jelly::Message::compute_size(val);
      double_value_size += ::pb_jelly::wire_format::serialized_length(6);
      double_value_size += l;
    }
    size += double_value_size;
    let mut string_value_size = 0;
    if let Some(ref val) = self.string_value {
      let l = ::pb_jelly::Message::compute_size(val);
      string_value_size += ::pb_jelly::wire_format::serialized_length(7);
      string_value_size += ::pb_jelly::varint::serialized_length(l as u64);
      string_value_size += l;
    }
    size += string_value_size;
    let mut aggregate_value_size = 0;
    if let Some(ref val) = self.aggregate_value {
      let l = ::pb_jelly::Message::compute_size(val);
      aggregate_value_size += ::pb_jelly::wire_format::serialized_length(8);
      aggregate_value_size += ::pb_jelly::varint::serialized_length(l as u64);
      aggregate_value_size += l;
    }
    size += aggregate_value_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.name {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.identifier_value {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.positive_int_value {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.negative_int_value {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.double_value {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Fixed64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.string_value {
      ::pb_jelly::wire_format::write(7, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.aggregate_value {
      ::pb_jelly::wire_format::write(8, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, UninterpretedOption_NamePart>(buf, typ, "UninterpretedOption", 2)?;
          self.name.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "UninterpretedOption", 3)?;
          self.identifier_value = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u64>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "UninterpretedOption", 4)?;
          self.positive_int_value = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i64>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "UninterpretedOption", 5)?;
          self.negative_int_value = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, f64>(buf, typ, ::pb_jelly::wire_format::Type::Fixed64, "UninterpretedOption", 6)?;
          self.double_value = Some(val);
        }
        7 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "UninterpretedOption", 7)?;
          self.string_value = Some(val);
        }
        8 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "UninterpretedOption", 8)?;
          self.aggregate_value = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for UninterpretedOption {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "identifier_value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.identifier_value.get_or_insert_with(::std::default::Default::default))
      }
      "positive_int_value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.positive_int_value.get_or_insert_with(::std::default::Default::default))
      }
      "negative_int_value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.negative_int_value.get_or_insert_with(::std::default::Default::default))
      }
      "double_value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.double_value.get_or_insert_with(::std::default::Default::default))
      }
      "string_value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.string_value.get_or_insert_with(::std::default::Default::default))
      }
      "aggregate_value" => {
        ::pb_jelly::reflection::FieldMut::Value(self.aggregate_value.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// The name of the uninterpreted option.  Each string represents a segment in
/// a dot-separated name.  is_extension is true iff a segment represents an
/// extension (denoted with parentheses in options specs in .proto files).
/// E.g.,{ ["foo", false], ["bar.baz", true], ["moo", false] } represents
/// "foo.(bar.baz).moo".
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
    self.name_part.as_deref().unwrap_or("")
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
impl ::pb_jelly::Message for UninterpretedOption_NamePart {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "UninterpretedOption_NamePart",
      full_name: "google.protobuf.UninterpretedOption_NamePart",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name_part",
          full_name: "google.protobuf.UninterpretedOption_NamePart.name_part",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Required,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "is_extension",
          full_name: "google.protobuf.UninterpretedOption_NamePart.is_extension",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Required,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut name_part_size = 0;
    if let Some(ref val) = self.name_part {
      let l = ::pb_jelly::Message::compute_size(val);
      name_part_size += ::pb_jelly::wire_format::serialized_length(1);
      name_part_size += ::pb_jelly::varint::serialized_length(l as u64);
      name_part_size += l;
    }
    size += name_part_size;
    let mut is_extension_size = 0;
    if let Some(ref val) = self.is_extension {
      let l = ::pb_jelly::Message::compute_size(val);
      is_extension_size += ::pb_jelly::wire_format::serialized_length(2);
      is_extension_size += l;
    }
    size += is_extension_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name_part {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.is_extension {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "UninterpretedOption_NamePart", 1)?;
          self.name_part = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, bool>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "UninterpretedOption_NamePart", 2)?;
          self.is_extension = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for UninterpretedOption_NamePart {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "name_part" => {
        ::pb_jelly::reflection::FieldMut::Value(self.name_part.get_or_insert_with(::std::default::Default::default))
      }
      "is_extension" => {
        ::pb_jelly::reflection::FieldMut::Value(self.is_extension.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

// ===================================================================
// Features

/// TODO Enums in C++ gencode (and potentially other languages) are
/// not well scoped.  This means that each of the feature enums below can clash
/// with each other.  The short names we've chosen maximize call-site
/// readability, but leave us very open to this scenario.  A future feature will
/// be designed and implemented to handle this, hopefully before we ever hit a
/// conflict here.
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureSet {
  pub field_presence: ::std::option::Option<FeatureSet_FieldPresence>,
  pub enum_type: ::std::option::Option<FeatureSet_EnumType>,
  pub repeated_field_encoding: ::std::option::Option<FeatureSet_RepeatedFieldEncoding>,
  pub utf8_validation: ::std::option::Option<FeatureSet_Utf8Validation>,
  pub message_encoding: ::std::option::Option<FeatureSet_MessageEncoding>,
  pub json_format: ::std::option::Option<FeatureSet_JsonFormat>,
  pub _extensions: ::pb_jelly::Unrecognized,
}
impl FeatureSet {
  pub fn has_field_presence(&self) -> bool {
    self.field_presence.is_some()
  }
  pub fn set_field_presence(&mut self, v: FeatureSet_FieldPresence) {
    self.field_presence = Some(v);
  }
  pub fn get_field_presence(&self) -> FeatureSet_FieldPresence {
    self.field_presence.unwrap_or_default()
  }
  pub fn has_enum_type(&self) -> bool {
    self.enum_type.is_some()
  }
  pub fn set_enum_type(&mut self, v: FeatureSet_EnumType) {
    self.enum_type = Some(v);
  }
  pub fn get_enum_type(&self) -> FeatureSet_EnumType {
    self.enum_type.unwrap_or_default()
  }
  pub fn has_repeated_field_encoding(&self) -> bool {
    self.repeated_field_encoding.is_some()
  }
  pub fn set_repeated_field_encoding(&mut self, v: FeatureSet_RepeatedFieldEncoding) {
    self.repeated_field_encoding = Some(v);
  }
  pub fn get_repeated_field_encoding(&self) -> FeatureSet_RepeatedFieldEncoding {
    self.repeated_field_encoding.unwrap_or_default()
  }
  pub fn has_utf8_validation(&self) -> bool {
    self.utf8_validation.is_some()
  }
  pub fn set_utf8_validation(&mut self, v: FeatureSet_Utf8Validation) {
    self.utf8_validation = Some(v);
  }
  pub fn get_utf8_validation(&self) -> FeatureSet_Utf8Validation {
    self.utf8_validation.unwrap_or_default()
  }
  pub fn has_message_encoding(&self) -> bool {
    self.message_encoding.is_some()
  }
  pub fn set_message_encoding(&mut self, v: FeatureSet_MessageEncoding) {
    self.message_encoding = Some(v);
  }
  pub fn get_message_encoding(&self) -> FeatureSet_MessageEncoding {
    self.message_encoding.unwrap_or_default()
  }
  pub fn has_json_format(&self) -> bool {
    self.json_format.is_some()
  }
  pub fn set_json_format(&mut self, v: FeatureSet_JsonFormat) {
    self.json_format = Some(v);
  }
  pub fn get_json_format(&self) -> FeatureSet_JsonFormat {
    self.json_format.unwrap_or_default()
  }
}
impl ::std::default::Default for FeatureSet {
  fn default() -> Self {
    FeatureSet {
      field_presence: ::std::default::Default::default(),
      enum_type: ::std::default::Default::default(),
      repeated_field_encoding: ::std::default::Default::default(),
      utf8_validation: ::std::default::Default::default(),
      message_encoding: ::std::default::Default::default(),
      json_format: ::std::default::Default::default(),
      _extensions: ::pb_jelly::Unrecognized::default(),
    }
  }
}
lazy_static! {
  pub static ref FeatureSet_default: FeatureSet = FeatureSet::default();
}
impl ::pb_jelly::Message for FeatureSet {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FeatureSet",
      full_name: "google.protobuf.FeatureSet",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "field_presence",
          full_name: "google.protobuf.FeatureSet.field_presence",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "enum_type",
          full_name: "google.protobuf.FeatureSet.enum_type",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "repeated_field_encoding",
          full_name: "google.protobuf.FeatureSet.repeated_field_encoding",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "utf8_validation",
          full_name: "google.protobuf.FeatureSet.utf8_validation",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "message_encoding",
          full_name: "google.protobuf.FeatureSet.message_encoding",
          index: 4,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "json_format",
          full_name: "google.protobuf.FeatureSet.json_format",
          index: 5,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut field_presence_size = 0;
    if let Some(ref val) = self.field_presence {
      let l = ::pb_jelly::Message::compute_size(val);
      field_presence_size += ::pb_jelly::wire_format::serialized_length(1);
      field_presence_size += l;
    }
    size += field_presence_size;
    let mut enum_type_size = 0;
    if let Some(ref val) = self.enum_type {
      let l = ::pb_jelly::Message::compute_size(val);
      enum_type_size += ::pb_jelly::wire_format::serialized_length(2);
      enum_type_size += l;
    }
    size += enum_type_size;
    let mut repeated_field_encoding_size = 0;
    if let Some(ref val) = self.repeated_field_encoding {
      let l = ::pb_jelly::Message::compute_size(val);
      repeated_field_encoding_size += ::pb_jelly::wire_format::serialized_length(3);
      repeated_field_encoding_size += l;
    }
    size += repeated_field_encoding_size;
    let mut utf8_validation_size = 0;
    if let Some(ref val) = self.utf8_validation {
      let l = ::pb_jelly::Message::compute_size(val);
      utf8_validation_size += ::pb_jelly::wire_format::serialized_length(4);
      utf8_validation_size += l;
    }
    size += utf8_validation_size;
    let mut message_encoding_size = 0;
    if let Some(ref val) = self.message_encoding {
      let l = ::pb_jelly::Message::compute_size(val);
      message_encoding_size += ::pb_jelly::wire_format::serialized_length(5);
      message_encoding_size += l;
    }
    size += message_encoding_size;
    let mut json_format_size = 0;
    if let Some(ref val) = self.json_format {
      let l = ::pb_jelly::Message::compute_size(val);
      json_format_size += ::pb_jelly::wire_format::serialized_length(6);
      json_format_size += l;
    }
    size += json_format_size;
    size += self._extensions.compute_size();
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.field_presence {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.enum_type {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.repeated_field_encoding {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.utf8_validation {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.message_encoding {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.json_format {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    self._extensions.serialize(w)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FeatureSet_FieldPresence>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSet", 1)?;
          self.field_presence = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FeatureSet_EnumType>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSet", 2)?;
          self.enum_type = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FeatureSet_RepeatedFieldEncoding>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSet", 3)?;
          self.repeated_field_encoding = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FeatureSet_Utf8Validation>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSet", 4)?;
          self.utf8_validation = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FeatureSet_MessageEncoding>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSet", 5)?;
          self.message_encoding = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, FeatureSet_JsonFormat>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSet", 6)?;
          self.json_format = Some(val);
        }
        1000..=1000 | 1001..=1001 | 9995..=9999 => {
          self._extensions.gather(field_number, typ, &mut buf)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FeatureSet {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "field_presence" => {
        ::pb_jelly::reflection::FieldMut::Value(self.field_presence.get_or_insert_with(::std::default::Default::default))
      }
      "enum_type" => {
        ::pb_jelly::reflection::FieldMut::Value(self.enum_type.get_or_insert_with(::std::default::Default::default))
      }
      "repeated_field_encoding" => {
        ::pb_jelly::reflection::FieldMut::Value(self.repeated_field_encoding.get_or_insert_with(::std::default::Default::default))
      }
      "utf8_validation" => {
        ::pb_jelly::reflection::FieldMut::Value(self.utf8_validation.get_or_insert_with(::std::default::Default::default))
      }
      "message_encoding" => {
        ::pb_jelly::reflection::FieldMut::Value(self.message_encoding.get_or_insert_with(::std::default::Default::default))
      }
      "json_format" => {
        ::pb_jelly::reflection::FieldMut::Value(self.json_format.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
impl ::pb_jelly::extensions::Extensible for FeatureSet {
  fn _extensions(&self) -> &::pb_jelly::Unrecognized {
    &self._extensions
  }
}

/// A compiled specification for the defaults of a set of features.  These
/// messages are generated from FeatureSet extensions and can be used to seed
/// feature resolution. The resolution with this object becomes a simple search
/// for the closest matching edition, followed by proto merges.
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureSetDefaults {
  pub defaults: ::std::vec::Vec<FeatureSetDefaults_FeatureSetEditionDefault>,
  /// The minimum supported edition (inclusive) when this was constructed.
  /// Editions before this will not have defaults.
  pub minimum_edition: ::std::option::Option<Edition>,
  /// The maximum known edition (inclusive) when this was constructed. Editions
  /// after this will not have reliable defaults.
  pub maximum_edition: ::std::option::Option<Edition>,
}
impl FeatureSetDefaults {
  pub fn set_defaults(&mut self, v: ::std::vec::Vec<FeatureSetDefaults_FeatureSetEditionDefault>) {
    self.defaults = v;
  }
  pub fn take_defaults(&mut self) -> ::std::vec::Vec<FeatureSetDefaults_FeatureSetEditionDefault> {
    ::std::mem::take(&mut self.defaults)
  }
  pub fn get_defaults(&self) -> &[FeatureSetDefaults_FeatureSetEditionDefault] {
    &self.defaults
  }
  pub fn mut_defaults(&mut self) -> &mut ::std::vec::Vec<FeatureSetDefaults_FeatureSetEditionDefault> {
    &mut self.defaults
  }
  pub fn has_minimum_edition(&self) -> bool {
    self.minimum_edition.is_some()
  }
  pub fn set_minimum_edition(&mut self, v: Edition) {
    self.minimum_edition = Some(v);
  }
  pub fn get_minimum_edition(&self) -> Edition {
    self.minimum_edition.unwrap_or_default()
  }
  pub fn has_maximum_edition(&self) -> bool {
    self.maximum_edition.is_some()
  }
  pub fn set_maximum_edition(&mut self, v: Edition) {
    self.maximum_edition = Some(v);
  }
  pub fn get_maximum_edition(&self) -> Edition {
    self.maximum_edition.unwrap_or_default()
  }
}
impl ::std::default::Default for FeatureSetDefaults {
  fn default() -> Self {
    FeatureSetDefaults {
      defaults: ::std::default::Default::default(),
      minimum_edition: ::std::default::Default::default(),
      maximum_edition: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FeatureSetDefaults_default: FeatureSetDefaults = FeatureSetDefaults::default();
}
impl ::pb_jelly::Message for FeatureSetDefaults {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FeatureSetDefaults",
      full_name: "google.protobuf.FeatureSetDefaults",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "defaults",
          full_name: "google.protobuf.FeatureSetDefaults.defaults",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "minimum_edition",
          full_name: "google.protobuf.FeatureSetDefaults.minimum_edition",
          index: 1,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "maximum_edition",
          full_name: "google.protobuf.FeatureSetDefaults.maximum_edition",
          index: 2,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut defaults_size = 0;
    for val in &self.defaults {
      let l = ::pb_jelly::Message::compute_size(val);
      defaults_size += ::pb_jelly::wire_format::serialized_length(1);
      defaults_size += ::pb_jelly::varint::serialized_length(l as u64);
      defaults_size += l;
    }
    size += defaults_size;
    let mut minimum_edition_size = 0;
    if let Some(ref val) = self.minimum_edition {
      let l = ::pb_jelly::Message::compute_size(val);
      minimum_edition_size += ::pb_jelly::wire_format::serialized_length(4);
      minimum_edition_size += l;
    }
    size += minimum_edition_size;
    let mut maximum_edition_size = 0;
    if let Some(ref val) = self.maximum_edition {
      let l = ::pb_jelly::Message::compute_size(val);
      maximum_edition_size += ::pb_jelly::wire_format::serialized_length(5);
      maximum_edition_size += l;
    }
    size += maximum_edition_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.defaults {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.minimum_edition {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.maximum_edition {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSetDefaults_FeatureSetEditionDefault>(buf, typ, "FeatureSetDefaults", 1)?;
          self.defaults.push(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, Edition>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSetDefaults", 4)?;
          self.minimum_edition = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, Edition>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSetDefaults", 5)?;
          self.maximum_edition = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FeatureSetDefaults {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "defaults" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "minimum_edition" => {
        ::pb_jelly::reflection::FieldMut::Value(self.minimum_edition.get_or_insert_with(::std::default::Default::default))
      }
      "maximum_edition" => {
        ::pb_jelly::reflection::FieldMut::Value(self.maximum_edition.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// A map from every known edition with a unique set of defaults to its
/// defaults. Not all editions may be contained here.  For a given edition,
/// the defaults at the closest matching edition ordered at or before it should
/// be used.  This field must be in strict ascending order by edition.
#[derive(Clone, Debug, PartialEq)]
pub struct FeatureSetDefaults_FeatureSetEditionDefault {
  pub edition: ::std::option::Option<Edition>,
  pub features: ::std::option::Option<FeatureSet>,
}
impl FeatureSetDefaults_FeatureSetEditionDefault {
  pub fn has_edition(&self) -> bool {
    self.edition.is_some()
  }
  pub fn set_edition(&mut self, v: Edition) {
    self.edition = Some(v);
  }
  pub fn get_edition(&self) -> Edition {
    self.edition.unwrap_or_default()
  }
  pub fn has_features(&self) -> bool {
    self.features.is_some()
  }
  pub fn set_features(&mut self, v: FeatureSet) {
    self.features = Some(v);
  }
  pub fn take_features(&mut self) -> FeatureSet {
    self.features.take().unwrap_or_default()
  }
  pub fn get_features(&self) -> &FeatureSet {
    self.features.as_ref().unwrap_or(&FeatureSet_default)
  }
}
impl ::std::default::Default for FeatureSetDefaults_FeatureSetEditionDefault {
  fn default() -> Self {
    FeatureSetDefaults_FeatureSetEditionDefault {
      edition: ::std::default::Default::default(),
      features: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FeatureSetDefaults_FeatureSetEditionDefault_default: FeatureSetDefaults_FeatureSetEditionDefault = FeatureSetDefaults_FeatureSetEditionDefault::default();
}
impl ::pb_jelly::Message for FeatureSetDefaults_FeatureSetEditionDefault {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "FeatureSetDefaults_FeatureSetEditionDefault",
      full_name: "google.protobuf.FeatureSetDefaults_FeatureSetEditionDefault",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "edition",
          full_name: "google.protobuf.FeatureSetDefaults_FeatureSetEditionDefault.edition",
          index: 0,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "features",
          full_name: "google.protobuf.FeatureSetDefaults_FeatureSetEditionDefault.features",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut edition_size = 0;
    if let Some(ref val) = self.edition {
      let l = ::pb_jelly::Message::compute_size(val);
      edition_size += ::pb_jelly::wire_format::serialized_length(3);
      edition_size += l;
    }
    size += edition_size;
    let mut features_size = 0;
    if let Some(ref val) = self.features {
      let l = ::pb_jelly::Message::compute_size(val);
      features_size += ::pb_jelly::wire_format::serialized_length(2);
      features_size += ::pb_jelly::varint::serialized_length(l as u64);
      features_size += l;
    }
    size += features_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.features {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.edition {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, Edition>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "FeatureSetDefaults_FeatureSetEditionDefault", 3)?;
          self.edition = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, FeatureSet>(buf, typ, "FeatureSetDefaults_FeatureSetEditionDefault", 2)?;
          self.features = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for FeatureSetDefaults_FeatureSetEditionDefault {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "edition" => {
        ::pb_jelly::reflection::FieldMut::Value(self.edition.get_or_insert_with(::std::default::Default::default))
      }
      "features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.features.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    ::std::mem::take(&mut self.location)
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
impl ::pb_jelly::Message for SourceCodeInfo {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SourceCodeInfo",
      full_name: "google.protobuf.SourceCodeInfo",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "location",
          full_name: "google.protobuf.SourceCodeInfo.location",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut location_size = 0;
    for val in &self.location {
      let l = ::pb_jelly::Message::compute_size(val);
      location_size += ::pb_jelly::wire_format::serialized_length(1);
      location_size += ::pb_jelly::varint::serialized_length(l as u64);
      location_size += l;
    }
    size += location_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.location {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, SourceCodeInfo_Location>(buf, typ, "SourceCodeInfo", 1)?;
          self.location.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for SourceCodeInfo {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "location" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceCodeInfo_Location {
  /// Identifies which part of the FileDescriptorProto was defined at this
  /// location.

  /// Each element is a field number or an index.  They form a path from
  /// the root FileDescriptorProto to the place where the definition occurs.
  /// For example, this path:
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

  ///   // Comment attached to moo.
  ///   //
  ///   // Another line attached to moo.
  ///   optional double moo = 4;

  ///   // Detached comment for corge. This is not leading or trailing comments
  ///   // to moo or corge because there are blank lines separating it from
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
    ::std::mem::take(&mut self.path)
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
    ::std::mem::take(&mut self.span)
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
    self.leading_comments.as_deref().unwrap_or("")
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
    self.trailing_comments.as_deref().unwrap_or("")
  }
  pub fn set_leading_detached_comments(&mut self, v: ::std::vec::Vec<::std::string::String>) {
    self.leading_detached_comments = v;
  }
  pub fn take_leading_detached_comments(&mut self) -> ::std::vec::Vec<::std::string::String> {
    ::std::mem::take(&mut self.leading_detached_comments)
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
impl ::pb_jelly::Message for SourceCodeInfo_Location {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SourceCodeInfo_Location",
      full_name: "google.protobuf.SourceCodeInfo_Location",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "path",
          full_name: "google.protobuf.SourceCodeInfo_Location.path",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "span",
          full_name: "google.protobuf.SourceCodeInfo_Location.span",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "leading_comments",
          full_name: "google.protobuf.SourceCodeInfo_Location.leading_comments",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "trailing_comments",
          full_name: "google.protobuf.SourceCodeInfo_Location.trailing_comments",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "leading_detached_comments",
          full_name: "google.protobuf.SourceCodeInfo_Location.leading_detached_comments",
          index: 4,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut path_size = 0;
    for val in &self.path {
      let l = ::pb_jelly::Message::compute_size(val);
      path_size += l;
    }
    if !self.path.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(1);
      size += ::pb_jelly::varint::serialized_length(path_size as u64);
    }
    size += path_size;
    let mut span_size = 0;
    for val in &self.span {
      let l = ::pb_jelly::Message::compute_size(val);
      span_size += l;
    }
    if !self.span.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(2);
      size += ::pb_jelly::varint::serialized_length(span_size as u64);
    }
    size += span_size;
    let mut leading_comments_size = 0;
    if let Some(ref val) = self.leading_comments {
      let l = ::pb_jelly::Message::compute_size(val);
      leading_comments_size += ::pb_jelly::wire_format::serialized_length(3);
      leading_comments_size += ::pb_jelly::varint::serialized_length(l as u64);
      leading_comments_size += l;
    }
    size += leading_comments_size;
    let mut trailing_comments_size = 0;
    if let Some(ref val) = self.trailing_comments {
      let l = ::pb_jelly::Message::compute_size(val);
      trailing_comments_size += ::pb_jelly::wire_format::serialized_length(4);
      trailing_comments_size += ::pb_jelly::varint::serialized_length(l as u64);
      trailing_comments_size += l;
    }
    size += trailing_comments_size;
    let mut leading_detached_comments_size = 0;
    for val in &self.leading_detached_comments {
      let l = ::pb_jelly::Message::compute_size(val);
      leading_detached_comments_size += ::pb_jelly::wire_format::serialized_length(6);
      leading_detached_comments_size += ::pb_jelly::varint::serialized_length(l as u64);
      leading_detached_comments_size += l;
    }
    size += leading_detached_comments_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.path.is_empty() {
      let mut size = 0;
      for val in &self.path {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.path {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if !self.span.is_empty() {
      let mut size = 0;
      for val in &self.span {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.span {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.leading_comments {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.trailing_comments {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.leading_detached_comments {
      ::pb_jelly::wire_format::write(6, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::helpers::deserialize_packed::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "SourceCodeInfo_Location", 1, &mut self.path)?;
        }
        2 => {
          ::pb_jelly::helpers::deserialize_packed::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "SourceCodeInfo_Location", 2, &mut self.span)?;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SourceCodeInfo_Location", 3)?;
          self.leading_comments = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SourceCodeInfo_Location", 4)?;
          self.trailing_comments = Some(val);
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SourceCodeInfo_Location", 6)?;
          self.leading_detached_comments.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for SourceCodeInfo_Location {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "path" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "span" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "leading_comments" => {
        ::pb_jelly::reflection::FieldMut::Value(self.leading_comments.get_or_insert_with(::std::default::Default::default))
      }
      "trailing_comments" => {
        ::pb_jelly::reflection::FieldMut::Value(self.trailing_comments.get_or_insert_with(::std::default::Default::default))
      }
      "leading_detached_comments" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
    ::std::mem::take(&mut self.annotation)
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
impl ::pb_jelly::Message for GeneratedCodeInfo {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "GeneratedCodeInfo",
      full_name: "google.protobuf.GeneratedCodeInfo",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "annotation",
          full_name: "google.protobuf.GeneratedCodeInfo.annotation",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut annotation_size = 0;
    for val in &self.annotation {
      let l = ::pb_jelly::Message::compute_size(val);
      annotation_size += ::pb_jelly::wire_format::serialized_length(1);
      annotation_size += ::pb_jelly::varint::serialized_length(l as u64);
      annotation_size += l;
    }
    size += annotation_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.annotation {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, GeneratedCodeInfo_Annotation>(buf, typ, "GeneratedCodeInfo", 1)?;
          self.annotation.push(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for GeneratedCodeInfo {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "annotation" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
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
  /// relates to the identified object. The end offset should be one past
  /// the last relevant byte (so the length of the text = end - begin).
  pub end: ::std::option::Option<i32>,
  pub semantic: ::std::option::Option<GeneratedCodeInfo_Annotation_Semantic>,
}
impl GeneratedCodeInfo_Annotation {
  pub fn set_path(&mut self, v: ::std::vec::Vec<i32>) {
    self.path = v;
  }
  pub fn take_path(&mut self) -> ::std::vec::Vec<i32> {
    ::std::mem::take(&mut self.path)
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
    self.source_file.as_deref().unwrap_or("")
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
  pub fn has_semantic(&self) -> bool {
    self.semantic.is_some()
  }
  pub fn set_semantic(&mut self, v: GeneratedCodeInfo_Annotation_Semantic) {
    self.semantic = Some(v);
  }
  pub fn get_semantic(&self) -> GeneratedCodeInfo_Annotation_Semantic {
    self.semantic.unwrap_or_default()
  }
}
impl ::std::default::Default for GeneratedCodeInfo_Annotation {
  fn default() -> Self {
    GeneratedCodeInfo_Annotation {
      path: ::std::default::Default::default(),
      source_file: ::std::default::Default::default(),
      begin: ::std::default::Default::default(),
      end: ::std::default::Default::default(),
      semantic: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref GeneratedCodeInfo_Annotation_default: GeneratedCodeInfo_Annotation = GeneratedCodeInfo_Annotation::default();
}
impl ::pb_jelly::Message for GeneratedCodeInfo_Annotation {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "GeneratedCodeInfo_Annotation",
      full_name: "google.protobuf.GeneratedCodeInfo_Annotation",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "path",
          full_name: "google.protobuf.GeneratedCodeInfo_Annotation.path",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "source_file",
          full_name: "google.protobuf.GeneratedCodeInfo_Annotation.source_file",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "begin",
          full_name: "google.protobuf.GeneratedCodeInfo_Annotation.begin",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "end",
          full_name: "google.protobuf.GeneratedCodeInfo_Annotation.end",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "semantic",
          full_name: "google.protobuf.GeneratedCodeInfo_Annotation.semantic",
          index: 4,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0;
    let mut path_size = 0;
    for val in &self.path {
      let l = ::pb_jelly::Message::compute_size(val);
      path_size += l;
    }
    if !self.path.is_empty() {
      size += ::pb_jelly::wire_format::serialized_length(1);
      size += ::pb_jelly::varint::serialized_length(path_size as u64);
    }
    size += path_size;
    let mut source_file_size = 0;
    if let Some(ref val) = self.source_file {
      let l = ::pb_jelly::Message::compute_size(val);
      source_file_size += ::pb_jelly::wire_format::serialized_length(2);
      source_file_size += ::pb_jelly::varint::serialized_length(l as u64);
      source_file_size += l;
    }
    size += source_file_size;
    let mut begin_size = 0;
    if let Some(ref val) = self.begin {
      let l = ::pb_jelly::Message::compute_size(val);
      begin_size += ::pb_jelly::wire_format::serialized_length(3);
      begin_size += l;
    }
    size += begin_size;
    let mut end_size = 0;
    if let Some(ref val) = self.end {
      let l = ::pb_jelly::Message::compute_size(val);
      end_size += ::pb_jelly::wire_format::serialized_length(4);
      end_size += l;
    }
    size += end_size;
    let mut semantic_size = 0;
    if let Some(ref val) = self.semantic {
      let l = ::pb_jelly::Message::compute_size(val);
      semantic_size += ::pb_jelly::wire_format::serialized_length(5);
      semantic_size += l;
    }
    size += semantic_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if !self.path.is_empty() {
      let mut size = 0;
      for val in &self.path {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
    }
    for val in &self.path {
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.source_file {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.begin {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.end {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.semantic {
      ::pb_jelly::wire_format::write(5, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb_jelly::helpers::deserialize_packed::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "GeneratedCodeInfo_Annotation", 1, &mut self.path)?;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "GeneratedCodeInfo_Annotation", 2)?;
          self.source_file = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "GeneratedCodeInfo_Annotation", 3)?;
          self.begin = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "GeneratedCodeInfo_Annotation", 4)?;
          self.end = Some(val);
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, GeneratedCodeInfo_Annotation_Semantic>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "GeneratedCodeInfo_Annotation", 5)?;
          self.semantic = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for GeneratedCodeInfo_Annotation {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "path" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "source_file" => {
        ::pb_jelly::reflection::FieldMut::Value(self.source_file.get_or_insert_with(::std::default::Default::default))
      }
      "begin" => {
        ::pb_jelly::reflection::FieldMut::Value(self.begin.get_or_insert_with(::std::default::Default::default))
      }
      "end" => {
        ::pb_jelly::reflection::FieldMut::Value(self.end.get_or_insert_with(::std::default::Default::default))
      }
      "semantic" => {
        ::pb_jelly::reflection::FieldMut::Value(self.semantic.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

