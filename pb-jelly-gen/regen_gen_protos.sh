#!/usr/bin/env bash
cargo build && \
protoc --plugin=protoc-gen-jellyrust=target/debug/protoc-gen-rust -Iproto rust/extensions.proto google/protobuf/{compiler/plugin,descriptor}.proto --jellyrust_out=single_file:src
