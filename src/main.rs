extern crate clap;

use clap::{Arg, App, crate_version};

mod arithmetic;
mod standard;
mod stack;
mod errors;
mod calculator;

fn main() {
	let rose_args = App::new("rose:")
				.about("A simple (reverse) polish notation evaluator and stack calculator")
				.version(crate_version!())
				.arg(Arg::with_name("stack")
					.short("S")
					.long("stack")
					.help("Use a stack based calculator"))
				.arg(Arg::with_name("reverse")
					.short("r")
					.long("reverse")
					.conflicts_with("stack")
					.help("Enables RPN mode"))
				.arg(Arg::with_name("evaluate")
					.short("e")
					.long("evaluate")
					.help("Evaluate stdin line by line"))
				.arg(Arg::with_name("silent")
					.short("s")
					.long("silent")
					.help("Supress output"))
				.arg(Arg::with_name("format")
					.short("f")
					.long("format")
					.help("Don't format output"))
				.arg(Arg::with_name("EXPRESSION")
					.index(1)
					.help("Expression to evaluate"))
				.get_matches();

	let mut rev = None;

	// not in stack mode

	if !rose_args.is_present("stack") {
		rev = Some(rose_args.is_present("reverse")); // only standard mode requires the reverse option
	}

	// create our new calculator

	let mut rose = calculator::new_calc(rose_args.is_present("silent"),
		!rose_args.is_present("format"),
		rev);

	if let Some(e) = rose_args.value_of("EXPRESSION") { // if we get an expression argument, parse it!
		let result = rose.meta_parse(e);
		rose.handle(&result);
	} else if rose_args.is_present("evaluate") { // evaluate option has been passed
		rose.parse_stdin() // parse stdin
	} else {
		rose.start() // start the shell!
	}
}
