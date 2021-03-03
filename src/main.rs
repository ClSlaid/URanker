//! # URanker's main functions...
//! As you see, it's mainly adapted from 6.824's mrsequencial.go

use std::env;
use clap::{Arg, App, SubCommand};
use uranker::MyReader;


fn startup(file: String) {
    let reader = MyReader::
}
fn main(){
    let matches = App::new("URanker")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .args_from_usage(
            "<INPUT>    'Sets the input file to use'"
        )
        .subcommand(SubCommand::with_name("file")
            .about("URL datasets")
        )
        .get_matches();

    if let Some(file) = matches.subcommand_matches("file") {
        startup(file);
    }

}