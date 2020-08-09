// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct HelloTeam {
  pub team_id: u64,
}
impl ::std::default::Default for HelloTeam {
  fn default() -> Self {
    HelloTeam {
      team_id: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref HelloTeam_default: HelloTeam = HelloTeam::default();
}
impl ::pb::Message for HelloTeam {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut team_id_size = 0;
    if self.team_id != <u64 as ::std::default::Default>::default() {
      let val = &self.team_id;
      let l = ::pb::Message::compute_size(val);
      team_id_size += ::pb::wire_format::serialized_length(1);
      team_id_size += l;
    }
    size += team_id_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.team_id != <u64 as ::std::default::Default>::default() {
      let val = &self.team_id;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.team_id != <u64 as ::std::default::Default>::default() {
      let val = &self.team_id;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "HelloTeam", 1)?;
          let mut val: u64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.team_id = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for HelloTeam {
  const NAME: &'static str = "HelloTeam";
  const FULL_NAME: &'static str = "team.HelloTeam";
}

