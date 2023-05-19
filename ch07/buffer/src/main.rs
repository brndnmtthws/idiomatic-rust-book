struct Buffer<T, const LENGTH: usize> {
    buf: [T; LENGTH],
}

impl<T, const LENGTH: usize> From<[T; LENGTH]> for Buffer<T, LENGTH> {
    fn from(buf: [T; LENGTH]) -> Self {
        Buffer { buf }
    }
}

fn main() {}
