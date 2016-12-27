pub enum InputSource {
    Fixed(Vec<String>),
    Stdin,
}

pub enum ReturnType {
    All,
    Selected,
}

pub struct HeadlessUI;

impl View for HeadlessUI {

    fn trigger(&self, event: &'static str) -> () {
    }
}

trait View {
    fn trigger(&self, event: &'static str) -> ();
}

pub struct App {
    headless: bool,
    unfiltered_results: Vec<String>,
    filtered_results: Vec<String>,
    filter: String,
    input: InputSource,
    return_type: ReturnType,
}

impl App {


    pub fn new(headless: bool, filter: String, input: InputSource, return_type: ReturnType) -> Self {
        App { headless, filter, input, return_type, unfiltered_results: vec![], filtered_results: vec![] }
    }

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
        vec![String::from("superman"), String::from("batman")]
    }


    //-------- private ---------//

    fn create_ui(&self) {
        if self.headless { println!("Running in headless mode!") }
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

    use super::{App, InputSource, ReturnType, View};

    #[test]
    fn it_does_basic_filtering() {
        let headless = true;
        let filter = String::from("man");
        let input_source =  InputSource::Fixed(vec!["superman".to_string(), "joker".to_string(), "batman".to_string()]);
        let return_type = ReturnType::All;
        let app = App::new(headless, filter, input_source, return_type);
        app.start();
        app.view().trigger("control c");
        let results = app.results();
        assert_eq!(results, vec!["superman".to_string(), "batman".to_string()]);
    }
}
