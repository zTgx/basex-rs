pub trait Encode {
    type Output;
    fn encode(src: AsRef<u8>) -> Self::Output;
}

pub trait Decode {
    type Output;
    fn decode(src: AsRef<u8>) -> Self::Output;
}
