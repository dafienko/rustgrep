use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

	let config = Config::build(&args).unwrap_or_else(|err| {
		println!("problem processing arguments: {err}");
		process::exit(-1);
	});

	println!("searching for {}", config.query);
	println!("in file path {}", config.file_path);

	if let Err(e) = minigrep::run(config) {
		println!("Application failed with error: {e}");
		process::exit(-1);
	}
}
