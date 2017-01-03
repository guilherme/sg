#![feature(field_init_shorthand)]
#![feature(conservative_impl_trait)]
extern crate crossbeam;

mod sg;

pub use sg::{App, AppFactory, ReturnType, DataSourceType};
