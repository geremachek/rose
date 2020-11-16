use std::collections::HashMap;
use std::io::{self, Write, BufRead};
use std::env;

mod arithmetic;
mod parse;

const ANSWER: &str = "Ans";

pub struct Repl {
	pub silent: bool,
	pub format: bool,
	pub reverse: bool,

	vars: HashMap<String, f64>,
}

impl Repl {
	pub fn new() -> Repl {
		let hash = 
			[(ANSWER.to_string(), 0.0),
			 ("π".to_string(), std::f64::consts::PI),
			 ("pi".to_string(), std::f64::consts::PI),
			 ("τ".to_string(), std::f64::consts::PI*2.0),
			 ("tau".to_string(), std::f64::consts::PI*2.0),
			 ("e".to_string(), std::f64::consts::E)]
			.iter().cloned().collect();

		Repl { silent: false, format: true, reverse: false, vars: hash }
	}

	pub fn start(&mut self) {
		let mut input = String::new();
		let mut result: parse::ReplResult;
		let mut prompt = String::new();

		if let Ok(p) = env::var("ROSE_PROMPT") {
			prompt = p;
		}

		loop {
			input.clear();
			
			print!("{}", prompt);

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

	pub fn match_result(&mut self, result: parse::ReplResult) {
		match result {
			parse::ReplResult::Answer(n) => {
				self.output_result(n, self.silent);
				self.vars.insert(String::from(ANSWER), n);
			}
			parse::ReplResult::Output(n) => self.output_result(n, false),
			parse::ReplResult::Error     => eprintln!("?"),
			_                            => (),
		}
	}

	fn output_result(&self, result: f64, silent: bool) {
		if !silent {
			let mut postfix = String::new();
			
			if self.format {
				match env::var("ROSE_FORMAT_PREFIX") {
					Ok(p)  => print!("{}", p),
					Err(_) => print!("  -> "),
				}

				if let Ok(p) = env::var("ROSE_FORMAT_POSTFIX") {
					postfix = p;
				}
			}

			println!("{}{}", result, postfix);
		}
	}

}
