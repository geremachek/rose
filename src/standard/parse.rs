use crate::{arithmetic, standard, errors::RoseError};

impl standard::Standard {
	// evaluate an expression

	pub fn evaluate_expression(&self, elems: &Vec<&str>) -> Result<f64, RoseError> {
		if elems.len() == 1 { // only one element
			return match self.env.check_value(elems[0]) {
				Ok(v)  => Ok(v),
				Err(_) => Err(RoseError::StrangeArguments),
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

			if let Ok(o)  = arithmetic::new_operator(symbol) {
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
				
				return match o.operate(&values) {
					Ok((v, _)) => Ok(v),
					Err(e)     => Err(e),
				}
			}
		}

		Err(RoseError::UnknownCommand)
	}
}
