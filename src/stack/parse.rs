use super::Stack;

use crate::{
	errors::RoseError,
	arithmetic,
	calculator::CalcResult,
};

impl Stack {
	// parse a single element from our list of elements

	pub fn parse_element(&mut self, elem: &str) -> Result<CalcResult, RoseError> {
		if let Ok(v) = self.env.check_value(elem) { // normal value
			self.stack.push(v);
		} else if let Ok(o) = arithmetic::new_operator(elem) { // operator!
			if !self.stack.is_empty() { // our stack musn't be empty to operate
				let mut opr_vec = Vec::new();
				let len = self.stack.len();

				if len > 1 { // detach the last two variables
					opr_vec = self.stack[(len-2)..].to_vec();
				} else { // the the last variable
					opr_vec.push(self.stack[len - 1]);
				}

				let (v, n) = o.operate(&opr_vec)?; // operate!
				
				// remove the used values from the stack

				for _ in 0..n { // remove the values from the stack
					self.stack.pop();
				}

				// add the result to the stack and return it

				self.stack.push(v);
				return Ok(CalcResult::Answer(v));
			} else {
				return Err(RoseError::StackEmpty);
			}
		} else {
			match elem {
				"put"     | "p"         => return Ok(CalcResult::Output(*self.stack.last()
								.ok_or(RoseError::StackEmpty)?)), // last stack item
				"stack"   | "."         => return Ok(CalcResult::Message(self.show_stack())),
				"clear"   | "c"         => self.stack.clear(),
				"reverse" | "rev" | "r" => self.stack.reverse(),
				"twirl"   | "t"         => self.twirl(),
				"pop"     | "P"         => { self.stack.pop(); }
				_                       => return self.env.command(elem), // send to the enviroment command parser
			}
		}

		Ok(CalcResult::None) // nothing to handle by default
	}
}
