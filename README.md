# basex-rs

Base58 implement in [Rust](http://www.rust-lang.org).

# Usage
Add dependencies
```
[dependencies]
basex-rs = { git = "https://github.com/zTgx/basex-rs.git" }
```

```
//encode
extern crate basex_rs;
use basex_rs::BaseX;

let src = "jkuzA".to_string();
let decoded = BaseX::decode(src);
assert_eq!(decoded, Some(vec![0, 130, 189, 40]));
```

```
//decode
extern crate basex_rs;
use basex_rs::BaseX;

let src = vec![0, 130, 189, 40];
let encoded = BaseX::encode(&src);
assert_eq!(encoded, "jkuzA".to_string());
```
