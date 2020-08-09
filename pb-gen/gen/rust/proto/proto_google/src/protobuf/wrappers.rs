// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// Wrapper message for `double`.

/// The JSON representation for `DoubleValue` is JSON number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DoubleValue {
  /// The double value.
  pub value: f64,
}
impl ::std::default::Default for DoubleValue {
  fn default() -> Self {
    DoubleValue {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref DoubleValue_default: DoubleValue = DoubleValue::default();
}
impl ::pb::Message for DoubleValue {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <f64 as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <f64 as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <f64 as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Fixed64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Fixed64, "DoubleValue", 1)?;
          let mut val: f64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
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
impl ::pb::MessageDescriptor for DoubleValue {
  const NAME: &'static str = "DoubleValue";
  const FULL_NAME: &'static str = "google.protobuf.DoubleValue";
}

/// Wrapper message for `float`.

/// The JSON representation for `FloatValue` is JSON number.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FloatValue {
  /// The float value.
  pub value: f32,
}
impl ::std::default::Default for FloatValue {
  fn default() -> Self {
    FloatValue {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref FloatValue_default: FloatValue = FloatValue::default();
}
impl ::pb::Message for FloatValue {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <f32 as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <f32 as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <f32 as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Fixed32, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Fixed32, "FloatValue", 1)?;
          let mut val: f32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
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
impl ::pb::MessageDescriptor for FloatValue {
  const NAME: &'static str = "FloatValue";
  const FULL_NAME: &'static str = "google.protobuf.FloatValue";
}

/// Wrapper message for `int64`.

/// The JSON representation for `Int64Value` is JSON string.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int64Value {
  /// The int64 value.
  pub value: i64,
}
impl ::std::default::Default for Int64Value {
  fn default() -> Self {
    Int64Value {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Int64Value_default: Int64Value = Int64Value::default();
}
impl ::pb::Message for Int64Value {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <i64 as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <i64 as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <i64 as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Int64Value", 1)?;
          let mut val: i64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
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
impl ::pb::MessageDescriptor for Int64Value {
  const NAME: &'static str = "Int64Value";
  const FULL_NAME: &'static str = "google.protobuf.Int64Value";
}

/// Wrapper message for `uint64`.

/// The JSON representation for `UInt64Value` is JSON string.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UInt64Value {
  /// The uint64 value.
  pub value: u64,
}
impl ::std::default::Default for UInt64Value {
  fn default() -> Self {
    UInt64Value {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref UInt64Value_default: UInt64Value = UInt64Value::default();
}
impl ::pb::Message for UInt64Value {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <u64 as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <u64 as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <u64 as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "UInt64Value", 1)?;
          let mut val: u64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
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
impl ::pb::MessageDescriptor for UInt64Value {
  const NAME: &'static str = "UInt64Value";
  const FULL_NAME: &'static str = "google.protobuf.UInt64Value";
}

/// Wrapper message for `int32`.

/// The JSON representation for `Int32Value` is JSON number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Int32Value {
  /// The int32 value.
  pub value: i32,
}
impl ::std::default::Default for Int32Value {
  fn default() -> Self {
    Int32Value {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Int32Value_default: Int32Value = Int32Value::default();
}
impl ::pb::Message for Int32Value {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <i32 as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <i32 as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <i32 as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Int32Value", 1)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
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
impl ::pb::MessageDescriptor for Int32Value {
  const NAME: &'static str = "Int32Value";
  const FULL_NAME: &'static str = "google.protobuf.Int32Value";
}

/// Wrapper message for `uint32`.

/// The JSON representation for `UInt32Value` is JSON number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UInt32Value {
  /// The uint32 value.
  pub value: u32,
}
impl ::std::default::Default for UInt32Value {
  fn default() -> Self {
    UInt32Value {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref UInt32Value_default: UInt32Value = UInt32Value::default();
}
impl ::pb::Message for UInt32Value {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <u32 as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <u32 as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <u32 as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "UInt32Value", 1)?;
          let mut val: u32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
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
impl ::pb::MessageDescriptor for UInt32Value {
  const NAME: &'static str = "UInt32Value";
  const FULL_NAME: &'static str = "google.protobuf.UInt32Value";
}

/// Wrapper message for `bool`.

/// The JSON representation for `BoolValue` is JSON `true` and `false`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoolValue {
  /// The bool value.
  pub value: bool,
}
impl ::std::default::Default for BoolValue {
  fn default() -> Self {
    BoolValue {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref BoolValue_default: BoolValue = BoolValue::default();
}
impl ::pb::Message for BoolValue {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <bool as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <bool as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <bool as ::std::default::Default>::default() {
      let val = &self.value;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "BoolValue", 1)?;
          let mut val: bool = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
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
impl ::pb::MessageDescriptor for BoolValue {
  const NAME: &'static str = "BoolValue";
  const FULL_NAME: &'static str = "google.protobuf.BoolValue";
}

/// Wrapper message for `string`.

/// The JSON representation for `StringValue` is JSON string.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StringValue {
  /// The string value.
  pub value: ::std::string::String,
}
impl ::std::default::Default for StringValue {
  fn default() -> Self {
    StringValue {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref StringValue_default: StringValue = StringValue::default();
}
impl ::pb::Message for StringValue {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += ::pb::varint::serialized_length(l as u64);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.value;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "StringValue", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
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
impl ::pb::MessageDescriptor for StringValue {
  const NAME: &'static str = "StringValue";
  const FULL_NAME: &'static str = "google.protobuf.StringValue";
}

/// Wrapper message for `bytes`.

/// The JSON representation for `BytesValue` is JSON string.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BytesValue {
  /// The bytes value.
  pub value: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for BytesValue {
  fn default() -> Self {
    BytesValue {
      value: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref BytesValue_default: BytesValue = BytesValue::default();
}
impl ::pb::Message for BytesValue {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut value_size = 0;
    if self.value != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.value;
      let l = ::pb::Message::compute_size(val);
      value_size += ::pb::wire_format::serialized_length(1);
      value_size += ::pb::varint::serialized_length(l as u64);
      value_size += l;
    }
    size += value_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.value != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.value;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.value != <::std::vec::Vec<u8> as ::std::default::Default>::default() {
      let val = &self.value;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "BytesValue", 1)?;
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
impl ::pb::MessageDescriptor for BytesValue {
  const NAME: &'static str = "BytesValue";
  const FULL_NAME: &'static str = "google.protobuf.BytesValue";
}

