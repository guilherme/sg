extern crate sg;
extern crate clap;

use clap::App as Cli;
use clap::Arg;

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

  App::new(headless).start();

}
