extern crate clap;
extern crate sg;

use clap::App as Cli;

use sg::App;

fn main() {

  let matches = Cli::new("SG")
                        .version("0.1")
                        .author("Sebastian Glazebrook, Guilherme Reis Campos")
                        .about("The best fuzzy finder ever")
                        .get_matches();

  let _ = App::new();
}
