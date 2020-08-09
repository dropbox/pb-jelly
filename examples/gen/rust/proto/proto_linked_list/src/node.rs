// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
  pub content: ::std::string::String,
  /// This self-referential node will automatically be put in a Box
  pub next: ::std::option::Option<::std::boxed::Box<Node>>,
}
impl ::std::default::Default for Node {
  fn default() -> Self {
    Node {
      content: ::std::default::Default::default(),
      next: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Node_default: Node = Node::default();
}
impl ::pb::Message for Node {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut content_size = 0;
    if self.content != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.content;
      let l = ::pb::Message::compute_size(val);
      content_size += ::pb::wire_format::serialized_length(1);
      content_size += ::pb::varint::serialized_length(l as u64);
      content_size += l;
    }
    size += content_size;
    let mut next_size = 0;
    for val in &self.next {
      let val = &**val;
      let l = ::pb::Message::compute_size(val);
      next_size += ::pb::wire_format::serialized_length(2);
      next_size += ::pb::varint::serialized_length(l as u64);
      next_size += l;
    }
    size += next_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.content != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.content;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    for val in &self.next {
      let val = &**val;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.content != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.content;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb::Message::compute_size(val);
      ::pb::varint::write(l as u64, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    for val in &self.next {
      let val = &**val;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Node", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.content = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "Node", 2)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: Node = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.next = Some(Box::new(val));
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Node {
  const NAME: &'static str = "Node";
  const FULL_NAME: &'static str = "linked_list.Node";
}

