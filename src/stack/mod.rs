use crate::calculator::{Calculator, CalcResult, enviroment::Enviroment};

mod parse;

// Stack calculator

pub struct Stack {
	pub env: Enviroment,
	pub stack: Vec<f64>,
}

impl Stack {
	fn show_stack(&self) -> String {
		let mut shown = String::new();
		
		if !self.stack.is_empty() {
			for n in &self.stack {
				shown.push_str(&format!{"{} ", n});
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
	// create a new stack calculator

	fn new(s: bool, f: bool) -> Stack {
		Stack { env: Enviroment::new(s, f), stack: Vec::new() }
	}

	// return our enviroment structure

	fn get_env(&self) -> &Enviroment {
		&self.env
	}

	// stack parser

	fn parse(&mut self, elems: Vec<&str>) -> Vec<CalcResult> {
		let mut results = Vec::new();

		for e in elems {
			results.push(self.parse_element(e))
		}

		results
	}
}

fn reverse<T: Copy>(items: &Vec<T>) -> Vec<T> {
	let mut reversed = Vec::new();

	for i in items.iter().rev() {
		reversed.push(*i);
	}

	reversed
}
