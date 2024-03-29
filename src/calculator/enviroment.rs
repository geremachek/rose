use std::{collections::HashMap};
use super::{CalcResult, config::Config};
use crate::errors::RoseError;

// a common "enviroment" for calculators

#[derive(Default)]
pub struct Enviroment {
	pub silent: bool,
	pub format: bool,

	pub vars: HashMap<String, f64>,
	
	pub conf: Config,
}

impl Enviroment {
	// create a new enviroment instance

	pub fn new(s: bool, f: bool) -> Self {
		let variables =
		 	 [("pi".to_string(), std::f64::consts::PI),
			 ("tau".to_string(), std::f64::consts::PI*2.0),
			 ("e".to_string(), std::f64::consts::E)]
			.iter().cloned().collect();

		Self {
			silent: s,
			format: f,
			
			vars: variables,
			conf: Config::new(),
		}
	}

	// common commands for calculator enviroments
	
	pub fn command(&mut self, word: &str) -> Result<CalcResult, RoseError> {
		match word {
			"quit"    | "q" => return Ok(CalcResult::Quit),
			"silent"  | "s" => self.silent = !self.silent, // toggle silent mode
			"format"  | "f" => self.format = !self.format, // toggle formatting
			"memory"  | "m" | "mem" => {
				// output memory variables in a little "table"
			
				let mut msg = String::new();

				for (name, value) in &self.vars {
					msg.push_str(&format!("{}: {}\n", name, value));
				}

				return Ok(CalcResult::Message(msg));
			}
			_               => return Err(RoseError::UnknownCommand),
		}

		Ok(CalcResult::None)
	}

	// format the output based on enviroment variables

	pub fn output_result(&self, result: f64, silent: bool) {
		if !silent { // show nothing if in silent mode
			if self.format { // format 
				self.conf.format_result(result);
			} else {
				println!("{}", result);
			}
		}
	}

	// check to see if a value is valid against the enviroment

	pub fn check_value(&self, val: &str) -> Result<f64, RoseError> {
		let mut trimmed_val = val;
		let mut sign = 1.0; // by default the sign is positive

		// trim '-'

		if let Some(trim) = val.strip_prefix("-") {
			trimmed_val = trim;
			sign = -1.0;
		}

		match self.vars.get(trimmed_val) {
			Some(n) => Ok(*n*sign), // valid variable, multiply so we can preserve the possible negative value
			None    => val.parse::<f64>() // normal value
					.or(Err(RoseError::StrangeArguments))
		}
	}
}
