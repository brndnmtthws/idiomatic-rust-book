#[derive(
    Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
struct KitchenSink;

trait FullFeatured {}

impl<T> FullFeatured for T where
    T: Clone
        + Copy
        + std::fmt::Debug
        + Default
        + Eq
        + std::hash::Hash
        + Ord
        + PartialEq
        + PartialOrd
{
}

#[derive(Debug)]
struct Container<T: FullFeatured> {
    t: T,
}

fn main() {
    let container = Container { t: KitchenSink {} };
    println!("{:?}", container);
}
