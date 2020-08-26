# examples

### basic
How to use `protobuf-rs` in the most basic sense. Creating, serializing, and de-serializing a message.

### custom_type
Defining custom Rust types for a field.

### linked_list
Self referential messages automatically [`Box`]("https://doc.rust-lang.org/std/boxed/struct.Box.html) the recursive type.

### multi
How directory structure of protos effects the module structure of the generated Rust crate.

### non_optional
Making non-optional `enum` fields, and `oneof`s.

### serde
All messages defined in a given `.proto` file derive [`Serialize`]("https://docs.rs/serde/1.0.114/serde/trait.Serialize.html") and [`Deserialize`]("https://docs.rs/serde/1.0.114/serde/trait.Deserialize.html").

### zero_copy
Fields of type `bytes` in proto messages map to a Rust type of `pb::Lazy<bytes::Bytes>` to enable zero-copy deserialization
