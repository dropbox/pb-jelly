// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct NewYorkCity {
  pub num_windows: u64,
}
impl ::std::default::Default for NewYorkCity {
  fn default() -> Self {
    NewYorkCity {
      num_windows: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref NewYorkCity_default: NewYorkCity = NewYorkCity::default();
}
impl ::pb::Message for NewYorkCity {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut num_windows_size = 0;
    if self.num_windows != <u64 as ::std::default::Default>::default() {
      let val = &self.num_windows;
      let l = ::pb::Message::compute_size(val);
      num_windows_size += ::pb::wire_format::serialized_length(1);
      num_windows_size += l;
    }
    size += num_windows_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.num_windows != <u64 as ::std::default::Default>::default() {
      let val = &self.num_windows;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.num_windows != <u64 as ::std::default::Default>::default() {
      let val = &self.num_windows;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "NewYorkCity", 1)?;
          let mut val: u64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.num_windows = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for NewYorkCity {
  const NAME: &'static str = "NewYorkCity";
  const FULL_NAME: &'static str = "basic.NewYorkCity";
}

