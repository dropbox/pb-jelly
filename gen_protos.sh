# Install protoc
# - https://github.com/protocolbuffers/protobuf
# Install gogoproto
# - go get github.com/gogo/protobuf/proto
protoc --plugin=protoc-gen-rust=codegen/codegen.py --proto_path=proto/ --proto_path=$GOPATH/src --rust_out=generated/ `find proto -name "*.proto"`
