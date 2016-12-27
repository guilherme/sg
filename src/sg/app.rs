pub enum InputSource {
    Fixed(Vec<String>),
    Stdin,
}

pub enum ReturnType {
    All,
    Selected,
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
