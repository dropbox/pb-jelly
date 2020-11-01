# Upcoming
* Add windows support to pb_jelly_gen
* Add windows support to our CI

# 0.0.4
### October 21, 2020
* Drop python2 support (remove six and inline type annotations)
* Only generate crate level attributes in lib.rs (vs every module)
* Better error message if python-protobuf version is too low for codegen plugin
* Bump protobuf in requirements to 3.13.0
    * Requires upgrading to protobuf 3.13.0

#### Bugs
* Fixed issue where sometimes in codegen field type was incorrect when using `err_if_default_or_unknown`

# 0.0.3
### September 21, 2020
* Forgot to bump the version of `pb-jelly` that the codegen script uses

# v0.0.2
### September 19, 2020
* Use the `license` field instead of the `license-file` field in the Cargo.toml of `pb-jelly` and `pb-jelly-gen`.
    * Note: The License is still the same, the update is purely for better metadata from [crates.io](https://crates.io/crates/pb-jelly)
* Warn on `rust_2018_idioms` closes [#45](https://github.com/dropbox/pb-jelly/issues/45)
* A few changes related solely to re-integrating `pb-jelly` into the Dropbox codebase.
