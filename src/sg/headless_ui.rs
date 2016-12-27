use super::view::View;
use super::super::InputSource;

pub struct HeadlessUI {
    input_source: InputSource,
}

impl HeadlessUI {

    pub fn new(input_source: InputSource) -> Self {
        HeadlessUI { input_source }
    }
}

impl View for HeadlessUI {

    fn init(&self) {
        // TODO implement me
    }

    fn trigger(&self, event: &'static str) -> () {
        // TODO implement me
    }
}
