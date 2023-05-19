pub trait Observer {
    type Subject;
    fn observe(&self, subject: &Self::Subject);
}
