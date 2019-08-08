# [basex-rs](https://github.com/zTgx/basex-rs.git)  [![Build Status](https://travis-ci.org/zTgx/basex-rs.svg?branch=master)](https://travis-ci.org/zTgx/basex-rs) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Base58 implement in [Rust](http://www.rust-lang.org).

# Usage
Add dependencies
```
[dependencies]
basex-rs = "0.1.0"
```

```rust
//decode
extern crate basex_rs;
use basex_rs::BaseX;

let src = "jkuzA".to_string();
let decoded = BaseX::decode(src);
assert_eq!(decoded, Some(vec![0, 130, 189, 40]));
```

```rust
//encode
extern crate basex_rs;
use basex_rs::BaseX;

let src = vec![0, 130, 189, 40];
let encoded = BaseX::encode(&src);
assert_eq!(encoded, "jkuzA".to_string());
```
