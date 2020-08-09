// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SourceContext {
  /// The path-qualified name of the .proto file that contained the associated
  /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
  pub file_name: ::std::string::String,
}
impl ::std::default::Default for SourceContext {
  fn default() -> Self {
    SourceContext {
      file_name: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref SourceContext_default: SourceContext = SourceContext::default();
}
impl ::pb::Message for SourceContext {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut file_name_size = 0;
    if self.file_name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.file_name;
      let l = ::pb::Message::compute_size(val);
      file_name_size += ::pb::wire_format::serialized_length(1);
      file_name_size += ::pb::varint::serialized_length(l as u64);
      file_name_size += l;
    }
    size += file_name_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.file_name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.file_name;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.file_name != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.file_name;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "SourceContext", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.file_name = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for SourceContext {
  const NAME: &'static str = "SourceContext";
  const FULL_NAME: &'static str = "google.protobuf.SourceContext";
}

