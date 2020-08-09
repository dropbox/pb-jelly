// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.

///  The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct NullValue(i32);
impl NullValue {
  /// Null value.
  pub const NULL_VALUE: NullValue = NullValue(0);
  pub const KNOWN_VARIANTS: [NullValue; 1] = [NullValue::NULL_VALUE];
  pub const fn value(self) -> i32 {
    self.0
  }
  pub fn into_known(self) -> ::std::option::Option<NullValue_Closed> {
    match self {
      NullValue::NULL_VALUE => Some(NullValue_Closed::NULL_VALUE),
      _ => None,
    }
  }
}
impl ::std::default::Default for NullValue {
  fn default() -> Self {
    NullValue::NULL_VALUE
  }
}
impl From<NullValue> for i32 {
  fn from(v: NullValue) -> i32 {
    v.0
  }
}
impl From<i32> for NullValue {
  fn from(v: i32) -> NullValue {
    NullValue(v)
  }
}
impl From<NullValue_Closed> for NullValue {
  fn from(v: NullValue_Closed) -> NullValue {
    NullValue(v as i32)
  }
}
impl ::pb::ProtoEnum for NullValue {
}
impl ::pb::OpenProtoEnum for NullValue {
  fn name(self) -> ::std::option::Option<&'static str> {
    match self {
      NullValue::NULL_VALUE => Some("NULL_VALUE"),
      _ => None,
    }
  }
  fn is_known(self) -> bool {
    match self {
      NullValue::NULL_VALUE => true,
      _ => false,
    }
  }
}
impl ::std::fmt::Debug for NullValue {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    use ::pb::OpenProtoEnum;
    match self.name() {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.

///  The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum NullValue_Closed {
  /// Null value.
  NULL_VALUE = 0,
}
impl NullValue_Closed {
  pub const KNOWN_VARIANTS: [NullValue_Closed; 1] = [NullValue_Closed::NULL_VALUE];
}
impl ::std::default::Default for NullValue_Closed {
  fn default() -> Self {
    NullValue_Closed::NULL_VALUE
  }
}
impl From<NullValue_Closed> for i32 {
  fn from(v: NullValue_Closed) -> i32 {
    match v {
      NullValue_Closed::NULL_VALUE => 0,
    }
  }
}
impl ::std::convert::TryFrom<i32> for NullValue_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(NullValue_Closed::NULL_VALUE),
      _ => Err(v),
    }
  }
}
impl ::pb::ProtoEnum for NullValue_Closed {
}
impl ::pb::ClosedProtoEnum for NullValue_Closed {
  fn name(self) -> &'static str {
    match self {
      NullValue_Closed::NULL_VALUE => "NULL_VALUE",
    }
  }
}

/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.

/// The JSON representation for `Struct` is JSON object.
#[derive(Clone, Debug, PartialEq)]
pub struct Struct {
  /// Unordered map of dynamically typed values.
  pub fields: ::std::vec::Vec<Struct_FieldsEntry>,
}
impl ::std::default::Default for Struct {
  fn default() -> Self {
    Struct {
      fields: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Struct_default: Struct = Struct::default();
}
impl ::pb::Message for Struct {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut fields_size = 0;
    for val in &self.fields {
      let l = ::pb::Message::compute_size(val);
      fields_size += ::pb::wire_format::serialized_length(1);
      fields_size += ::pb::varint::serialized_length(l as u64);
      fields_size += l;
    }
    size += fields_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.fields {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.fields {
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Struct", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Struct_FieldsEntry = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.fields.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Struct {
  const NAME: &'static str = "Struct";
  const FULL_NAME: &'static str = "google.protobuf.Struct";
}

#[derive(Clone, Debug, PartialEq)]
pub struct Struct_FieldsEntry {
  pub key: ::std::string::String,
  pub value: ::std::option::Option<Value>,
}
impl ::std::default::Default for Struct_FieldsEntry {
  fn default() -> Self {
    Struct_FieldsEntry {
      key: ::std::default::Default::default(),
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Struct_FieldsEntry_default: Struct_FieldsEntry = Struct_FieldsEntry::default();
}
impl ::pb::Message for Struct_FieldsEntry {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut key_size = 0;
    if self.key != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.key;
      let l = ::pb::Message::compute_size(val);
      key_size += ::pb::wire_format::serialized_length(1);
      key_size += ::pb::varint::serialized_length(l as u64);
      key_size += l;
    }
    size += key_size;
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
    if self.key != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.key;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.value {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.key != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.key;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Struct_FieldsEntry", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.key = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Struct_FieldsEntry", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Value = ::std::default::Default::default();
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
impl ::pb::MessageDescriptor for Struct_FieldsEntry {
  const NAME: &'static str = "Struct_FieldsEntry";
  const FULL_NAME: &'static str = "google.protobuf.Struct_FieldsEntry";
}

/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of that
/// variants, absence of any variant indicates an error.

/// The JSON representation for `Value` is JSON value.
#[derive(Clone, Debug, PartialEq)]
pub struct Value {
  /// Represents a null value.
  /// Represents a double value.
  /// Represents a string value.
  /// Represents a boolean value.
  /// Represents a structured value.
  /// Represents a repeated `Value`.
  pub kind: ::std::option::Option<Value_Kind>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Value_Kind {
  NullValue(NullValue),
  NumberValue(f64),
  StringValue(::std::string::String),
  BoolValue(bool),
  StructValue(Struct),
  ListValue(ListValue),
}
impl ::std::default::Default for Value {
  fn default() -> Self {
    Value {
      kind: None,
    }
  }
}
lazy_static! {
  pub static ref Value_default: Value = Value::default();
}
impl ::pb::Message for Value {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut null_value_size = 0;
    if let Some(Value_Kind::NullValue(ref val)) = self.kind {
      let l = ::pb::Message::compute_size(val);
      null_value_size += ::pb::wire_format::serialized_length(1);
      null_value_size += l;
    }
    size += null_value_size;
    let mut number_value_size = 0;
    if let Some(Value_Kind::NumberValue(ref val)) = self.kind {
      let l = ::pb::Message::compute_size(val);
      number_value_size += ::pb::wire_format::serialized_length(2);
      number_value_size += l;
    }
    size += number_value_size;
    let mut string_value_size = 0;
    if let Some(Value_Kind::StringValue(ref val)) = self.kind {
      let l = ::pb::Message::compute_size(val);
      string_value_size += ::pb::wire_format::serialized_length(3);
      string_value_size += ::pb::varint::serialized_length(l as u64);
      string_value_size += l;
    }
    size += string_value_size;
    let mut bool_value_size = 0;
    if let Some(Value_Kind::BoolValue(ref val)) = self.kind {
      let l = ::pb::Message::compute_size(val);
      bool_value_size += ::pb::wire_format::serialized_length(4);
      bool_value_size += l;
    }
    size += bool_value_size;
    let mut struct_value_size = 0;
    if let Some(Value_Kind::StructValue(ref val)) = self.kind {
      let l = ::pb::Message::compute_size(val);
      struct_value_size += ::pb::wire_format::serialized_length(5);
      struct_value_size += ::pb::varint::serialized_length(l as u64);
      struct_value_size += l;
    }
    size += struct_value_size;
    let mut list_value_size = 0;
    if let Some(Value_Kind::ListValue(ref val)) = self.kind {
      let l = ::pb::Message::compute_size(val);
      list_value_size += ::pb::wire_format::serialized_length(6);
      list_value_size += ::pb::varint::serialized_length(l as u64);
      list_value_size += l;
    }
    size += list_value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if let Some(Value_Kind::NullValue(ref val)) = self.kind {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if let Some(Value_Kind::NumberValue(ref val)) = self.kind {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if let Some(Value_Kind::StringValue(ref val)) = self.kind {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if let Some(Value_Kind::BoolValue(ref val)) = self.kind {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if let Some(Value_Kind::StructValue(ref val)) = self.kind {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if let Some(Value_Kind::ListValue(ref val)) = self.kind {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(Value_Kind::NullValue(ref val)) = self.kind {
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if let Some(Value_Kind::NumberValue(ref val)) = self.kind {
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Fixed64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if let Some(Value_Kind::StringValue(ref val)) = self.kind {
      ::pb::wire_format::write(3, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if let Some(Value_Kind::BoolValue(ref val)) = self.kind {
      ::pb::wire_format::write(4, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if let Some(Value_Kind::StructValue(ref val)) = self.kind {
      ::pb::wire_format::write(5, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if let Some(Value_Kind::ListValue(ref val)) = self.kind {
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Value", 1)?;
          let mut val: NullValue = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.kind = Some(Value_Kind::NullValue(val));
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Fixed64, "Value", 2)?;
          let mut val: f64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.kind = Some(Value_Kind::NumberValue(val));
        }
        3 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Value", 3)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.kind = Some(Value_Kind::StringValue(val));
        }
        4 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Value", 4)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.kind = Some(Value_Kind::BoolValue(val));
        }
        5 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Value", 5)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Struct = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.kind = Some(Value_Kind::StructValue(val));
        }
        6 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Value", 6)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ListValue = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.kind = Some(Value_Kind::ListValue(val));
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Value {
  const NAME: &'static str = "Value";
  const FULL_NAME: &'static str = "google.protobuf.Value";
}

/// `ListValue` is a wrapper around a repeated field of values.

/// The JSON representation for `ListValue` is JSON array.
#[derive(Clone, Debug, PartialEq)]
pub struct ListValue {
  /// Repeated field of dynamically typed values.
  pub values: ::std::vec::Vec<Value>,
}
impl ::std::default::Default for ListValue {
  fn default() -> Self {
    ListValue {
      values: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref ListValue_default: ListValue = ListValue::default();
}
impl ::pb::Message for ListValue {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut values_size = 0;
    for val in &self.values {
      let l = ::pb::Message::compute_size(val);
      values_size += ::pb::wire_format::serialized_length(1);
      values_size += ::pb::varint::serialized_length(l as u64);
      values_size += l;
    }
    size += values_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    for val in &self.values {
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.values {
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "ListValue", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Value = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.values.push(val);
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for ListValue {
  const NAME: &'static str = "ListValue";
  const FULL_NAME: &'static str = "google.protobuf.ListValue";
}

