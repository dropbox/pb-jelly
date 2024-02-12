cargo build && protoc --plugin=protoc-gen-jellyrust=target/debug/protoc-gen-rust -Icodegen rust/extensions.proto google/protobuf/{compiler/plugin,descriptor}.proto --jellyrust_out=.
