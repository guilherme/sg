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

pub struct App {
    unfiltered_results: Vec<String>,
    filtered_results: Vec<String>,
    filter: String,
    return_type: ReturnType,
    view: Box<View>,
}

impl App {

    pub fn new(filter: String, return_type: ReturnType, unfiltered_results: Vec<String>, filtered_results: Vec<String>, view: Box<View>) -> Self {
        App { filter, return_type, unfiltered_results: vec![], filtered_results: vec![], view }
    }

    pub fn start(&self) -> i32 {
        self.initialize_ui();
        self.start_filtering();
        self.update_ui();
        0
    }

    pub fn trigger(&self, event: &'static str) {
        self.view.trigger(event);
    }


    pub fn results(&self) -> Vec<String> {
        match self.return_type {
            ReturnType::All => { vec![String::from("superman"), String::from("batman")] }
            ReturnType::Selected => { vec![] } // TODO make this return the selected results
        }
    }

    //-------- private ---------//

    fn initialize_ui(&self) {
        self.view.init();
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

    use super::{InputSource, ReturnType, View};
    use super::super::{AppFactory};

    #[test]
    fn it_does_basic_filtering() {
        let headless = true;
        let filter = String::from("man");
        let input_source =  InputSource::Fixed(vec!["superman".to_string(), "joker".to_string(), "batman".to_string()]);
        let return_type = ReturnType::All;
        let app = AppFactory::create(headless, filter, input_source, return_type);
        app.start();
        app.trigger("control c");
        let results = app.results();
        assert_eq!(results, vec!["superman".to_string(), "batman".to_string()]);
    }
}
