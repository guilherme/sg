use super::view::View;
use super::data_source::DataSource;
use crossbeam::sync::MsQueue;

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
    data_source: DataSource,
    data_to_filter: MsQueue<String>,
}

impl App {

    pub fn new(filter: String, return_type: ReturnType, unfiltered_results: Vec<String>, filtered_results: Vec<String>, view: Box<View>, data_source: DataSource) -> Self {
        // Need a global "done" boolean
        App { filter, return_type, unfiltered_results, filtered_results, view, data_source, data_to_filter: MsQueue::new() }
    }

    pub fn start(&mut self) -> i32 {
        self.initialize_ui();
        self.listen_for_data_to_filter();
        self.start_filtering();
        self.update_ui();
        0 // TODO does this function "need" to return anything?
    }

    pub fn trigger(&self, event: &'static str) {
        self.view.trigger(event);
    }

    pub fn results(&self) -> Vec<String> {
        match self.return_type {
            ReturnType::All => { self.filtered_results.clone() }
            ReturnType::Selected => { vec![] } // TODO make this return the selected results
        }
    }

    //-------- private ---------//

    fn initialize_ui(&self) {
        self.view.init();
    }

    fn listen_for_data_to_filter(&self) {
        // TODO this needs to be on it's own thread
        let mut done = false;
        while !done {
            match self.data_source.try_next_line() {
                None => { done = true; }
                Some(line) => { self.data_to_filter.push(line); }
            }
        }
    }

    fn start_filtering(&mut self) {
        let mut done = false;
        while !done {
            match self.data_to_filter.try_pop() {
                None => { done = true; }
                Some(line) => { self.filter_line(line); },
            }
        }
    }

    fn update_ui(&self) {
        // TODO take things off the post filter queue and update the UI if needed
    }

    // ---------- private ----------- //

    fn filter_line(&mut self, line: String) {
        match line.contains(&self.filter) {
            // TODO have to trigger some kind on conditional variable to say something has changed.
            true => { self.filtered_results.push(line); }
            false => { self.unfiltered_results.push(line); }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::ReturnType;
    use super::super::{AppFactory, DataSourceType};

    #[test]
    fn it_does_basic_filtering() {
        let headless = true;
        let filter = String::from("man");
        let data_source_type = DataSourceType::Fixed(vec!["superman".to_string(), "joker".to_string(), "batman".to_string()]);
        let return_type = ReturnType::All;
        let mut app = AppFactory::create(headless, filter, return_type, data_source_type);
        app.start();
        app.trigger("control c");
        let results = app.results();
        assert_eq!(results, vec!["superman".to_string(), "batman".to_string()]);
    }
}
