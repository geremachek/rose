use crate::{errors::RoseError, calculator::{Calculator, CalcResult, enviroment::Enviroment}};

mod parse;

const ANSWER: &str = "$"; // the variable that stores the answer

pub struct Standard {
	env: Enviroment,
	reverse: bool
}

impl Standard {
	// create a new standard calculator

	pub fn new(s: bool, f: bool, r: bool) -> Standard {
		let mut e = Enviroment::new(s, f);
		e.store(ANSWER, 0.0);

		Standard { env: e, reverse: r }	
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
