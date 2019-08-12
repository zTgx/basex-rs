use std::collections::HashMap;

pub trait Encode {
    type Output;
    fn encode(&self, src: &[u8]) -> Self::Output;
}

pub trait Decode {
    type Output;
    fn decode(&self, src: String) -> Self::Output;
}

//default source ALPHABET.
pub fn generate_alpha_map(alphabet: &[u8]) -> HashMap<char, usize> {
    let mut map: HashMap<char, usize> = HashMap::new();
    let lens = alphabet.len();

    // pre-compute lookup table
    let mut i = 0;
    while i < lens {
        let x = alphabet[i] as char;
        map.insert(x, i);

        i += 1;
    }

    map
}
