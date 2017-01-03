use std::cell::Cell;

pub enum DataSourceType {
    Fixed(Vec<String>),
    Stdin,
}

pub struct DataSourceFactory;

impl DataSourceFactory {

    pub fn create(data_source_type: DataSourceType) -> DataSource {
        match data_source_type {
            DataSourceType::Fixed(lines) => {
                DataSource { data_source_strategy: Box::new(FixedDataSource::new(lines)) }
            },
            DataSourceType::Stdin => {
                DataSource { data_source_strategy: Box::new(FixedDataSource::new(vec![])) }
            }
        }
    }
}

pub trait DataSourceLike {
    fn try_next_line(&self) -> Option<String>;
}

pub struct DataSource {
    pub data_source_strategy: Box<DataSourceLike>,
}

impl DataSource {

    pub fn try_next_line(&self) -> Option<String> {
        self.data_source_strategy.try_next_line()
    }

}

struct FixedDataSource {
    lines: Vec<String>,
    iterator_index: Cell<usize>,
}

impl FixedDataSource {

    pub fn new(lines: Vec<String>) -> Self {
        FixedDataSource { lines, iterator_index: Cell::new(0) }
    }

}

impl DataSourceLike for FixedDataSource {

    fn try_next_line(&self) -> Option<String> {
        match self.lines.get(self.iterator_index.get()) {
            Some(string) => { 
                let return_value = Some(string.to_string());
                self.iterator_index.set(self.iterator_index.get() + 1);
                return_value
            },
            None => None
        }

    }
}

#[cfg(test)]
mod tests {

    use super::{DataSourceType, DataSourceFactory};

    #[test]
    fn fixed_data_source_return_lines_of_data() {
        let lines = vec!["first one".to_string(), "second one".to_string(), "third one".to_string()];
        let data_source = DataSourceFactory::create(DataSourceType::Fixed(lines));
        assert_eq!(Some("first one".to_string()), data_source.try_next_line());
        assert_eq!(Some("second one".to_string()), data_source.try_next_line());
        assert_eq!(Some("third one".to_string()), data_source.try_next_line());
        assert_eq!(None, data_source.try_next_line());
    }
}
