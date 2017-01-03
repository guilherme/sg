extern crate sg;
extern crate clap;
use clap::App as Cli;
use clap::Arg;

use std::process::exit;

use sg::{AppFactory, ReturnType, DataSourceType};


fn main() {

  let matches = Cli::new("SG")
                        .version("0.1")
                        .author("Sebastian Glazebrook, Guilherme Reis Campos")
                        .about("The best fuzzy finder ever")
                        .arg(Arg::with_name("headless")
                             .short("h")
                             .long("headless")
                             .help("Run without a UI. Commonly used for testing"))
                        .arg(Arg::with_name("filter")
                             .short("f")
                             .long("filter")
                             .takes_value(true)
                             .value_name("FILTER")
                             .help("Filter string to filter the input"))
                        .arg(Arg::with_name("input")
                             .short("i")
                             .long("input")
                             .takes_value(true)
                             .value_name("INPUT")
                             .help("Input to filter, defaults to STDIN"))
                        .arg(Arg::with_name("return")
                             .short("r")
                             .long("return")
                             .takes_value(true)
                             .value_name("RETURN_TYPE")
                             .help("Configure what you want to return.\nDefault: 'selected-rows'\nAvailable options: 'all-rows', 'selected-rows'"))
                        .get_matches();

  let headless = matches.is_present("headless");


  let filter = match matches.value_of("filter") {
      Some(string) => { string.to_string() },
      None => { String::new() }
  };

  let data_source_type = match matches.value_of("input") {
      Some(string) => { DataSourceType::Fixed(string.lines().map(|line| line.to_string()).collect()) }
      None => { DataSourceType::Stdin }
  };

  let return_type = match matches.value_of("return") {
      Some(string) => {
          match string {
              "all-rows" => { ReturnType::All }
              "selected-rows" => { ReturnType::Selected }
              _ => {
                  println!("Unknown return value type");
                  ReturnType::Selected // TODO don't do this
              }
          }
      }
      None => { ReturnType::Selected }
  };

  let exit_code = AppFactory::create(headless, filter, return_type, data_source_type).start();

  exit(exit_code);
}
