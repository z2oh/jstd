# jstd

I've been copying around a `utils.rs` file between small Rust projects for a while now, so I figured it's time to formalize it so I can more easily transfer improvements between projects. This is only intended for bootstrapping new projects; mature projects should explicitly specify the dependencies they use. Custom bootstrapping code is exported as `macros` to ensure syntactic referential transparency.

Most of my projects resemble a CLI tool, and I typically start with the following `main.rs`:

```
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use jstd::prelude::*;

// cargo add serde --features derive
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {}

#[tokio::main]
async fn main() -> Result<()> {
    init_log!();

    let args = Args::parse();

    Ok(())
}
```
I couldn't find a way to re-export the `Deserialize`/`Serialize` proc macros through this crate, so `serde` must still be explicitly depended on in consumers.

### `init_log!`
With no parameters, initializes [`colog`](https://docs.rs/colog/latest/colog/) at the default [`LogLevel::Info`](https://docs.rs/log/latest/log/enum.Level.html) for release builds, and [`LogLevel::Trace`](https://docs.rs/log/latest/log/enum.Level.html) for debug builds (by proxy via `cfg!(debug_assertions)`). An optional `$filter_module:expr` string expression can be passed, which will forward the argument to [`env_logger::Builder::filter_module`](https://docs.rs/env_logger/latest/env_logger/struct.Builder.html#method.filter_module); this is useful if any additional dependencies left in logs at the [`Trace`](https://docs.rs/log/latest/log/enum.Level.html) level.

### `benchmark!`

Example usage:
```
let n = 1000000;
let output_vec = benchmark!(format!("Generating {} random numbers took:", n), {
    let mut rng = rand::rng();
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        v.push(rng.random::<f32>());
    }
    v
});
```
Outputs:
```
[*] Generating 1000000 random numbers took: 174.8337ms
```

Benchmarking is disabled for release builds (by proxy via `cfg!(debug_assertions)`), and the first parameter, `$msg:expr`, is lazily evaluated in a closure to avoid eager evaluation when benchmarking is disabled.
