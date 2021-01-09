extern crate clap;

use clap::{Arg, App};

mod arithmetic;
mod standard;
mod stack;
mod errors;
mod calculator;

fn main() {
	let rose_args = App::new("rose")
				.version("2.0")
				.about("A simple (reverse) polish notation evaluator and stack calculator")
				.author("J. Geremachek")
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

	let mut rev = None;

	if !rose_args.is_present("stack") {
		rev = Some(rose_args.is_present("reverse"));
	}

	let mut rose = calculator::new_calc(rose_args.is_present("silent"),
		!rose_args.is_present("format"),
		rev);

	if rose_args.is_present("evaluate") {
		if let Some(e) = rose_args.value_of("EXPRESSION") {
			let result = rose.parse(&calculator::prep_str(e));

			rose.handle(&result);
		} else {
			rose.parse_stdin();
		}
	} else {
		rose.start()
	}
}
