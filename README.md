# embed_staticfile.rs

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
include_dir_bytes = "0.1"
embed_staticfile = "0.1"
```

and use them like this:

```rust
#![feature(plugin)]
#![plugin(include_dir_bytes)]

extern crate iron;
extern crate mount;

extern crate embed_staticfile;

use iron::prelude::*;
use mount::Mount;
use embed_staticfile::EmbedStatic;

fn main() {
    let mut mount = Mount::new();
    mount.mount("/assets/", EmbedStatic::new(include_dir!("../assets")));

    Iron::new(mount).http("localhost:3000").unwrap();
}
```
