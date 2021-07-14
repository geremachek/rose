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

	fn show_stack(&self) -> String {
		let mut shown = String::new();
		
		if !self.stack.is_empty() {
			for n in &self.stack {
				shown.push_str(&n.to_string());
			}

			shown.push_str("\n")
		}

		shown
	}

	fn reverse_stack(&mut self) {
		self.stack = reverse(&self.stack);
	}

	fn twirl(&mut self) {
		let len = self.stack.len();

		if len > 1 {
			let second = self.stack[len - 2]; // save the second to last item

			self.stack.remove(len - 2); // remove the second to last item
			self.stack.push(second); // push it to the end of the stack
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

fn reverse<T: Copy>(items: &Vec<T>) -> Vec<T> {
	let mut reversed = Vec::new();

	for i in items.iter().rev() {
		reversed.push(*i);
	}

	reversed
}
