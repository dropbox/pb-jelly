# Unreleased
* Everything has been released!

# 0.0.9
### October 29, 2021
* Simplify Option<String> -> &str conversion in pb-jelly-gen (clippy warning)

# 0.0.8
### October 25, 2021
* Move `(gogoproto.nullable)` option to `(rust.nullable_field)`, removing the dependency on gogoproto
* Support running with any version of protoc (by dynamically generating `extensions_pb2.py` in a venv)
* Use github CI for tests, rustfmt, black, mypy --strict
* Support windows for codegen
* Rename master branch to main
* Use a setup.cfg to install protoc-gen-rust to avoid need for --plugin flag. Simplifies manual usage process, especially on windows (no need for codegen.bat)
* Remove need for requirements.txt - by using setup.cfg `install_requires`

# 0.0.7
### May 5, 2021
* Add `(rust.closed_enum)` option to only generate closed enums

# 0.0.6
### April 25, 2021
* Bump `bytes` to `1.0`
* Bump `byteorder` to `1.4`

# 0.0.5
### November 20, 2020
* Add Windows support to `pb-jelly-gen`
* Add Windows support to our CI
* Allows proto files at the root of proto path

#### Bugs
* Fixed a bug around non-optional boxed messages

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
