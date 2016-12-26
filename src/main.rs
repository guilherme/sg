extern crate clap;
extern crate sg;

use clap::{App};

use sg::*;

fn main() {

  let matches = App::new("SG")
                        .version("0.1")
                        .author("Sebastian Glazebrook, Guilherme Reis Campos")
                        .about("The best fuzzy finder ever")
                        .get_matches();
}
