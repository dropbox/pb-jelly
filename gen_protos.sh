set -ex

# Install protoc
# - https://github.com/protocolbuffers/protobuf
# Install gogoproto
# - go get github.com/gogo/protobuf/proto

rm -rf generated/  # Start clean
mkdir -p generated/python/proto
mkdir -p generated/rust/proto

PROTOC_ARGS="--proto_path=$GOPATH/src --proto_path=proto/"

# Generate python protos - since codegen uses them
GOGO=$GOPATH/src/github.com/gogo/protobuf/gogoproto/gogo.proto
export PYTHONPATH="generated/python/"
protoc $PROTOC_ARGS --python_out=generated/python/proto proto/rust/extensions.proto $GOGO

# Add __init__.py files
find generated/python -type d -exec touch {}/__init__.py \;


# Rust codegen the protos!
protoc $PROTOC_ARGS --plugin=protoc-gen-rust=codegen/codegen.py --rust_out=generated/rust/proto `find proto -name "*.proto"`
