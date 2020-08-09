// @generated, do not edit
#![allow(clippy::float_cmp)]
#![allow(irrefutable_let_patterns)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]

/// A Duration represents a signed, fixed-length span of time represented
/// as a count of seconds and fractions of seconds at nanosecond
/// resolution. It is independent of any calendar and concepts like "day"
/// or "month". It is related to Timestamp in that the difference between
/// two Timestamp values is a Duration and it can be added or subtracted
/// from a Timestamp. Range is approximately +-10,000 years.

/// # Examples

/// Example 1: Compute Duration from two Timestamps in pseudo code.

///     Timestamp start = ...;
///     Timestamp end = ...;
///     Duration duration = ...;

///     duration.seconds = end.seconds - start.seconds;
///     duration.nanos = end.nanos - start.nanos;

///     if (duration.seconds < 0 && duration.nanos > 0) {
///       duration.seconds += 1;
///       duration.nanos -= 1000000000;
///     } else if (duration.seconds > 0 && duration.nanos < 0) {
///       duration.seconds -= 1;
///       duration.nanos += 1000000000;
///     }

/// Example 2: Compute Timestamp from Timestamp + Duration in pseudo code.

///     Timestamp start = ...;
///     Duration duration = ...;
///     Timestamp end = ...;

///     end.seconds = start.seconds + duration.seconds;
///     end.nanos = start.nanos + duration.nanos;

///     if (end.nanos < 0) {
///       end.seconds -= 1;
///       end.nanos += 1000000000;
///     } else if (end.nanos >= 1000000000) {
///       end.seconds += 1;
///       end.nanos -= 1000000000;
///     }

/// Example 3: Compute Duration from datetime.timedelta in Python.

///     td = datetime.timedelta(days=3, minutes=10)
///     duration = Duration()
///     duration.FromTimedelta(td)

/// # JSON Mapping

/// In JSON format, the Duration type is encoded as a string rather than an
/// object, where the string ends in the suffix "s" (indicating seconds) and
/// is preceded by the number of seconds, with nanoseconds expressed as
/// fractional seconds. For example, 3 seconds with 0 nanoseconds should be
/// encoded in JSON format as "3s", while 3 seconds and 1 nanosecond should
/// be expressed in JSON format as "3.000000001s", and 3 seconds and 1
/// microsecond should be expressed in JSON format as "3.000001s".
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Duration {
  /// Signed seconds of the span of time. Must be from -315,576,000,000
  /// to +315,576,000,000 inclusive. Note: these bounds are computed from:
  /// 60 sec/min * 60 min/hr * 24 hr/day * 365.25 days/year * 10000 years
  pub seconds: i64,
  /// Signed fractions of a second at nanosecond resolution of the span
  /// of time. Durations less than one second are represented with a 0
  /// `seconds` field and a positive or negative `nanos` field. For durations
  /// of one second or more, a non-zero value for the `nanos` field must be
  /// of the same sign as the `seconds` field. Must be from -999,999,999
  /// to +999,999,999 inclusive.
  pub nanos: i32,
}
impl ::std::default::Default for Duration {
  fn default() -> Self {
    Duration {
      seconds: ::std::default::Default::default(),
      nanos: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Duration_default: Duration = Duration::default();
}
impl ::pb::Message for Duration {
  fn compute_size(&self) -> usize  {
    let mut size = 0;
    let mut seconds_size = 0;
    if self.seconds != <i64 as ::std::default::Default>::default() {
      let val = &self.seconds;
      let l = ::pb::Message::compute_size(val);
      seconds_size += ::pb::wire_format::serialized_length(1);
      seconds_size += l;
    }
    size += seconds_size;
    let mut nanos_size = 0;
    if self.nanos != <i32 as ::std::default::Default>::default() {
      let val = &self.nanos;
      let l = ::pb::Message::compute_size(val);
      nanos_size += ::pb::wire_format::serialized_length(2);
      nanos_size += l;
    }
    size += nanos_size;
    size
  }
  fn compute_grpc_slices_size(&self) -> usize  {
    let mut size = 0;
    if self.seconds != <i64 as ::std::default::Default>::default() {
      let val = &self.seconds;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    if self.nanos != <i32 as ::std::default::Default>::default() {
      let val = &self.nanos;
      size += ::pb::Message::compute_grpc_slices_size(val);
    }
    size
  }
  fn serialize<W: ::pb::BlobWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if self.seconds != <i64 as ::std::default::Default>::default() {
      let val = &self.seconds;
      ::pb::wire_format::write(1, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    if self.nanos != <i32 as ::std::default::Default>::default() {
      let val = &self.nanos;
      ::pb::wire_format::write(2, ::pb::wire_format::Type::Varint, w)?;
      ::pb::Message::serialize(val, w)?;
    }
    Ok(())
  }
  fn deserialize<B: ::pb::BlobReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Duration", 1)?;
          let mut val: i64 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.seconds = val;
        }
        2 => {
          ::pb::ensure_wire_format(typ, ::pb::wire_format::Type::Varint, "Duration", 2)?;
          let mut val: i32 = ::std::default::Default::default();
          ::pb::Message::deserialize(&mut val, buf)?;
          self.nanos = val;
        }
        _ => {
          ::pb::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb::MessageDescriptor for Duration {
  const NAME: &'static str = "Duration";
  const FULL_NAME: &'static str = "google.protobuf.Duration";
}

