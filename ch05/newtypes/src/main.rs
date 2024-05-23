#[derive(Debug)]
struct BitCount(u32);
#[derive(Debug)]
struct ByteCount(u32);

impl BitCount {
    fn to_bytes(&self) -> ByteCount {
        ByteCount(self.0 / 8)
    }
}

impl ByteCount {
    fn to_bits(&self) -> BitCount {
        BitCount(self.0 * 8)
    }
}

fn main() {
    let bits = BitCount(8);
    let bytes = ByteCount(12);
    dbg!(&bits);
    dbg!(&bytes);

    dbg!(bits.to_bytes());
    dbg!(bytes.to_bits());

    dbg!(bits.to_bytes().to_bits());
    dbg!(bytes.to_bits().to_bytes());

    dbg!(bits.0);
    dbg!(bytes.0);
}
