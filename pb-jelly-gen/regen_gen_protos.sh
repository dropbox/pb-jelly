#!/usr/bin/env bash
cargo build --no-default-features && \
protoc --plugin=protoc-gen-jellyrust=target/debug/protoc-gen-jellyrust -Iproto rust/extensions.proto google/protobuf/{compiler/plugin,descriptor}.proto --jellyrust_out=single_file:src
