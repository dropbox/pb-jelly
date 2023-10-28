# `pb-jelly`
###### With great power, comes great responsibility - Peter Parker

[![Crates.io](https://img.shields.io/crates/v/pb-jelly)](https://crates.io/crates/pb-jelly) [![Documentation](https://docs.rs/pb-jelly/badge.svg)](https://docs.rs/pb-jelly) [![Crates.io](https://img.shields.io/crates/l/pb-jelly)](LICENSE)

This crates provides the necessary trait implementations to power code generated with [`pb-jelly-gen`](https://github.com/dropbox/pb-jelly/tree/main/pb-gen). You should
include this crate as a dependency in your `Cargo.toml`.


##### `Cargo.toml`
```
[dependencies]
pb-jelly = "0.0.14"
```

Then in the general case, all you'll need to use in your code is the `Message` trait this crate defines, e.g.
```
use pb_jelly::Message;
```

More complete examples can be found in the [`examples`](https://github.com/dropbox/pb-jelly/tree/main/examples) crate, or
the [`pb-test`](https://github.com/dropbox/pb-jelly/tree/main/pb-test) crate itself.
