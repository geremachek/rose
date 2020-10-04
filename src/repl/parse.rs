use super::arithmetic;
use crate::repl;

impl repl::Repl {
	// parse a line

	pub fn parse(&mut self, line: &str) -> ReplResult {
		let mut result = ReplResult::None;

		if line != "\n" && !line.trim().is_empty() {
			let split:Vec<&str> = line.split_whitespace().collect(); // create a vector of values

			if let Some(o) = arithmetic::Operator::from(split[0]) { // the command a valid operator
				let replaced = self.subsitute_vars(split);

				match arithmetic::Numbers::from(&replaced[1..]) { // are we able to to convert the values?
					Ok(n) => // if so, 
						match n.operate(o) { // are there any values actually there?
							Some(v) => {
								self.vars.insert(String::from("$"), v); // update the result value
								result =  ReplResult::Answer(v);
							}
							None    => result = ReplResult::Error,
						}

					Err(_) => result = ReplResult::Error,
				}
			} else {
				match split[0] {
					"quit"   | "q" => result = ReplResult::Quit,
					"put"    | "p" => result = ReplResult::Output(*self.vars.get("$").unwrap()),
					"set"    | "=" =>
						if split.len() == 3 {
							if let Some(n) = self.vars.clone().get(split[2]) { // is the value a variable?
								self.vars.insert(split[1].to_string(), *n); // it is! We set the variable.
							} else {
								if let Ok(n) = split[2].parse::<f64>() {
									self.vars.insert(split[1].to_string(), n);
								} else { result = ReplResult::Error; }
							}
						} else {
							result = ReplResult::Error;
						}
					"silent" | "s" => self.silent = !self.silent,
					"format" | "f" => self.format = !self.format,
					"memory" | "m" =>
						for (name, value) in &self.vars {
							println!("{} => {}", name, value);
						}
					_ => {
						match self.vars.get(split[0]) {
							Some(n) => result = ReplResult::Output(*n),
							None    => result = ReplResult::Error,
						}
					}
				}
			}
		}

		result
	} 

	// subsitute variables in a vector of strings

	fn subsitute_vars(&mut self, vals: Vec<&str>) -> Vec<String> {
		let mut updated = Vec::new();

		for value in vals { // loop through the vector of string values

			match self.vars.get(&value.to_string()) { // is the value a proper variable?
				Some(n) => updated.push(n.to_string()),
				None => updated.push(value.to_string())
			}
		}

		updated
	}
}

pub enum ReplResult {
	Answer(f64),
	Output(f64),
	Error,
	Quit,
	None,
}
