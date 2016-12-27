use super::view::View;

pub struct HeadlessUI;

impl View for HeadlessUI {

    fn trigger(&self, event: &'static str) -> () {
    }
}
