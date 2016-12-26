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
        App { headless, unfiltered_results: vec![], filtered_results: vec![], filter, input, return_type }
    }

    pub fn start(&self) -> i32 {
        if self.headless { println!("Running in headless mode!") }
        0
    }

}
