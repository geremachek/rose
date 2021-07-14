use crate::{calculator::CalcResult, arithmetic, errors::RoseError};
use super::{Standard, ANSWER};

impl Standard {
	// parse commands or an expression and return the result

	pub fn parser(&mut self, elems: &[&str]) -> Result<CalcResult, RoseError> {
		if let Ok(v) = self.evaluate_expression(&elems) {
			self.env.store(ANSWER, v);
			return Ok(CalcResult::Answer(v));
		} else {
			match elems[0] { // match the command
				"put"     | "p"  => return Ok(CalcResult::Output(*self.env.read_var(ANSWER).unwrap())),
				"reverse" | "r"  => self.reverse = !self.reverse,
				"set"     | "s"  => {
					if elems.len() >= 3 {
						if let Err(_) = elems[1].parse::<f64>() { // we don't want the user redefining the value of a number!
							if let Ok(n) = self.evaluate_expression(&elems[2..]) {
								self.env.store(elems[1], n);
							}
						} else {
							return Err(RoseError::InvalidSyntax);
						}
					 } else {
					 	return Err(RoseError::StrangeArguments);
					 }
				}
				_                => return self.env.command(elems[0]),
			}
		}

		Ok(CalcResult::None)
	}

	// evaluate an expression

	fn evaluate_expression(&self, elems: &[&str]) -> Result<f64, RoseError> {
		if elems.len() == 1 { // only one element
			return self.env.check_value(elems[0])
		} else {
			let mut symbol = 0;
			let mut begin = 1;
			let mut end = elems.len();

			if self.reverse { // are we in RPN mode?
				symbol = elems.len() - 1; // set where we should check for the operator symbol
				
				begin = 0; // shift the location of values
				end = end - 1;
			}

			if let Ok(o)  = arithmetic::new_operator(elems[symbol]) {
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

							values.push(self.evaluate_expression(&sub_expr)?);

							sub_mode = false;
							sub_expr.clear();

							open = 1;
							closed = 0;
						} else {
							sub_expr.push(val)
						}
					} else {
						match self.env.check_value(val) {
							Ok(v)  => values.push(v),
							Err(_) => { // this could be a genuine error... or we could be using paranthesis.
								if val.starts_with("(") {
									sub_expr.push(&val.strip_prefix("(").unwrap());

									sub_mode = true;
								} else {
									return Err(RoseError::StrangeArguments);
								}
							}
						}
					}
				}
				
				return Ok(o.operate(&values)?.0)
			}
		}

		Err(RoseError::UnknownCommand)
	}
}
