use schism::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("\n *** ERROR *** Problem parsing arguments: {err}\n");
        process::exit(1);
    });

    if let Err(e) = schism::run(config) {
        println!("\n *** Application error *** {e}\n");
        process::exit(1);
    }
}
