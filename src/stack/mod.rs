use crate::{errors::RoseError, calculator::{Calculator, CalcResult, enviroment::Enviroment}};

mod parse;

// Stack calculator

pub struct Stack {
	env: Enviroment,
	stack: Vec<f64>,
}

impl Stack {
	// create a new stack calculator

	pub fn new(s: bool, f: bool) -> Stack {
		Stack { env: Enviroment::new(s, f), stack: Vec::new() }
	}

	// display the stack

	fn show_stack(&self) -> String {
		let mut shown = String::new();
		
		if !self.stack.is_empty() {
			for n in &self.stack {
				shown.push_str(&format!("{} ", n));
			}

			shown.push_str("\n")
		}

		shown
	}

	// swap values

	fn twirl(&mut self) {
		let len = self.stack.len();

		if len > 1 {
			self.stack.swap(len-2, len-1)
		}
	}
}

impl Calculator for Stack {
	// return our enviroment structure

	fn get_env(&self) -> &Enviroment {
		&self.env
	}

	// stack parser

	fn parse(&mut self, elems: &[&str]) -> Result<Vec<CalcResult>, RoseError> {
		let mut results = Vec::new();

		for e in elems {
			results.push(self.parse_element(e)?)
		}

		Ok(results)
	}
}
