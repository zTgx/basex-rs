// pub const ALPHABET: &[u8; 62] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const BITCOIN:  &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub const RIPPLE:   &[u8; 58] = b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";
pub const FLICKR:   &[u8; 58] = b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
pub const SKYWELL:  &[u8; 58] = b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";

mod traits;
pub use traits::{Encode, Decode};
use traits::generate_alpha_map;

pub struct BaseX <'a> {
    pub alphabet: &'a [u8],
}

impl <'a> BaseX <'a> {
    pub fn new(alphabet: &'a [u8]) -> Self {
        BaseX {
            alphabet: alphabet,
        }
    }
}

impl <'a> Encode for BaseX <'a>{
    type Output = String;
    fn encode(&self, source: &[u8]) -> String {

        let base = self.alphabet.len() as u16;

        let mut digits: Vec<u16> = vec![0u16; 1];

        let mut i = 0;
        while i < source.len() {

            let mut j = 0;
            let mut carry: u16 = source[i] as u16;

            let digits_len = digits.len();
            while j < digits_len {
                carry += digits.as_slice()[j] << 8;

                digits.as_mut_slice()[j] = carry % (base as u16);

                carry = (carry / (base as u16)) | 0;

                j += 1;
            }

            while carry > 0 {
                digits.push(carry % (base as u16));
                carry = (carry / base) | 0;
            }

            i += 1;
        }

        let mut string = "".to_string();

        // deal with leading zeros
        let mut k = 0;
        while source[k] == 0 && k < source.len() - 1 {

            string.push(self.alphabet[0] as char);

            k += 1;
        }

        // convert digits to a string
        let mut q: i32 = (digits.len() - 1) as i32;
        while q >= 0 {

            let uu: u8 = self.alphabet[digits[q as usize] as usize];
            let xx = uu as char;

            string.push( xx );

            q -= 1;
        }

        string
    }
}

impl <'a> Decode for BaseX <'a> {
    type Output = Option<Vec<u8>>;
    fn decode(&self, string: String) -> Option<Vec<u8>> {
        if string.len() == 0 { return None; }

        let alphabet_map = generate_alpha_map(self.alphabet);
        let base = self.alphabet.len() as u16;
        let ledger = self.alphabet[0] as char;

        let mut bytes: Vec<u8> = vec![];
        let mut i = 0;
        while i < string.len() {
            let c = string.as_str().chars().nth(i).unwrap();
            let val = alphabet_map.get(&c);
            if val.is_none() {
                return None;
            }

            let mut j = 0;
            let mut carry: u16 = *val.unwrap() as u16;
            while j < bytes.len() {
                carry += bytes[j] as u16 * base;
                bytes[j] = (carry as u8) & 0xff;
                carry >>= 8;

                j += 1;
            }

            while carry > 0 {
                bytes.push((carry as u8) & 0xff );
                carry >>= 8;
            }

            i += 1;
        }

        // deal with leading zeros
        let mut k = 0;
        while string.as_str().chars().nth(k).unwrap() == ledger && k < string.len() - 1 {
            bytes.push(0);
            k += 1;
        }

        bytes.as_mut_slice().reverse();

        Some(bytes)
    }
}

#[cfg(test)]
mod test_set {
    use super::*;

    #[test]
    fn encode_test() {
        let src = vec![28, 215, 33, 155];
        let encoded = BaseX::new(BITCOIN).encode(&src);
        assert_eq!(encoded, "jkuzA".to_string());
    }

    #[test]
    fn decode_test() {
        let src = "jkuzA".to_string();
        let decoded = BaseX::new(BITCOIN).decode(src);
        assert_eq!(decoded, Some(vec![28, 215, 33, 155]));
    }
}
