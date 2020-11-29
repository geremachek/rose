use std::io::{self, Write, BufRead};
use crate::errors;

pub mod enviroment;
mod config;

// enum representing the possible values returned by our calculator

pub enum CalcResult {
	Answer(f64),
	Output(f64),
	Message(String),
	Error(errors::RoseError),
	Quit,
	None,
}

pub trait Calculator {
	// create a new calculator

	fn new(s: bool, f: bool) -> Self where Self: Sized;
	
	// the calculator's parser

	fn parse(&mut self, elems: Vec<&str>) -> Vec<CalcResult>;

	// get the enviroment variable

	fn get_env(&self) -> &enviroment::Enviroment;

	// handle results we get from the parser, return true if we are quiting

	fn handle(&self, results: Vec<CalcResult>) -> bool {
		for r in results {
			match r {
				CalcResult::Answer(n)  => self.get_env().output_result(n, self.get_env().silent),
				CalcResult::Output(n)  => self.get_env().output_result(n, false),
				CalcResult::Message(m) => print!("{}", m),
				CalcResult::Error(_)   => eprintln!("?"),
				CalcResult::Quit       => return true,
				CalcResult::None       => (),
			}
		}

		false
	}

	// start an interative REPL

	fn start(&mut self) {
		let mut input = String::new();

		loop {
			input.clear();
			
			print!("{}", self.get_env().conf.prompt);

			std::io::stdout()
				.flush()
				.expect("rose: unable to flush stdout");

			io::stdin()
				.read_line(&mut input)
				.expect("rose: unable to read line");
		
			let results = self.prep_parse(&input);

			if self.handle(results) {
				break;
			}

		}
	}

	// parse standard input (a file or some text) instead of launching into a repl

	fn parse_stdin(&mut self) {
		let stdin = io::stdin();

		for line in stdin.lock().lines() {
			let result = self.prep_parse(&line
				.expect("rose: couldn't read from stdin"));

			if self.handle(result) {
				break;
			}

		}
	}

	// prepare our string for parsing and then parse it

	fn prep_parse(&mut self, line: &str) -> Vec<CalcResult> {
		let trimmed = line.split_at(line.chars()
			.position(|c| c == '#')
			.or_else(|| Some(line.len()))
			.unwrap()).0;

		if !trimmed.trim().is_empty() {
			return self.parse(trimmed.split_whitespace().collect());	
		}

		Vec::new()
	}
}

pub fn new_calculator(stack: bool, s: bool, f: bool, r: bool) -> Box<dyn Calculator> {
	if stack {
		return Box::new(crate::stack::Stack::new(s, f));
	}

	let mut standard = Box::new(crate::standard::Standard::new(s, f));
	standard.reverse = r;

	return standard;
}
