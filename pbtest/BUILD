package(default_visibility = ["//visibility:public"])

load("//build_tools/rust:rust.bzl", "rust_bench_test", "rust_library", "rust_test")

rust_library(
    name = "pbtest",
    testonly = True,
    srcs = glob(
        ["src/**/*.rs"],
        exclude = ["src/bin/**"],
    ),
    clippy = True,
    crate_type = "lib",
    deps = [
        "//rust/dropbox/blob",
        "//rust/dropbox/blob/blob_pb",
        "//rust/dropbox/proto/pbtest/proto_pbtest",
        "//rust/dropbox/runfiles",
        "//rust/pallet/context",
        "//rust/pallet/pb",
        "//rust/vendor/pretty_assertions",
    ],
)

rust_test(
    name = "pbtest_test",
    deps = [
        ":pbtest",
    ],
)

rust_test(
    name = "lib_test",
    srcs = ["tests/lib.rs"],
    data = [
        "//go/src/dropbox/pbtest",
    ],
    deps = [
        ":pbtest",
        "//rust/dropbox/proto/pbtest/proto_pbtest",
        "//rust/dropbox/runfiles",
        "//rust/pallet/pb",
        "//rust/vendor/pretty_assertions",
    ],
)

rust_bench_test(
    name = "proto_bench",
    srcs = ["benches/lib.rs"],
    deps = [
        ":pbtest",
        "//rust/dropbox/proto/mp/proto_mp",
    ],
)
