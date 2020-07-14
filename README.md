# pb-rs

### TODO

- [x] Get onto github
- [x] Get lib compiling
- [x] Get tests compiling
- [ ] Remove references to dbx specific stuff
- [x] Add critical protos (eg rust.proto for extensions)
- [x] Move pbtest into tests/ directory per traditional rust testing design
- [x] Add pbtest protos
- [x] Get protos compiling somehow for tests
- [ ] Rename pbtest.proto to pbtest2.proto
- [ ] Add service generation codegen as well
- [ ] Add blob crate / zerocopy
- [ ] run benchmarks against zerocopy blob impl
- [ ] Make extensions.proto and servicepb.proto proto3
- [ ] Add travis-ci integration
- [ ] Mypy the codegen.py in CI
- [ ] Run mypy-protobuf for the tests - to better mypy codegen.py
- [ ] Add examples to README
- [ ] Create github issues for remaining todos
- [ ] Credit other Dropboxers that have contributed to pb-rs development (look at git log)
- [ ] Document features of this implementation
- [ ] Document why it exists (vs the standard open source proto options)
- [ ] Figure out how to host documentation somewhere
- [ ] Bonus: Port over the test which serializes in go and deserializes/reserializes in rust
- [ ] Bonus: Python3
- [ ] Create helper library similar to `prost_build`

## Contributors

### Dropboxers
- [@nipunn1313](https://github.com/nipunn1313)
