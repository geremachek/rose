use crate::{
	errors::RoseError,

	calculator::{
		Calculator,
		CalcResult,
		enviroment::Enviroment
	},
};

mod parse;

const ANSWER: &str = "$"; // the variable that stores the answer

#[derive(Default)]
pub struct Standard {
	env: Enviroment,
	reverse: bool
}

impl Standard {
	// create a new standard calculator

	pub fn new(s: bool, f: bool, r: bool) -> Self {
		let mut e = Enviroment::new(s, f);
		e.vars.insert(ANSWER.to_string(), 0.0); // add answer variable

		Self { env: e, reverse: r }	
	}
}

impl Calculator for Standard {
	// return our enviroment structure

	fn get_env(&self) -> &Enviroment {
		&self.env
	}

	// parse a line

	fn parse(&mut self, elems: &[&str]) -> Result<Vec<CalcResult>, RoseError> {
		Ok(vec![self.parser(elems)?])
	}
}
