# Run this to regenerate the checked-in `extensions_pb2.py`.
# The installed version of `protoc` must be compatible with the pinned version of `protobuf` in `requirements.txt`
protoc -I . --python_out proto rust/extensions.proto
