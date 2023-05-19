use std::{ops::Deref, vec::IntoIter};

struct WrappedVec<T>(Vec<T>);

impl<T> Deref for WrappedVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> WrappedVec<T> {
    fn into_iter(self) -> IntoIter<T> {
        self.0.into_iter()
    }
}

fn main() {
    let wrapped_vec = WrappedVec(vec![1, 2, 3]);
    wrapped_vec.iter().for_each(|v| println!("{}", v));
    wrapped_vec.into_iter().for_each(|v| println!("{}", v));
}
