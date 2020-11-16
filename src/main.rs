extern crate clap;
use clap::{Arg, App};

mod repl;

fn main() {
	let mut rose = repl::Repl::new();
	let rose_args = App::new("rose")
				.version("2.0")
				.about("A simple (reverse) polish notation calculator")
				.arg(Arg::with_name("reverse")
					.short("r")
					.long("reverse")
					.help("Enables RPN mode"))
				.arg(Arg::with_name("evaluate")
					.short("e")
					.long("evaluate")
					.help("Evaluate stdin line by line"))
				.arg(Arg::with_name("EXPRESSION")
					.index(1)
					.requires("evaluate")
					.help("Expression to evaluate (requires the \"-e\" flag)"))
				.arg(Arg::with_name("silent")
					.short("s")
					.long("silent")
					.help("Supress output"))
				.arg(Arg::with_name("format")
					.short("f")
					.long("format")
					.help("Don't format output"))
				.get_matches();

	// set options based on flags

	if rose_args.is_present("silent") {
		rose.silent = true;
	}

	if rose_args.is_present("format") {
		rose.format = false;
	}

	if rose_args.is_present("reverse") {
		rose.reverse = true;
	}

	if rose_args.is_present("evaluate") {
		if let Some(e) = rose_args.value_of("EXPRESSION") {
			let result = rose.parse(e);

			rose.match_result(result);
		} else {
			rose.parse_stdin();
		}
	} else {
		rose.start()
	}
}
