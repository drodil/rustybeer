extern crate clap;
use clap::{App, SubCommand};

pub fn add_subcommand<'a, 'b>(app: App<'a, 'b>) -> App<'a, 'b>{
    let ret = app.subcommand(SubCommand::with_name("priming"));
    ret
}
