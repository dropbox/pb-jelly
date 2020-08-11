// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:

///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }

/// The JSON representation for `Empty` is empty JSON object `{}`.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Empty {
}
impl ::std::default::Default for Empty {
  fn default() -> Self {
    Empty {
    }
  }
}
lazy_static! {
  pub static ref Empty_default: Empty = Empty::default();
}
impl ::pb::Message for Empty {
  fn compute_size(&self) -> usize  {
    0
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    0
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Empty {
  const NAME: &'static str = "Empty";
  const FULL_NAME: &'static str = "google.protobuf.Empty";
}

