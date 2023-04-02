use std::collections::HashMap;

const LEN: usize = 58;
pub const ALPHABET_BITCOIN: &[u8; LEN] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub const ALPHABET_RIPPLE: &[u8; LEN] =
    b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz";
pub const ALPHABET_FLICKR: &[u8; LEN] =
    b"123456789abcdefghijkmnopqrstuvwxyzABCDEFGHJKLMNPQRSTUVWXYZ";
pub const ALPHABET_SKYWELL: &[u8; LEN] =
    b"jpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65rkm8oFqi1tuvAxyz";

pub struct BaseX<'a> {
    pub alphabet: &'a [u8],
}

impl<'a> BaseX<'a> {
    pub fn with_alphabet(alphabet: &'a [u8]) -> Self {
        BaseX { alphabet }
    }

    pub fn to_bs58(&self, input: &[u8]) -> String {
        let base = self.alphabet.len() as u16;

        let mut digits: Vec<u16> = vec![0u16; 1];

        let mut i = 0;
        while i < input.len() {
            let mut j = 0;
            let mut carry: u16 = input[i] as u16;

            let digits_len = digits.len();
            while j < digits_len {
                carry += digits.as_slice()[j] << 8;

                digits.as_mut_slice()[j] = carry % base;

                carry /= base;

                j += 1;
            }

            while carry > 0 {
                digits.push(carry % base);
                carry /= base;
            }

            i += 1;
        }

        let mut output = "".to_string();

        // deal with leading zeros
        let mut k = 0;
        while input[k] == 0 && k < input.len() - 1 {
            output.push(self.alphabet[0] as char);

            k += 1;
        }

        // convert digits to a string
        let mut q: i32 = (digits.len() - 1) as i32;
        while q >= 0 {
            let uu: u8 = self.alphabet[digits[q as usize] as usize];
            let xx = uu as char;

            output.push(xx);

            q -= 1;
        }

        output
    }

    pub fn from_bs58(&self, input: &String) -> Option<Vec<u8>> {
        if input.is_empty() {
            return None;
        }

        let alphabet_map = lookup_table(self.alphabet);
        let base = self.alphabet.len() as u16;
        let ledger = self.alphabet[0] as char;

        let mut bytes: Vec<u8> = vec![];
        let mut i = 0;
        while i < input.len() {
            let c = input.as_str().chars().nth(i)?;
            let val = alphabet_map.get(&c)?;
            let mut j = 0;
            let mut carry: u16 = *val as u16;
            while j < bytes.len() {
                carry += bytes[j] as u16 * base;
                bytes[j] = carry as u8;
                carry >>= 8;

                j += 1;
            }

            while carry > 0 {
                bytes.push(carry as u8);
                carry >>= 8;
            }

            i += 1;
        }

        // deal with leading zeros
        let mut k = 0;
        while input.as_str().chars().nth(k).unwrap() == ledger && k < input.len() - 1 {
            bytes.push(0);
            k += 1;
        }

        bytes.as_mut_slice().reverse();

        Some(bytes)
    }
}

/// Pre-compute lookup table
fn lookup_table(alphabet: &[u8]) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    alphabet.iter().enumerate().for_each(|(idx, key)| {
        map.insert(*key as char, idx);
    });

    map
}

#[cfg(test)]
mod test_set {
    use super::*;

    #[test]
    fn to_bs58_works() {
        let src = vec![28, 215, 33, 155];
        let encoded = BaseX::with_alphabet(ALPHABET_BITCOIN).to_bs58(&src);
        assert_eq!(encoded, "jkuzA".to_string());
    }

    #[test]
    fn from_bs58_works() {
        let src = "jkuzA".to_string();
        let decoded = BaseX::with_alphabet(ALPHABET_BITCOIN).from_bs58(&src);
        assert_eq!(decoded, Some(vec![28, 215, 33, 155]));
    }
}
