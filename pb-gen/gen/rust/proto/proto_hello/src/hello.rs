// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HelloWorld {
  pub msg: ::std::string::String,
}
impl ::std::default::Default for HelloWorld {
  fn default() -> Self {
    HelloWorld {
      msg: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref HelloWorld_default: HelloWorld = HelloWorld::default();
}
impl ::pb::Message for HelloWorld {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut msg_size = 0;
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      let l = ::pb::Message::compute_size(val);
      msg_size += ::pb::wire_format::serialized_length(1);
      msg_size += ::pb::varint::serialized_length(l as u64);
      msg_size += l;
    }
    size += msg_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.msg != <::std::string::String as ::std::default::Default>::default() {
      let val = &self.msg;
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
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::LengthDelimited, "HelloWorld", 1)?;
          let len = ::pb::varint::ensure_read(&mut buf)?;
          let mut next = ::pb::ensure_split(buf, len as usize)?;
          let mut val: ::std::string::String = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, &mut next)?;
          self.msg = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for HelloWorld {
  const NAME: &'static str = "HelloWorld";
  const FULL_NAME: &'static str = "hello.HelloWorld";
}

