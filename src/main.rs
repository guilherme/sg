extern crate sg;
extern crate clap;

use clap::App as Cli;
use clap::Arg;

use std::process::exit;

use sg::App;

fn main() {

  let matches = Cli::new("SG")
                        .version("0.1")
                        .author("Sebastian Glazebrook, Guilherme Reis Campos")
                        .about("The best fuzzy finder ever")
                        .arg(Arg::with_name("headless")
                             .short("h")
                             .long("headless")
                             .help("Run sg without a UI. Commonly used for testing"))
                        .get_matches();

  let headless = matches.is_present("headless");

  let exit_code = App::new(headless).start();

  exit(exit_code);
}
