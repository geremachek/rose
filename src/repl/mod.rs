use std::collections::HashMap;
use std::io::{self, Write, BufRead};

mod arithmetic;
mod parse;

pub struct Repl {
	pub silent: bool,
	pub format: bool,

	vars: HashMap<String, f64>,
}

impl Repl {
	pub fn new() -> Repl {
		let hash = 
			[("$".to_string(), 0.0),
			 ("pi".to_string(), std::f64::consts::PI),
			 ("e".to_string(), std::f64::consts::E)]
			.iter().cloned().collect();

		Repl { silent: false, format: true, vars: hash }
	}

	pub fn start(&mut self) {
		let mut input = String::new();
		let mut result: parse::ReplResult;

		loop {
			input.clear();
			
			print!("( ");

			std::io::stdout()
				.flush()
				.expect("rose: unable to flush stdout");

			io::stdin()
				.read_line(&mut input)
				.expect("rose: unable to read line");
		
			result = self.parse(&input);

			match result {
				parse::ReplResult::Quit => break,
				_                       => self.match_result(result),
			}

		}
	}

	pub fn parse_stdin(&mut self) {
		let stdin = io::stdin();
		let mut result: parse::ReplResult;

		for line in stdin.lock().lines() {
			result = self.parse(&line
				.expect("rose: couldn't read from stdin"));

			match result {
				parse::ReplResult::Quit => break,
				_                       => self.match_result(result),
			}

		}
	}

	pub fn match_result(&self, result: parse::ReplResult) {
		match result {
			parse::ReplResult::Answer(n) => self.output_result(n, self.silent),
			parse::ReplResult::Output(n) => self.output_result(n, false),
			parse::ReplResult::Error     => eprintln!("?"),
			_                            => (),
		}
	}

	fn output_result(&self, result: f64, silent: bool) {
		if !silent {
			if self.format {
				print!("  => ");
			}

			println!("{}", result);
		}
	}
}
