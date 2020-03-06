#[macro_use]
extern crate log;
extern crate simple_logger;

use structopt::StructOpt;

 #[derive(StructOpt)]
struct Cli {
    pattern:    String,
    #[structopt(parse(from_os_str))]
    path:       std::path::PathBuf,
}

fn print_args(pattern: &str, path: &std::path::PathBuf) {
    println!("pattern: {} path: {:#?}", pattern, path);
}

fn main() {
    simple_logger::init().unwrap();
    info!("hello");
    let _args = Cli::from_args();
    print_args(&_args.pattern, &_args.path);
}
