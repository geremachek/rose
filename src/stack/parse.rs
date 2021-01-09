use super::Stack;
use crate::{errors::RoseError, {arithmetic, calculator::CalcResult}};

impl Stack {
	// parse a single element from our list of elements

	pub fn parse_element(&mut self, elem: &str) -> Result<CalcResult, RoseError> {
		if let Ok(v) = self.env.check_value(elem) {
			self.stack.push(v);
		} else if let Ok(o) = arithmetic::new_operator(elem) {
			if !self.stack.is_empty() {
				let mut opr_vec = Vec::new();
				let len = self.stack.len();

				if len > 1 {
					opr_vec = self.stack[(len-2)..].to_vec();
				} else {
					opr_vec.push(self.stack[len - 1]);
				}

				if let Ok((v, n)) = o.operate(&opr_vec) {
					// remove the used values from the stack

					for _ in 0..n {
						self.stack.pop();
					}

					// add the result to the stack and return it

					self.stack.push(v);
					return Ok(CalcResult::Answer(v));
				}
			}
		} else {
			match elem {
				"put"     | "p"         => return Ok(CalcResult::Output(*self.stack.last()
								.unwrap_or(&0.0))),
				"stack"   | "S"         => return Ok(CalcResult::Message(self.show_stack())),
				"clear"   | "c"         => self.stack.clear(),
				"reverse" | "rev" | "r" => self.reverse_stack(),
				"twirl"   | "t"         => self.twirl(),
				"pop"     | "P"         => { self.stack.pop(); }
				_                       => return self.env.command(elem),
			}
		}

		Ok(CalcResult::None)
	}
}
