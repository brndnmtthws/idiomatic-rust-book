pub trait ReverseExt<T> {
    fn reversed(&self) -> Vec<T>;
}

impl<T> ReverseExt<T> for Vec<T>
where
    T: Clone,
{
    fn reversed(&self) -> Vec<T> {
        self.iter().rev().cloned().collect()
    }
}

pub trait DoubleEndedIteratorExt: DoubleEndedIterator {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>;
}

impl<I: DoubleEndedIterator> DoubleEndedIteratorExt for I {
    fn to_reversed<'a, T>(self) -> Vec<T>
    where
        T: 'a + Clone,
        Self: Sized + Iterator<Item = &'a T>,
    {
        self.rev().cloned().collect()
    }
}

fn main() {
    let forward = vec![1, 2, 3];
    let reversed = forward.reversed();
    dbg!(&forward);
    dbg!(&reversed);

    let other_reversed = forward.iter().to_reversed();
    dbg!(&other_reversed);
}
