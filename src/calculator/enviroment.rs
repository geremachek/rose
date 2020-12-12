use std::{collections::HashMap};
use super::{CalcResult, config::Config};
use crate::errors::RoseError;

// a common "enviroment" for calculators

pub struct Enviroment {
	pub silent: bool,
	pub format: bool,

	pub vars: HashMap<String, f64>,
	
	pub conf: Config,
}

impl Enviroment {
	// create a new enviroment instance

	pub fn new(s: bool, f: bool) -> Enviroment {
		let variables =
			[("π".to_string(), std::f64::consts::PI),
		 	 ("pi".to_string(), std::f64::consts::PI),
			 ("τ".to_string(), std::f64::consts::PI*2.0),
			 ("tau".to_string(), std::f64::consts::PI*2.0),
			 ("e".to_string(), std::f64::consts::E)]
			.iter().cloned().collect();

		Enviroment {
			silent: s,
			format: f,
			
			vars: variables,
			conf: Config::new(),
		}
	}

	// common commands for calculator enviroments
	
	pub fn command(&mut self, word: &str) -> CalcResult {
		match word {
			"quit"    | "q" => return CalcResult::Quit,
			"silent"  | "s" => self.silent = !self.silent,
			"format"  | "f" => self.format = !self.format,
			"memory"  | "m" | "mem" => {
				let mut msg = String::new();

				for (name, value) in &self.vars {
					msg.push_str(&format!("{}: {}\n", name, value));
				}

				return CalcResult::Message(msg);
			}
			_               => return CalcResult::Error(RoseError::UnknownCommand),
		}

		CalcResult::None
	}

	// format the output based on enviroment variables

	pub fn output_result(&self, result: f64, silent: bool) {
		if !silent {
			if self.format {
				println!("{}{}{}",
					&self.conf.fmt_prefix,
					result,
					&self.conf.fmt_postfix);
			}
		}
	}

	// check to see if a value is valid against the enviroment

	pub fn check_value(&self, val: &str) -> Result<f64, std::num::ParseFloatError> {
		let mut trimmed_val = val;
		let mut sign = 1.0; // by default the sign is positive

		if val.starts_with("-") {
			trimmed_val = &val[1..];
			sign = -1.0; // sign is negative
		}

		match self.vars.get(trimmed_val) {
			Some(n) => Ok(*n*sign),
			None    => val.parse::<f64>(),
		}
	}
}

