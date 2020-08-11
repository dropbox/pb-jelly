# examples

### basic
How to use `protobuf-rs` in the most basic sense. Creating, serializing, and de-serializing a message.

### linked_list
Self referential messages automatically [`Box`]("https://doc.rust-lang.org/std/boxed/struct.Box.html) the recursive type.

### serde
All messages defined in a given `.proto` file derive [`Serialize`]("https://docs.rs/serde/1.0.114/serde/trait.Serialize.html") and [`Deserialize`]("https://docs.rs/serde/1.0.114/serde/trait.Deserialize.html").
