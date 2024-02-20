# Why does this module exist?

When trying to include generated code from [`protobuf-codegen`](https://github.com/stepancheg/rust-protobuf/tree/master/protobuf-codegen) in the same fashion as `PROST!`, e.g.
```
mod gen {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/gen/prost/pbtest.rs"));
}
```
you get a compiler error:
```
error: an inner attribute is not permitted following an outer attribute
```
According to issue [`#117`](https://github.com/stepancheg/rust-protobuf/issues/117) on `rust-protobuf`, the work around for this include issue is to bundle your code into a module like so. You don't need to be as deeply nested as this example is, but we found this organization to make sense.
