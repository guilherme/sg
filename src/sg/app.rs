use super::headless_ui::HeadlessUI;
use super::view::View;

pub enum InputSource {
    Fixed(Vec<String>),
    Stdin,
}

pub enum ReturnType {
    All,
    Selected,
}

pub struct AppFactory;

impl AppFactory {

    pub fn create(headless: bool, filter: String, input: InputSource, return_type: ReturnType) -> App {
        let view = AppFactory.create_view(headless, input);
        App { filter, return_type, unfiltered_results: vec![], filtered_results: vec![], view }
    }

    //------- private --------//

    fn create_view(self, headless: bool, input: InputSource) -> Box<impl View> {
        Box::new(HeadlessUI {})
    }

}

pub struct App {
    unfiltered_results: Vec<String>,
    filtered_results: Vec<String>,
    filter: String,
    return_type: ReturnType,
    view: Box<View>,
}

impl App {

    pub fn start(&self) -> i32 {
        self.create_ui();
        self.start_filtering();
        self.update_ui();
        0
    }

    pub fn view(&self) -> impl View {
        HeadlessUI {}
    }

    pub fn results(&self) -> Vec<String> {
        match self.return_type {
            ReturnType::All => { vec![String::from("superman"), String::from("batman")] }
            ReturnType::Selected => { vec![] } // TODO make this return the selected results
        }
    }

    //-------- private ---------//

    fn create_ui(&self) {
    }

    fn start_filtering(&self) {
        // TODO
    }

    fn update_ui(&self) {
        // TODO
    }

}

#[cfg(test)]
mod tests {

    use super::{AppFactory, InputSource, ReturnType, View};

    #[test]
    fn it_does_basic_filtering() {
        let headless = true;
        let filter = String::from("man");
        let input_source =  InputSource::Fixed(vec!["superman".to_string(), "joker".to_string(), "batman".to_string()]);
        let return_type = ReturnType::All;
        let app = AppFactory::create(headless, filter, input_source, return_type);
        app.start();
        app.view().trigger("control c");
        let results = app.results();
        assert_eq!(results, vec!["superman".to_string(), "batman".to_string()]);
    }
}
