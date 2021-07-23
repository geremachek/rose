use std::io::{self, Write, BufRead};
use crate::errors::RoseError;

pub mod enviroment;
mod config;

// macro that deals with handling output and quiting

macro_rules! handle_quit {
	($s:ident, $r:ident) => {
		if $s.handle($r) {
			break
		}
	}
}

// enum representing the possible values returned by our calculator

pub enum CalcResult {
	Answer(f64),
	Output(f64),
	Message(String),
	Quit,
	None,
}

pub trait Calculator {
	// the calculator's parser, it returns an array of results to handle

	fn parse(&mut self, elems: &[&str]) -> Result<Vec<CalcResult>, RoseError>;

	// get the enviroment variable

	fn get_env(&self) -> &enviroment::Enviroment;

	// handle results we get from the parser, return true if we are quiting

	fn handle(&self, results: &Result<Vec<CalcResult>, RoseError>) -> bool {
		match results {
			Ok(res) => {		
				for r in res {
					match r {
						CalcResult::Answer(n)  => self.get_env().output_result(*n, self.get_env().silent),
						CalcResult::Output(n)  => self.get_env().output_result(*n, false),
						CalcResult::Message(m) => print!("{}", m),
						CalcResult::Quit       => return true,
						CalcResult::None       => (),
					}
				}
			}
			Err(_)  => eprintln!("?"),
		}

		false
	}

	// start an interative REPL

	fn start(&mut self) {
		let mut input = String::new();

		loop {
			print!("{}", self.get_env().conf.prompt);

			std::io::stdout()
				.flush()
				.expect("rose: unable to flush stdout");

			io::stdin()
				.read_line(&mut input)
				.expect("rose: unable to read line");

			let results = &self.meta_parse(&input);
			
			handle_quit!(self, results);
			input.clear()
		}
	}

	// parse standard input (a file or some text) instead of launching into a repl

	fn parse_stdin(&mut self) {
		for line in io::stdin().lock().lines() {
			let result = &self.meta_parse(&line
				.expect("rose: couldn't read from stdin"));
		
			handle_quit!(self, result);
		}
	}

	// prepare our string for parsing and then parse it

	fn meta_parse(&mut self, line: &str) -> Result<Vec<CalcResult>, RoseError> {
		let prep = &line.split_at(line.chars()
			.position(|c| c == '#')
			.unwrap_or(line.len())).0
			.split_whitespace().collect::<Vec<&str>>()[..];

		if prep.is_empty() {
			return Ok(vec![CalcResult::None]);
		}
		
		self.parse(prep)
	}
}

// new calculator

pub fn new_calc(s: bool, f: bool, r: Option<bool>) -> Box<dyn Calculator> {
	if let Some(b) = r {
		return Box::new(crate::standard::Standard::new(s, f, b));
	}

	Box::new(crate::stack::Stack::new(s, f))
}
