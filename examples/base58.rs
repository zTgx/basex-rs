extern crate basex_rs;
use basex_rs::{BaseX, Decode, BITCOIN, SKYWELL, Encode};

fn main() {
    let src = "jkuzA".to_string();
    let decoded = BaseX::new(BITCOIN).decode(src);
    assert_eq!(decoded, Some(vec![28, 215, 33, 155]));

    let src = vec![28, 215, 33, 155];
    let encoded = BaseX::new(BITCOIN).encode(&src);
    assert_eq!(encoded, "jkuzA".to_string());

    let src = "jkuzA".to_string();
    let decoded = BaseX::new(SKYWELL).decode(src);
    assert_eq!(decoded, Some(vec![0, 130, 189, 40]));

    let src = vec![0, 130, 189, 40];
    let encoded = BaseX::new(SKYWELL).encode(&src);
    assert_eq!(encoded, "jkuzA".to_string());
}
