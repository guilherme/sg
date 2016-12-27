pub trait View {

    fn init(&self);

    fn trigger(&self, event: &'static str);

}

