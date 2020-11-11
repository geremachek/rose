use super::arithmetic;
use crate::repl;

pub enum ReplResult {
	Answer(f64),
	Output(f64),
	Error,
	Quit,
	None,
}

pub enum ExpressionError {
	Error,
}

impl repl::Repl {
	// parse a line

	pub fn parse(&mut self, line: &str) -> ReplResult {
		let mut result = ReplResult::None;
		let trimmed_line = trim_comment(line); // trim comments from the line

		if trimmed_line != "\n" && !trimmed_line.trim().is_empty() {
			let elems: Vec<&str> = trimmed_line.split_whitespace().collect();
			let exp = self.evaluate_expression(&elems);

			if let Ok(v) = exp {
				result = ReplResult::Answer(v);
			} else {
				match elems[0] {
					"quit"   | "q" => result = ReplResult::Quit,
					"put"    | "p" => result = ReplResult::Output(*self.vars.get(repl::ANSWER).unwrap()),
					"set"    | "=" | "var" =>
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

	fn evaluate_expression(&self, elems: &Vec<&str>) -> Result<f64, ExpressionError> {
		if elems.len() == 1 { // only one element
			return match self.check_value(elems[0]) {
				Ok(v)  => Ok(v),
				Err(_) => Err(ExpressionError::Error),
			}
		} else {
			let mut symbol = elems[0];
			let mut begin = 1;
			let mut end = elems.len();

			if self.reverse { // are we in RPN mode?
				symbol = elems.iter().last().unwrap(); // set where we should check for the operator symbol
				
				begin = 0; // shift the location of values
				end = end - 1;
			}

			if let Ok(o) = arithmetic::new_operator(symbol) {
				let mut values = Vec::new();
				let mut sub_expr = Vec::new();

				let mut sub_mode = false;
				
				// open and closed parentheses

				let mut open =   1;
				let mut closed = 0;

				for val in elems.iter().skip(begin).take(end) {
					if sub_mode {
						if val.starts_with("(") {
							open += val.matches("(").count();
						} else if val.ends_with(")") {
							closed += val.matches(")").count();
						}

						if open == closed {
							sub_expr.push(val.strip_suffix(")")
								.unwrap());

							values.push(self.evaluate_expression(&sub_expr)
								.or_else(|_| Err(ExpressionError::Error))?);

							sub_mode = false;
							sub_expr.clear();

							open = 1;
							closed = 0;
						} else {
							sub_expr.push(val)
						}
					} else {
						match self.check_value(val) {
							Ok(v)  => values.push(v),
							Err(_) => { // this could be a genuine error... or we could be using paranthesis.
								if val.starts_with("(") {
									sub_expr.push(&val.strip_prefix("(").unwrap());

									sub_mode = true;
								} else {
									return Err(ExpressionError::Error);
								}
							}
						}
					}
				}
				
				return match o.operate(values) {
					Ok(v)  => Ok(v),
					Err(_) => Err(ExpressionError::Error),
				}
			}
		}

		Err(ExpressionError::Error)
	}

	// check if a value, a literal or not, is valid

	fn check_value(&self, val: &str) -> Result<f64, std::num::ParseFloatError> {
		let mut trimmed_val = val.to_string();
		let mut sign = 1.0; // by default the sign is positive

		if val.starts_with("-") {
			trimmed_val = val.chars().skip(1).collect::<String>();
			sign = -1.0; // sign is negative
		}

		match self.vars.get(&trimmed_val) {
			Some(n) => Ok(*n*sign),
			None    => val.parse::<f64>(),
		}
	}
}

// remove comments

fn trim_comment(s: &str) -> &str {
	s.split_at(s.chars()
		.position(|c| c == '#')
		.or_else(|| Some(s.len()))
		.unwrap()).0
}

