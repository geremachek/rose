pub enum Operator {
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Power,
	Root,
}

impl Operator {
	// create a new operator based on a symbol

	pub fn from(symb: &str) -> Option<Operator> {
		match symb.to_lowercase().as_str() {
			"+"          => Some(Operator::Addition),
			"plus"       => Some(Operator::Addition),
			"add"        => Some(Operator::Addition),
			"sum"        => Some(Operator::Addition),

			"-"          => Some(Operator::Subtraction),
			"minus"      => Some(Operator::Subtraction),
			"subtract"   => Some(Operator::Subtraction),
			"difference" => Some(Operator::Subtraction),
			"diff"       => Some(Operator::Subtraction),

			"*"          => Some(Operator::Multiplication),
			"✕"          => Some(Operator::Multiplication),
			"⋅"          => Some(Operator::Multiplication),
			"multiply"   => Some(Operator::Multiplication),
			"product"    => Some(Operator::Multiplication),

			"/"          => Some(Operator::Division),
			"÷"          => Some(Operator::Division),
			"divide"     => Some(Operator::Division),
			"quotient"   => Some(Operator::Division),

			"^"          => Some(Operator::Power),
			"pow"        => Some(Operator::Power),
			"power"      => Some(Operator::Power),

			"√"          => Some(Operator::Root),
			"radical"    => Some(Operator::Root),
			"root"       => Some(Operator::Root),
			_            => None,
		}
	}
}

pub struct Numbers {
	pub values: Vec<f64>,
}

impl Numbers {
	// create a Numbers object from a list of strings

	pub fn from(nums: Vec<f64>) -> Numbers {
		Numbers { values: nums }
	}

	// operate on our values

	pub fn operate(&self, o: Operator) -> Option<f64> {
		let mut result: f64;

		// if the values feild is empty, return None, otherwise calculate.

		result = *self.values.get(0)?;

		// loop through the elements and calculate a result

		for i in self.values[1..].iter() {
			result = match o {
				Operator::Addition       => result + i,
				Operator::Subtraction    => result - i,
				Operator::Multiplication => result * i,
				Operator::Division       => result / i,
				Operator::Power          => result.powf(*i),
				Operator::Root           => result.powf(1.0/(*i)),
			}
		}

		Some(result)
	}
}
