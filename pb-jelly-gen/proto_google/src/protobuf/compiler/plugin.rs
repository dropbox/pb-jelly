// @generated, do not edit
/// Sync with code_generator.h.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct CodeGeneratorResponse_Feature(i32);
impl CodeGeneratorResponse_Feature {
  pub const FEATURE_NONE: CodeGeneratorResponse_Feature = CodeGeneratorResponse_Feature(0);
  pub const FEATURE_PROTO3_OPTIONAL: CodeGeneratorResponse_Feature = CodeGeneratorResponse_Feature(1);
  pub const FEATURE_SUPPORTS_EDITIONS: CodeGeneratorResponse_Feature = CodeGeneratorResponse_Feature(2);
  pub const KNOWN_VARIANTS: [CodeGeneratorResponse_Feature; 3] = [CodeGeneratorResponse_Feature::FEATURE_NONE, CodeGeneratorResponse_Feature::FEATURE_PROTO3_OPTIONAL, CodeGeneratorResponse_Feature::FEATURE_SUPPORTS_EDITIONS];
  pub const fn value(self) -> i32 {
    self.0
  }
}
impl ::std::default::Default for CodeGeneratorResponse_Feature {
  fn default() -> Self {
    CodeGeneratorResponse_Feature::FEATURE_NONE
  }
}
impl From<CodeGeneratorResponse_Feature> for i32 {
  fn from(v: CodeGeneratorResponse_Feature) -> i32 {
    v.0
  }
}
impl From<i32> for CodeGeneratorResponse_Feature {
  fn from(v: i32) -> CodeGeneratorResponse_Feature {
    CodeGeneratorResponse_Feature(v)
  }
}
impl From<CodeGeneratorResponse_Feature_Closed> for CodeGeneratorResponse_Feature {
  fn from(v: CodeGeneratorResponse_Feature_Closed) -> CodeGeneratorResponse_Feature {
    CodeGeneratorResponse_Feature(v as i32)
  }
}
impl ::pb_jelly::ProtoEnum for CodeGeneratorResponse_Feature {
}
impl ::pb_jelly::OpenProtoEnum for CodeGeneratorResponse_Feature {
  type Closed = CodeGeneratorResponse_Feature_Closed;
  fn into_known(self) -> ::std::option::Option<CodeGeneratorResponse_Feature_Closed> {
    match self {
      CodeGeneratorResponse_Feature::FEATURE_NONE => Some(CodeGeneratorResponse_Feature_Closed::FEATURE_NONE),
      CodeGeneratorResponse_Feature::FEATURE_PROTO3_OPTIONAL => Some(CodeGeneratorResponse_Feature_Closed::FEATURE_PROTO3_OPTIONAL),
      CodeGeneratorResponse_Feature::FEATURE_SUPPORTS_EDITIONS => Some(CodeGeneratorResponse_Feature_Closed::FEATURE_SUPPORTS_EDITIONS),
      _ => None,
    }
  }
}
impl ::std::fmt::Debug for CodeGeneratorResponse_Feature {
  fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
    match <Self as ::pb_jelly::OpenProtoEnum>::name(*self) {
      Some(s) => write!(f, "{}", s),
      None => write!(f, "Unknown({})", self.0),
    }
  }
}
/// Sync with code_generator.h.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(i32)]
pub enum CodeGeneratorResponse_Feature_Closed {
  FEATURE_NONE = 0,
  FEATURE_PROTO3_OPTIONAL = 1,
  FEATURE_SUPPORTS_EDITIONS = 2,
}
impl CodeGeneratorResponse_Feature_Closed {
  pub const KNOWN_VARIANTS: [CodeGeneratorResponse_Feature_Closed; 3] = [CodeGeneratorResponse_Feature_Closed::FEATURE_NONE, CodeGeneratorResponse_Feature_Closed::FEATURE_PROTO3_OPTIONAL, CodeGeneratorResponse_Feature_Closed::FEATURE_SUPPORTS_EDITIONS];
}
impl ::std::default::Default for CodeGeneratorResponse_Feature_Closed {
  fn default() -> Self {
    CodeGeneratorResponse_Feature_Closed::FEATURE_NONE
  }
}
impl From<CodeGeneratorResponse_Feature_Closed> for i32 {
  fn from(v: CodeGeneratorResponse_Feature_Closed) -> i32 {
    match v {
      CodeGeneratorResponse_Feature_Closed::FEATURE_NONE => 0,
      CodeGeneratorResponse_Feature_Closed::FEATURE_PROTO3_OPTIONAL => 1,
      CodeGeneratorResponse_Feature_Closed::FEATURE_SUPPORTS_EDITIONS => 2,
    }
  }
}
impl ::std::convert::TryFrom<i32> for CodeGeneratorResponse_Feature_Closed {
  type Error = i32;
  fn try_from(v: i32) -> ::std::result::Result<Self, i32> {
    match v {
      0 => Ok(CodeGeneratorResponse_Feature_Closed::FEATURE_NONE),
      1 => Ok(CodeGeneratorResponse_Feature_Closed::FEATURE_PROTO3_OPTIONAL),
      2 => Ok(CodeGeneratorResponse_Feature_Closed::FEATURE_SUPPORTS_EDITIONS),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for CodeGeneratorResponse_Feature_Closed {
}
impl ::pb_jelly::ClosedProtoEnum for CodeGeneratorResponse_Feature_Closed {
  fn name(self) -> &'static str {
    match self {
      CodeGeneratorResponse_Feature_Closed::FEATURE_NONE => "FEATURE_NONE",
      CodeGeneratorResponse_Feature_Closed::FEATURE_PROTO3_OPTIONAL => "FEATURE_PROTO3_OPTIONAL",
      CodeGeneratorResponse_Feature_Closed::FEATURE_SUPPORTS_EDITIONS => "FEATURE_SUPPORTS_EDITIONS",
    }
  }
}

/// The version number of protocol compiler.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Version {
  pub major: ::std::option::Option<i32>,
  pub minor: ::std::option::Option<i32>,
  pub patch: ::std::option::Option<i32>,
  /// A suffix for alpha, beta or rc release, e.g., "alpha-1", "rc2". It should
  /// be empty for mainline stable releases.
  pub suffix: ::std::option::Option<::std::string::String>,
}
impl Version {
  pub fn has_major(&self) -> bool {
    self.major.is_some()
  }
  pub fn set_major(&mut self, v: i32) {
    self.major = Some(v);
  }
  pub fn get_major(&self) -> i32 {
    self.major.unwrap_or(0)
  }
  pub fn has_minor(&self) -> bool {
    self.minor.is_some()
  }
  pub fn set_minor(&mut self, v: i32) {
    self.minor = Some(v);
  }
  pub fn get_minor(&self) -> i32 {
    self.minor.unwrap_or(0)
  }
  pub fn has_patch(&self) -> bool {
    self.patch.is_some()
  }
  pub fn set_patch(&mut self, v: i32) {
    self.patch = Some(v);
  }
  pub fn get_patch(&self) -> i32 {
    self.patch.unwrap_or(0)
  }
  pub fn has_suffix(&self) -> bool {
    self.suffix.is_some()
  }
  pub fn set_suffix(&mut self, v: ::std::string::String) {
    self.suffix = Some(v);
  }
  pub fn take_suffix(&mut self) -> ::std::string::String {
    self.suffix.take().unwrap_or_default()
  }
  pub fn get_suffix(&self) -> &str {
    self.suffix.as_deref().unwrap_or("")
  }
}
impl ::std::default::Default for Version {
  fn default() -> Self {
    Version {
      major: ::std::default::Default::default(),
      minor: ::std::default::Default::default(),
      patch: ::std::default::Default::default(),
      suffix: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref Version_default: Version = Version::default();
}
impl ::pb_jelly::Message for Version {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Version",
      full_name: "google.protobuf.compiler.Version",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "major",
          full_name: "google.protobuf.compiler.Version.major",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "minor",
          full_name: "google.protobuf.compiler.Version.minor",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "patch",
          full_name: "google.protobuf.compiler.Version.patch",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "suffix",
          full_name: "google.protobuf.compiler.Version.suffix",
          index: 3,
          number: 4,
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
    let mut major_size = 0;
    if let Some(ref val) = self.major {
      let l = ::pb_jelly::Message::compute_size(val);
      major_size += ::pb_jelly::wire_format::serialized_length(1);
      major_size += l;
    }
    size += major_size;
    let mut minor_size = 0;
    if let Some(ref val) = self.minor {
      let l = ::pb_jelly::Message::compute_size(val);
      minor_size += ::pb_jelly::wire_format::serialized_length(2);
      minor_size += l;
    }
    size += minor_size;
    let mut patch_size = 0;
    if let Some(ref val) = self.patch {
      let l = ::pb_jelly::Message::compute_size(val);
      patch_size += ::pb_jelly::wire_format::serialized_length(3);
      patch_size += l;
    }
    size += patch_size;
    let mut suffix_size = 0;
    if let Some(ref val) = self.suffix {
      let l = ::pb_jelly::Message::compute_size(val);
      suffix_size += ::pb_jelly::wire_format::serialized_length(4);
      suffix_size += ::pb_jelly::varint::serialized_length(l as u64);
      suffix_size += l;
    }
    size += suffix_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.major {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.minor {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.patch {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.suffix {
      ::pb_jelly::wire_format::write(4, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
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
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Version", 1)?;
          self.major = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Version", 2)?;
          self.minor = Some(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, i32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "Version", 3)?;
          self.patch = Some(val);
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "Version", 4)?;
          self.suffix = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Version {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "major" => {
        ::pb_jelly::reflection::FieldMut::Value(self.major.get_or_insert_with(::std::default::Default::default))
      }
      "minor" => {
        ::pb_jelly::reflection::FieldMut::Value(self.minor.get_or_insert_with(::std::default::Default::default))
      }
      "patch" => {
        ::pb_jelly::reflection::FieldMut::Value(self.patch.get_or_insert_with(::std::default::Default::default))
      }
      "suffix" => {
        ::pb_jelly::reflection::FieldMut::Value(self.suffix.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// An encoded CodeGeneratorRequest is written to the plugin's stdin.
#[derive(Clone, Debug, PartialEq)]
pub struct CodeGeneratorRequest {
  /// The .proto files that were explicitly listed on the command-line.  The
  /// code generator should generate code only for these files.  Each file's
  /// descriptor will be included in proto_file, below.
  pub file_to_generate: ::std::vec::Vec<::std::string::String>,
  /// The generator parameter passed on the command-line.
  pub parameter: ::std::option::Option<::std::string::String>,
  /// FileDescriptorProtos for all files in files_to_generate and everything
  /// they import.  The files will appear in topological order, so each file
  /// appears before any file that imports it.

  /// Note: the files listed in files_to_generate will include runtime-retention
  /// options only, but all other files will include source-retention options.
  /// The source_file_descriptors field below is available in case you need
  /// source-retention options for files_to_generate.

  /// protoc guarantees that all proto_files will be written after
  /// the fields above, even though this is not technically guaranteed by the
  /// protobuf wire format.  This theoretically could allow a plugin to stream
  /// in the FileDescriptorProtos and handle them one by one rather than read
  /// the entire set into memory at once.  However, as of this writing, this
  /// is not similarly optimized on protoc's end -- it will store all fields in
  /// memory at once before sending them to the plugin.

  /// Type names of fields and extensions in the FileDescriptorProto are always
  /// fully qualified.
  pub proto_file: ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto>,
  /// File descriptors with all options, including source-retention options.
  /// These descriptors are only provided for the files listed in
  /// files_to_generate.
  pub source_file_descriptors: ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto>,
  /// The version number of protocol compiler.
  pub compiler_version: ::std::option::Option<Version>,
}
impl CodeGeneratorRequest {
  pub fn set_file_to_generate(&mut self, v: ::std::vec::Vec<::std::string::String>) {
    self.file_to_generate = v;
  }
  pub fn take_file_to_generate(&mut self) -> ::std::vec::Vec<::std::string::String> {
    ::std::mem::take(&mut self.file_to_generate)
  }
  pub fn get_file_to_generate(&self) -> &[::std::string::String] {
    &self.file_to_generate
  }
  pub fn mut_file_to_generate(&mut self) -> &mut ::std::vec::Vec<::std::string::String> {
    &mut self.file_to_generate
  }
  pub fn has_parameter(&self) -> bool {
    self.parameter.is_some()
  }
  pub fn set_parameter(&mut self, v: ::std::string::String) {
    self.parameter = Some(v);
  }
  pub fn take_parameter(&mut self) -> ::std::string::String {
    self.parameter.take().unwrap_or_default()
  }
  pub fn get_parameter(&self) -> &str {
    self.parameter.as_deref().unwrap_or("")
  }
  pub fn set_proto_file(&mut self, v: ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto>) {
    self.proto_file = v;
  }
  pub fn take_proto_file(&mut self) -> ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto> {
    ::std::mem::take(&mut self.proto_file)
  }
  pub fn get_proto_file(&self) -> &[super::super::super::protobuf::descriptor::FileDescriptorProto] {
    &self.proto_file
  }
  pub fn mut_proto_file(&mut self) -> &mut ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto> {
    &mut self.proto_file
  }
  pub fn set_source_file_descriptors(&mut self, v: ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto>) {
    self.source_file_descriptors = v;
  }
  pub fn take_source_file_descriptors(&mut self) -> ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto> {
    ::std::mem::take(&mut self.source_file_descriptors)
  }
  pub fn get_source_file_descriptors(&self) -> &[super::super::super::protobuf::descriptor::FileDescriptorProto] {
    &self.source_file_descriptors
  }
  pub fn mut_source_file_descriptors(&mut self) -> &mut ::std::vec::Vec<super::super::super::protobuf::descriptor::FileDescriptorProto> {
    &mut self.source_file_descriptors
  }
  pub fn has_compiler_version(&self) -> bool {
    self.compiler_version.is_some()
  }
  pub fn set_compiler_version(&mut self, v: Version) {
    self.compiler_version = Some(v);
  }
  pub fn take_compiler_version(&mut self) -> Version {
    self.compiler_version.take().unwrap_or_default()
  }
  pub fn get_compiler_version(&self) -> &Version {
    self.compiler_version.as_ref().unwrap_or(&Version_default)
  }
}
impl ::std::default::Default for CodeGeneratorRequest {
  fn default() -> Self {
    CodeGeneratorRequest {
      file_to_generate: ::std::default::Default::default(),
      parameter: ::std::default::Default::default(),
      proto_file: ::std::default::Default::default(),
      source_file_descriptors: ::std::default::Default::default(),
      compiler_version: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref CodeGeneratorRequest_default: CodeGeneratorRequest = CodeGeneratorRequest::default();
}
impl ::pb_jelly::Message for CodeGeneratorRequest {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "CodeGeneratorRequest",
      full_name: "google.protobuf.compiler.CodeGeneratorRequest",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "file_to_generate",
          full_name: "google.protobuf.compiler.CodeGeneratorRequest.file_to_generate",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "parameter",
          full_name: "google.protobuf.compiler.CodeGeneratorRequest.parameter",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "proto_file",
          full_name: "google.protobuf.compiler.CodeGeneratorRequest.proto_file",
          index: 2,
          number: 15,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "source_file_descriptors",
          full_name: "google.protobuf.compiler.CodeGeneratorRequest.source_file_descriptors",
          index: 3,
          number: 17,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "compiler_version",
          full_name: "google.protobuf.compiler.CodeGeneratorRequest.compiler_version",
          index: 4,
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
    let mut file_to_generate_size = 0;
    for val in &self.file_to_generate {
      let l = ::pb_jelly::Message::compute_size(val);
      file_to_generate_size += ::pb_jelly::wire_format::serialized_length(1);
      file_to_generate_size += ::pb_jelly::varint::serialized_length(l as u64);
      file_to_generate_size += l;
    }
    size += file_to_generate_size;
    let mut parameter_size = 0;
    if let Some(ref val) = self.parameter {
      let l = ::pb_jelly::Message::compute_size(val);
      parameter_size += ::pb_jelly::wire_format::serialized_length(2);
      parameter_size += ::pb_jelly::varint::serialized_length(l as u64);
      parameter_size += l;
    }
    size += parameter_size;
    let mut proto_file_size = 0;
    for val in &self.proto_file {
      let l = ::pb_jelly::Message::compute_size(val);
      proto_file_size += ::pb_jelly::wire_format::serialized_length(15);
      proto_file_size += ::pb_jelly::varint::serialized_length(l as u64);
      proto_file_size += l;
    }
    size += proto_file_size;
    let mut source_file_descriptors_size = 0;
    for val in &self.source_file_descriptors {
      let l = ::pb_jelly::Message::compute_size(val);
      source_file_descriptors_size += ::pb_jelly::wire_format::serialized_length(17);
      source_file_descriptors_size += ::pb_jelly::varint::serialized_length(l as u64);
      source_file_descriptors_size += l;
    }
    size += source_file_descriptors_size;
    let mut compiler_version_size = 0;
    if let Some(ref val) = self.compiler_version {
      let l = ::pb_jelly::Message::compute_size(val);
      compiler_version_size += ::pb_jelly::wire_format::serialized_length(3);
      compiler_version_size += ::pb_jelly::varint::serialized_length(l as u64);
      compiler_version_size += l;
    }
    size += compiler_version_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    for val in &self.file_to_generate {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.parameter {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.compiler_version {
      ::pb_jelly::wire_format::write(3, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.proto_file {
      ::pb_jelly::wire_format::write(15, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.source_file_descriptors {
      ::pb_jelly::wire_format::write(17, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
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
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "CodeGeneratorRequest", 1)?;
          self.file_to_generate.push(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "CodeGeneratorRequest", 2)?;
          self.parameter = Some(val);
        }
        15 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, super::super::super::protobuf::descriptor::FileDescriptorProto>(buf, typ, "CodeGeneratorRequest", 15)?;
          self.proto_file.push(val);
        }
        17 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, super::super::super::protobuf::descriptor::FileDescriptorProto>(buf, typ, "CodeGeneratorRequest", 17)?;
          self.source_file_descriptors.push(val);
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, Version>(buf, typ, "CodeGeneratorRequest", 3)?;
          self.compiler_version = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for CodeGeneratorRequest {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "file_to_generate" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "parameter" => {
        ::pb_jelly::reflection::FieldMut::Value(self.parameter.get_or_insert_with(::std::default::Default::default))
      }
      "proto_file" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "source_file_descriptors" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      "compiler_version" => {
        ::pb_jelly::reflection::FieldMut::Value(self.compiler_version.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// The plugin writes an encoded CodeGeneratorResponse to stdout.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CodeGeneratorResponse {
  /// Error message.  If non-empty, code generation failed.  The plugin process
  /// should exit with status code zero even if it reports an error in this way.

  /// This should be used to indicate errors in .proto files which prevent the
  /// code generator from generating correct code.  Errors which indicate a
  /// problem in protoc itself -- such as the input CodeGeneratorRequest being
  /// unparseable -- should be reported by writing a message to stderr and
  /// exiting with a non-zero status code.
  pub error: ::std::option::Option<::std::string::String>,
  /// A bitmask of supported features that the code generator supports.
  /// This is a bitwise "or" of values from the Feature enum.
  pub supported_features: ::std::option::Option<u64>,
  pub file: ::std::vec::Vec<CodeGeneratorResponse_File>,
}
impl CodeGeneratorResponse {
  pub fn has_error(&self) -> bool {
    self.error.is_some()
  }
  pub fn set_error(&mut self, v: ::std::string::String) {
    self.error = Some(v);
  }
  pub fn take_error(&mut self) -> ::std::string::String {
    self.error.take().unwrap_or_default()
  }
  pub fn get_error(&self) -> &str {
    self.error.as_deref().unwrap_or("")
  }
  pub fn has_supported_features(&self) -> bool {
    self.supported_features.is_some()
  }
  pub fn set_supported_features(&mut self, v: u64) {
    self.supported_features = Some(v);
  }
  pub fn get_supported_features(&self) -> u64 {
    self.supported_features.unwrap_or(0)
  }
  pub fn set_file(&mut self, v: ::std::vec::Vec<CodeGeneratorResponse_File>) {
    self.file = v;
  }
  pub fn take_file(&mut self) -> ::std::vec::Vec<CodeGeneratorResponse_File> {
    ::std::mem::take(&mut self.file)
  }
  pub fn get_file(&self) -> &[CodeGeneratorResponse_File] {
    &self.file
  }
  pub fn mut_file(&mut self) -> &mut ::std::vec::Vec<CodeGeneratorResponse_File> {
    &mut self.file
  }
}
impl ::std::default::Default for CodeGeneratorResponse {
  fn default() -> Self {
    CodeGeneratorResponse {
      error: ::std::default::Default::default(),
      supported_features: ::std::default::Default::default(),
      file: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref CodeGeneratorResponse_default: CodeGeneratorResponse = CodeGeneratorResponse::default();
}
impl ::pb_jelly::Message for CodeGeneratorResponse {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "CodeGeneratorResponse",
      full_name: "google.protobuf.compiler.CodeGeneratorResponse",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "error",
          full_name: "google.protobuf.compiler.CodeGeneratorResponse.error",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "supported_features",
          full_name: "google.protobuf.compiler.CodeGeneratorResponse.supported_features",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "file",
          full_name: "google.protobuf.compiler.CodeGeneratorResponse.file",
          index: 2,
          number: 15,
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
    let mut error_size = 0;
    if let Some(ref val) = self.error {
      let l = ::pb_jelly::Message::compute_size(val);
      error_size += ::pb_jelly::wire_format::serialized_length(1);
      error_size += ::pb_jelly::varint::serialized_length(l as u64);
      error_size += l;
    }
    size += error_size;
    let mut supported_features_size = 0;
    if let Some(ref val) = self.supported_features {
      let l = ::pb_jelly::Message::compute_size(val);
      supported_features_size += ::pb_jelly::wire_format::serialized_length(2);
      supported_features_size += l;
    }
    size += supported_features_size;
    let mut file_size = 0;
    for val in &self.file {
      let l = ::pb_jelly::Message::compute_size(val);
      file_size += ::pb_jelly::wire_format::serialized_length(15);
      file_size += ::pb_jelly::varint::serialized_length(l as u64);
      file_size += l;
    }
    size += file_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.error {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.supported_features {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::Varint, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    for val in &self.file {
      ::pb_jelly::wire_format::write(15, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
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
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "CodeGeneratorResponse", 1)?;
          self.error = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u64>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "CodeGeneratorResponse", 2)?;
          self.supported_features = Some(val);
        }
        15 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, CodeGeneratorResponse_File>(buf, typ, "CodeGeneratorResponse", 15)?;
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
impl ::pb_jelly::Reflection for CodeGeneratorResponse {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "error" => {
        ::pb_jelly::reflection::FieldMut::Value(self.error.get_or_insert_with(::std::default::Default::default))
      }
      "supported_features" => {
        ::pb_jelly::reflection::FieldMut::Value(self.supported_features.get_or_insert_with(::std::default::Default::default))
      }
      "file" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

/// Represents a single generated file.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CodeGeneratorResponse_File {
  /// The file name, relative to the output directory.  The name must not
  /// contain "." or ".." components and must be relative, not be absolute (so,
  /// the file cannot lie outside the output directory).  "/" must be used as
  /// the path separator, not "\".

  /// If the name is omitted, the content will be appended to the previous
  /// file.  This allows the generator to break large files into small chunks,
  /// and allows the generated text to be streamed back to protoc so that large
  /// files need not reside completely in memory at one time.  Note that as of
  /// this writing protoc does not optimize for this -- it will read the entire
  /// CodeGeneratorResponse before writing files to disk.
  pub name: ::std::option::Option<::std::string::String>,
  /// If non-empty, indicates that the named file should already exist, and the
  /// content here is to be inserted into that file at a defined insertion
  /// point.  This feature allows a code generator to extend the output
  /// produced by another code generator.  The original generator may provide
  /// insertion points by placing special annotations in the file that look
  /// like:
  ///   @@protoc_insertion_point(NAME)
  /// The annotation can have arbitrary text before and after it on the line,
  /// which allows it to be placed in a comment.  NAME should be replaced with
  /// an identifier naming the point -- this is what other generators will use
  /// as the insertion_point.  Code inserted at this point will be placed
  /// immediately above the line containing the insertion point (thus multiple
  /// insertions to the same point will come out in the order they were added).
  /// The double-@ is intended to make it unlikely that the generated code
  /// could contain things that look like insertion points by accident.

  /// For example, the C++ code generator places the following line in the
  /// .pb.h files that it generates:
  ///   // @@protoc_insertion_point(namespace_scope)
  /// This line appears within the scope of the file's package namespace, but
  /// outside of any particular class.  Another plugin can then specify the
  /// insertion_point "namespace_scope" to generate additional classes or
  /// other declarations that should be placed in this scope.

  /// Note that if the line containing the insertion point begins with
  /// whitespace, the same whitespace will be added to every line of the
  /// inserted text.  This is useful for languages like Python, where
  /// indentation matters.  In these languages, the insertion point comment
  /// should be indented the same amount as any inserted code will need to be
  /// in order to work correctly in that context.

  /// The code generator that generates the initial file and the one which
  /// inserts into it must both run as part of a single invocation of protoc.
  /// Code generators are executed in the order in which they appear on the
  /// command line.

  /// If |insertion_point| is present, |name| must also be present.
  pub insertion_point: ::std::option::Option<::std::string::String>,
  /// The file contents.
  pub content: ::std::option::Option<::std::string::String>,
  /// Information describing the file content being inserted. If an insertion
  /// point is used, this information will be appropriately offset and inserted
  /// into the code generation metadata for the generated files.
  pub generated_code_info: ::std::option::Option<super::super::super::protobuf::descriptor::GeneratedCodeInfo>,
}
impl CodeGeneratorResponse_File {
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
  pub fn has_insertion_point(&self) -> bool {
    self.insertion_point.is_some()
  }
  pub fn set_insertion_point(&mut self, v: ::std::string::String) {
    self.insertion_point = Some(v);
  }
  pub fn take_insertion_point(&mut self) -> ::std::string::String {
    self.insertion_point.take().unwrap_or_default()
  }
  pub fn get_insertion_point(&self) -> &str {
    self.insertion_point.as_deref().unwrap_or("")
  }
  pub fn has_content(&self) -> bool {
    self.content.is_some()
  }
  pub fn set_content(&mut self, v: ::std::string::String) {
    self.content = Some(v);
  }
  pub fn take_content(&mut self) -> ::std::string::String {
    self.content.take().unwrap_or_default()
  }
  pub fn get_content(&self) -> &str {
    self.content.as_deref().unwrap_or("")
  }
  pub fn has_generated_code_info(&self) -> bool {
    self.generated_code_info.is_some()
  }
  pub fn set_generated_code_info(&mut self, v: super::super::super::protobuf::descriptor::GeneratedCodeInfo) {
    self.generated_code_info = Some(v);
  }
  pub fn take_generated_code_info(&mut self) -> super::super::super::protobuf::descriptor::GeneratedCodeInfo {
    self.generated_code_info.take().unwrap_or_default()
  }
  pub fn get_generated_code_info(&self) -> &super::super::super::protobuf::descriptor::GeneratedCodeInfo {
    self.generated_code_info.as_ref().unwrap_or(&super::super::super::protobuf::descriptor::GeneratedCodeInfo_default)
  }
}
impl ::std::default::Default for CodeGeneratorResponse_File {
  fn default() -> Self {
    CodeGeneratorResponse_File {
      name: ::std::default::Default::default(),
      insertion_point: ::std::default::Default::default(),
      content: ::std::default::Default::default(),
      generated_code_info: ::std::default::Default::default(),
    }
  }
}
lazy_static! {
  pub static ref CodeGeneratorResponse_File_default: CodeGeneratorResponse_File = CodeGeneratorResponse_File::default();
}
impl ::pb_jelly::Message for CodeGeneratorResponse_File {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "CodeGeneratorResponse_File",
      full_name: "google.protobuf.compiler.CodeGeneratorResponse_File",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "name",
          full_name: "google.protobuf.compiler.CodeGeneratorResponse_File.name",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "insertion_point",
          full_name: "google.protobuf.compiler.CodeGeneratorResponse_File.insertion_point",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "content",
          full_name: "google.protobuf.compiler.CodeGeneratorResponse_File.content",
          index: 2,
          number: 15,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "generated_code_info",
          full_name: "google.protobuf.compiler.CodeGeneratorResponse_File.generated_code_info",
          index: 3,
          number: 16,
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
    let mut insertion_point_size = 0;
    if let Some(ref val) = self.insertion_point {
      let l = ::pb_jelly::Message::compute_size(val);
      insertion_point_size += ::pb_jelly::wire_format::serialized_length(2);
      insertion_point_size += ::pb_jelly::varint::serialized_length(l as u64);
      insertion_point_size += l;
    }
    size += insertion_point_size;
    let mut content_size = 0;
    if let Some(ref val) = self.content {
      let l = ::pb_jelly::Message::compute_size(val);
      content_size += ::pb_jelly::wire_format::serialized_length(15);
      content_size += ::pb_jelly::varint::serialized_length(l as u64);
      content_size += l;
    }
    size += content_size;
    let mut generated_code_info_size = 0;
    if let Some(ref val) = self.generated_code_info {
      let l = ::pb_jelly::Message::compute_size(val);
      generated_code_info_size += ::pb_jelly::wire_format::serialized_length(16);
      generated_code_info_size += ::pb_jelly::varint::serialized_length(l as u64);
      generated_code_info_size += l;
    }
    size += generated_code_info_size;
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    if let Some(ref val) = self.name {
      ::pb_jelly::wire_format::write(1, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.insertion_point {
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.content {
      ::pb_jelly::wire_format::write(15, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      let l = ::pb_jelly::Message::compute_size(val);
      ::pb_jelly::varint::write(l as u64, w)?;
      ::pb_jelly::Message::serialize(val, w)?;
    }
    if let Some(ref val) = self.generated_code_info {
      ::pb_jelly::wire_format::write(16, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
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
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "CodeGeneratorResponse_File", 1)?;
          self.name = Some(val);
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "CodeGeneratorResponse_File", 2)?;
          self.insertion_point = Some(val);
        }
        15 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "CodeGeneratorResponse_File", 15)?;
          self.content = Some(val);
        }
        16 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, super::super::super::protobuf::descriptor::GeneratedCodeInfo>(buf, typ, "CodeGeneratorResponse_File", 16)?;
          self.generated_code_info = Some(val);
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for CodeGeneratorResponse_File {
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
      "insertion_point" => {
        ::pb_jelly::reflection::FieldMut::Value(self.insertion_point.get_or_insert_with(::std::default::Default::default))
      }
      "content" => {
        ::pb_jelly::reflection::FieldMut::Value(self.content.get_or_insert_with(::std::default::Default::default))
      }
      "generated_code_info" => {
        ::pb_jelly::reflection::FieldMut::Value(self.generated_code_info.get_or_insert_with(::std::default::Default::default))
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

