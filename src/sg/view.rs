pub trait View {
    fn trigger(&self, event: &'static str) -> ();
}
