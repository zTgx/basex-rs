# [basex-rs](https://github.com/zTgx/basex-rs.git)  [![Build Status](https://travis-ci.org/zTgx/basex-rs.svg?branch=master)](https://travis-ci.org/zTgx/basex-rs) [![crate](https://img.shields.io/crates/v/basex-rs.svg)](https://crates.io/crates/basex-rs)

Base58 implement in [Rust](http://www.rust-lang.org).

# Usage
Add dependencies
```
[dependencies]
basex-rs = "0.1.1"
```

```rust
//decode
extern crate basex_rs;
use basex_rs::{BaseX, Decode, BITCOIN};

let src = "jkuzA".to_string();
let decoded = BaseX::new(BITCOIN).decode(src);
assert_eq!(decoded, Some(vec![28, 215, 33, 155]));
```

```rust
//encode
extern crate basex_rs;
use basex_rs::{BaseX, Encode, BITCOIN};

let src = vec![28, 215, 33, 155];
let encoded = BaseX::new(BITCOIN).encode(&src);
assert_eq!(encoded, "jkuzA".to_string());
```
