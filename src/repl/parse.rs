use super::arithmetic;
use crate::repl;

impl repl::Repl {
	// parse a line

	pub fn parse(&mut self, line: &str) -> ReplResult {
		let mut result = ReplResult::None;

		if line != "\n" && !line.trim().is_empty() {
			let elems: Vec<&str> = line.split_whitespace().collect();
			let exp = self.evalutate_expression(&elems);

			if let ReplResult::Answer(_) = exp {
				result = exp;
			} else {
				match elems[0] {
					"quit"   | "q" => result = ReplResult::Quit,
					"put"    | "p" => result = ReplResult::Output(*self.vars.get(repl::ANSWER).unwrap()),
					"set"    | "=" =>
						if elems.len() == 3 {
							if let Some(n) = self.vars.clone().get(elems[2]) { // is the value a variable?
								self.vars.insert(elems[1].to_string(), *n); // it is! We set the variable.
							} else {
								if let Ok(n) = elems[2].parse::<f64>() {
									self.vars.insert(elems[1].to_string(), n);
								} else { result = ReplResult::Error; }
							}
						} else {
							result = ReplResult::Error;
						}
					"silent"  | "s" => self.silent = !self.silent,
					"format"  | "f" => self.format = !self.format,
					"reverse" | "r" => self.reverse = !self.reverse,
					"memory" | "m" =>
						for (name, value) in &self.vars {
							println!("{}: {}", name, value);
						}
					_ => result = ReplResult::Error,
				}
			}
		}

		result
	} 

	// evaluate an expression

	fn evalutate_expression(&self, elems: &Vec<&str>) -> ReplResult {
		if elems.len() == 1 { // only one element
			return result_to_repl(self.check_value(elems[0])) // return the value of that element
		} else {
			let mut symbol = elems[0];
			let mut begin = 1;
			let mut end = elems.len();

			if self.reverse { // are we in RPN mode?
				symbol = elems.iter().last().unwrap(); // set where we should check for the operator symbol
				
				begin = 0; // shift the location of values
				end = end - 1;
			}

			if let Some(o) = arithmetic::Operator::from(symbol) {
				let mut values = Vec::new();

				for val in elems.iter().skip(begin).take(end) {
					match self.check_value(val) {
						Ok(v)  => values.push(v),
						Err(_) => return ReplResult::Error,
					}
				}

				return ReplResult::Answer(arithmetic::Numbers::from(values).operate(o).unwrap())
			}
		}

		ReplResult::Error
	}

	// check if a value, a literal or not, is valid

	fn check_value(&self, val: &str) -> Result<f64, std::num::ParseFloatError> {
		match self.vars.get(val) {
			Some(n) => Ok(*n),
			None    => val.parse::<f64>(),
		}

	}
}

fn result_to_repl<E>(r: Result<f64, E>) -> ReplResult {
	match r {
		Ok(v)  => ReplResult::Answer(v),
		Err(_) => ReplResult::Error,
	}
}

pub enum ReplResult {
	Answer(f64),
	Output(f64),
	Error,
	Quit,
	None,
}
