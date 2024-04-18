#![feature(test)]

#[cfg(test)]
mod test {
    extern crate test;
    use test::Bencher;
    #[bench]
    fn hello_world_10_times(b: &mut Bencher) {
        b.iter(|| {
            for _ in 0..10 {
                println!("Hello, world!");
            }
        });
    }
}
