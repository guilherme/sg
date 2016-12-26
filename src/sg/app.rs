pub enum InputSource {
    Fixed(Vec<String>),
    Stdin,
}

pub enum ReturnType {
    All,
    Selected,
}

struct Filter;

impl Filter {

    pub fn new() -> Self {
        Filter
    }

    pub fn start(&self) {
        // TODO this should listen for filter events and filter
        // for that it will need to know the current ins and outs and keep track of the current and
        // previous filters
    }

    pub fn event_queue(&self) -> () {
        () // TODO return the real event queue
    }
}

pub struct App {
    headless: bool,
    unfiltered_results: Vec<String>,
    filtered_results: Vec<String>,
    filter: Filter,
    input: InputSource,
    return_type: ReturnType,
}

impl App {

    pub fn new(headless: bool, filter_string: String, input: InputSource, return_type: ReturnType) -> Self {
        let filter = Filter::new();
        App {
            headless,
            input,
            return_type,
            filter,
            unfiltered_results: vec![],
            filtered_results: vec![],
        }
    }

    pub fn start(&self) -> i32 {
        self.create_ui();
        self.start_filtering();
        self.update_ui();
        0 // TODO exit code, make this dynamic
    }


    //-------- private ---------//

    fn create_ui(&self) {
        if self.headless { println!("Running in headless mode!") } // TODO remove this after non-headless mode created
        // TODO create a view
        // TODO send the view to the UI for rendering
    }

    fn start_filtering(&self) {
        self.filter.start();
    }

    fn update_ui(&self) {
        // TODO listen for filter events and update UI as required
        let _ = self.filter.event_queue();
    }

}
