use crate::{errors::RoseError, calculator::{Calculator, CalcResult, enviroment::Enviroment}};

mod parse;

const ANSWER: &str = "$"; // the variable that stores the answer

pub struct Standard {
	pub env: Enviroment,
	pub reverse: bool
}

impl Calculator for Standard {
	// create a new standard calculator

	fn new(s: bool, f: bool) -> Standard {
		let mut e = Enviroment::new(s, f);
		e.vars.insert(ANSWER.to_string(), 0.0);

		Standard { env: e, reverse: false }	
	}

	// return our enviroment structure

	fn get_env(&self) -> &Enviroment {
		&self.env
	}

	// parse a line

	fn parse(&mut self, elems: Vec<&str>) -> Vec<CalcResult> {
		let mut to_handle = Vec::new();

		if let Ok(v) = self.evaluate_expression(&elems) {
			self.env.vars.insert(ANSWER.to_string(), v);
			to_handle.push(CalcResult::Answer(v));
		} else {
			let mut locales = (0, 1, 2); // location of the various elements of the "set" command

			// set x 0 | x 0 set
				
			if self.reverse {
				locales = (elems.len() - 1, 0, 1);
			}

			match elems[locales.0] {
				"put"     | "p"       => to_handle.push(CalcResult::Output(*self.env.vars.get(ANSWER).unwrap())),
				"reverse" | "r"       => self.reverse = !self.reverse,
				"set"     | "s" | "=" => {
					if elems.len() == 3 {
						if let Err(_) = elems[locales.1].parse::<f64>() { // we don't want the user redefining the value of a number!
							if let Ok(n) = self.env.check_value(elems[locales.2]) {
								self.env.vars.insert(elems[locales.1].to_string(), n);
							} else {
								to_handle.push(CalcResult::Error(RoseError::StrangeArguments));
							}
						} else {
							to_handle.push(CalcResult::Error(RoseError::InvalidSyntax));
						}
					 } else {
					 	to_handle.push(CalcResult::Error(RoseError::StrangeArguments));
					 }
				}
				_                 => to_handle.push(self.env.command(elems[locales.0])),
			}
		}

		to_handle
	}
}
