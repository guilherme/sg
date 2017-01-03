use super::super::headless_ui::HeadlessUI;
use super::super::view::View;
use super::super::data_source::{DataSourceType, DataSourceFactory, DataSource};
use super::super::{ReturnType, App};

pub struct AppFactory;

impl AppFactory {

    pub fn create(headless: bool, filter: String, return_type: ReturnType, data_source_type: DataSourceType) -> App {
        let view = AppFactory.create_view(headless);
        let data_source = AppFactory.create_data_source(data_source_type);
        App::new(filter, return_type, vec![], vec![], view, data_source)

    }

    //------- private --------//

    fn create_view(self, headless: bool) -> Box<impl View> {
        // TODO handle when headless == false
        Box::new(HeadlessUI::new())
    }

    fn create_data_source(self, data_source_type: DataSourceType) -> DataSource {
        match data_source_type {
            DataSourceType::Fixed(lines) => { DataSourceFactory::create(DataSourceType::Fixed(lines)) }
            _ => { // handle other data source type properly
                let lines = vec!["first one".to_string(), "second one".to_string(), "third one".to_string()];
                DataSourceFactory::create(DataSourceType::Fixed(lines))
            }
        }
    }

}
