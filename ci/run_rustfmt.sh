#!/bin/bash -ex

cargo --version
cargo fmt --version
for d in examples pb-jelly pb-jelly-gen pb-test; do
  pushd $d
  cargo fmt -- --check
  popd
done
