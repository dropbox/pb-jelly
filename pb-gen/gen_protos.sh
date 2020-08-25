#!/bin/sh

set -ex

rm -rf generated/  # Start clean
mkdir -p generated/python/proto
mkdir -p generated/rust/proto

# If GOPATH is not set use the default
if [[ -z $GOPATH ]]; then
	GOPATH=$HOME/go
fi

PROTOC_ARGS="--proto_path=$GOPATH/src --proto_path=codegen/"

# Generate python protos - since codegen uses them
GOGO=$GOPATH/src/github.com/gogo/protobuf/gogoproto/gogo.proto
export PYTHONPATH="generated/python/"
protoc $PROTOC_ARGS --python_out=generated/python/proto codegen/rust/extensions.proto $GOGO

# Add __init__.py files
find generated/python -type d -exec touch {}/__init__.py \;

# May have to do brew install coreutils to get realpath
# Determine protobuf path
PROTOC_PATH=$(readlink -f $(which protoc) || realpath $(which protoc))

# Rust codegen the protos!
OURS=$(find proto -name "*.proto")
GOOGLE=$(find "${PROTOC_PATH%/bin/protoc}/include/google/protobuf" -name "*.proto")
protoc $PROTOC_ARGS --plugin=protoc-gen-rust=codegen/codegen.py --rust_out=generated/rust/proto $OURS $GOOGLE
