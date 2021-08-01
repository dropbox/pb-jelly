#!/bin/bash -ex

cd pb-jelly-gen/codegen

python3 -m venv .mypy_venv
source .mypy_venv/bin/activate

pip3 install mypy mypy-protobuf
protoc --mypy_out=proto rust/extensions.proto

mypy --strict .
