#!/bin/bash -ex

cd pb-jelly-gen/codegen

python3 -m venv .black_venv
source .black_venv/bin/activate

pip3 install black
black --check codegen.py
