# `pb-rs`
###### With great power, comes great responsibility - Peter Parker

This crates provides the necessary trait implementations to power code generated with [`pb-gen`](https://github.com/dropbox/pb-rs/tree/master/pb-gen). You should
include this crate as a dependency in your `Cargo.toml`.


##### `Cargo.toml`
```
[dependencies]
pb-rs = "0.1"
```

Then in the general case, all you'll need to use in your code is the `Message` trait this crate defines, e.g.
```
use pb::Message;
```

More complete examples can be found in the [`examples`](https://github.com/dropbox/pb-rs/tree/master/examples) crate, or 
the [`pb-test`](https://github.com/dropbox/pb-rs/tree/master/pb-test) crate itself.
