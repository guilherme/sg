pub struct ResultSet {
    all_data: Vec<String>,
    matches: Vec<String>,
    pending_data: Vec<String>,
    filter_string: Option<String>,
}

impl ResultSet {

    pub fn new() -> Self {
        ResultSet { all_data: vec![], matches: vec![], pending_data: vec![], filter_string: None }
    }

    pub fn push(&mut self, result: &str) {
        self.pending_data.push(result.to_string());
    }

    pub fn filtered_length(&self) -> usize {
        self.matches.len()
    }

    pub fn set_filter_string(&mut self, filter_string: &str) {
        self.filter_string = Some(filter_string.to_string());
    }

}

pub struct Filter;

impl Filter {

    pub fn filter(result_set: &mut ResultSet, filter_string: &str) {
        let matches = result_set.pending_data.clone().into_iter().filter(|x| x.contains(filter_string) ).collect();
        result_set.matches = matches;
        result_set.all_data.append(&mut result_set.pending_data);
        result_set.set_filter_string(filter_string);
        // trigger filter complete event?
    }

}

#[test]
fn it_does_basic_filtering_correctly() {
    let mut result_set = ResultSet::new();
    result_set.push("abcd");
    result_set.push("cdef");
    result_set.push("efgh");
    Filter::filter(&mut result_set, "cd");
    assert_eq!(result_set.filtered_length(), 2);
    assert_eq!(result_set.filter_string, Some("cd".to_string()));
    assert_eq!(result_set.pending_data.len(), 0);
    assert_eq!(result_set.all_data.len(), 3);
}
