use super::super::headless_ui::HeadlessUI;
use super::super::view::View;
use super::super::{InputSource, ReturnType, App};


pub struct AppFactory;

impl AppFactory {

    pub fn create(headless: bool, filter: String, input_source: InputSource, return_type: ReturnType) -> App {
        let view = AppFactory.create_view(headless, input_source);
        App::new(filter, return_type, vec![], vec![], view)
    }

    //------- private --------//

    fn create_view(self, headless: bool, input_source: InputSource) -> Box<impl View> {
        Box::new(HeadlessUI::new(input_source))
    }

}
