trait Blanket {}
impl<T> Blanket for T {}

struct Buffer<T, const LENGTH: usize> {
    buf: [T; LENGTH],
}

impl<T: Default + Copy, const LENGTH: usize> From<Vec<T>>
    for Buffer<T, LENGTH>
{
    fn from(v: Vec<T>) -> Self {
        assert_eq!(LENGTH, v.len());
        let mut ret = Self {
            buf: [T::default(); LENGTH],
        };
        ret.buf.copy_from_slice(&v);
        ret
    }
}

fn main() {
    println!("Hello, world!");
}
