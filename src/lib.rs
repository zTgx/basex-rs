

use std::collections::HashMap;

static ALPHABET: &[u8; 58] = b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";

pub struct BaseX {}
impl BaseX {
    pub fn encode(source: &[u8]) -> String {

        let base = ALPHABET.len() as u16;

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

            string.push(ALPHABET[0] as char);

            k += 1;
        }

        // convert digits to a string
        let mut q: i32 = (digits.len() - 1) as i32;
        while q >= 0 {

            let uu: u8 = ALPHABET[digits[q as usize] as usize];
            let xx = uu as char;

            string.push( xx );

            q -= 1;
        }

        string
    }

    pub fn decode(string: String) -> Option<Vec<u8>> {
        if string.len() == 0 { return None; }

        let alphabet_map = BaseX::generate_alpha_map();
        let base = ALPHABET.len() as u16;
        let ledger = ALPHABET[0] as char;

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

    //default source ALPHABET.
    pub fn generate_alpha_map() -> HashMap<char, usize> {
        let mut map: HashMap<char, usize> = HashMap::new();
        let lens = ALPHABET.len();

        // pre-compute lookup table
        let mut i = 0;
        while i < lens {
            let x = ALPHABET[i] as char;
            map.insert(x, i);

            i += 1;
        }

        map
    }
}

#[cfg(test)]
mod test_set {
    use super::*;

    #[test]
    fn encode_test() {
        let src = vec![0, 130, 189, 40];
        let encoded = BaseX::encode(&src);
        assert_eq!(encoded, "jkuzA".to_string());
    }

    #[test]
    fn decode_test() {
        let src = "jkuzA".to_string();
        let decoded = BaseX::decode(src);
        assert_eq!(decoded, Some(vec![0, 130, 189, 40]));
    }
}
