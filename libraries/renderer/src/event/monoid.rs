pub trait Monoid {
    fn empty() -> Self;

    fn combine(&self, other: &Self) -> Self;
}

impl Monoid for () {
    fn empty() -> Self {}

    fn combine(&self, _other: &Self) -> Self {}
}

impl<T: Clone> Monoid for Vec<T> {
    fn empty() -> Self {
        Self::new()
    }

    fn combine(&self, other: &Self) -> Self {
        let mut combined = self.clone();
        combined.extend_from_slice(other);
        combined
    }
}
