extern crate basex_rs;
use basex_rs::BaseX;

fn main() {
    let src = "jkuzA".to_string();
    let decoded = BaseX::decode(src);
    assert_eq!(decoded, Some(vec![0, 130, 189, 40]));
}
