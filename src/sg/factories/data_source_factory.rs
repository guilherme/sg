use super::super::data_source::{DataSource, DataSourceType, FixedDataSource};
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
