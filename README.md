# [basex-rs](https://github.com/zTgx/basex-rs.git) [![crate](https://img.shields.io/crates/v/basex-rs.svg)](https://crates.io/crates/basex-rs)

# Usage
Add dependencies
```
[dependencies]
basex-rs = "0.2.0"
```

```rust
use basex_rs::{BaseX, ALPHABET_BITCOIN};

let src = vec![28, 215, 33, 155];
let encoded = BaseX::with_alphabet(ALPHABET_BITCOIN).to_bs58(&src);
assert_eq!(encoded, "jkuzA".to_string());
```