pub struct App {
    headless: bool,
    unfiltered_results: Vec<String>,
    filtered_results: Vec<String>,
}

impl App {

    pub fn new(headless: bool) -> Self {
        App { headless: headless, unfiltered_results: vec![], filtered_results: vec![] }
    }

    pub fn start(&self) -> i32 {
        if self.headless { println!("Running in headless mode!") }
        0
    }

}
