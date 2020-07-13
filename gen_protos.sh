protoc --plugin=protoc-gen-rust=codegen/codegen.py --proto_path=proto/ --rust_out=generated/ `find proto -name "*.proto"`
