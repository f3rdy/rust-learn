extern crate clap;
use clap::{Arg, App, SubCommand};

fn main() {
    let _matches = App::new("My CLI app")
        .version("1.0")
        .author("Fred Thiele <ferdy_news at mailbox dot org>")
        .about("CLI app does wonderful stuff")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("test")
            .about("controls testing features")
            .version("1.3")
            .author("Foo Bar foo.bar at foo dot com")
            .arg(Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely")))
        .get_matches();

}
