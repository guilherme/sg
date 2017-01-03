use super::view::View;

pub struct HeadlessUI;

impl HeadlessUI {

    pub fn new() -> Self {
        HeadlessUI { }
    }

    fn listen_for_input(&self) {
        // TODO listen for input from the user what does that mean for headless?
    }
}

impl View for HeadlessUI {

    fn init(&self) {
        self.listen_for_input();
    }

    fn trigger(&self, event: &'static str) -> () {
        // TODO implement me
        // put an event on a queue?
    }
}
