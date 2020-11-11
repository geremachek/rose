// enum for basic mathematical operators

enum OpBasic {
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Power,
}

// enum for mathematical functions

enum OpFunction {
	Root,

	Logarithm,
	Ln,

	Sine,
	Cosine,
	Tangent,

	Arcsine,
	Arccosine,
	Arctangent,
}

// operator errors

pub enum OperatorError {
	BadSyntax,
	InvalidSymbol,
}

// operator trait

pub trait Operator {
	// create an operator from a symbol

	fn new(symb: &str) -> Result<Self, OperatorError> where Self: std::marker::Sized;
	
	// calculate an answer based on the operator and a string of values

	fn operate(&self, nums: Vec<f64>) -> Result<f64, OperatorError>;
}

// create a basic, or function operator from a symbol

pub fn new_operator(symb: &str) -> Result<Box<dyn Operator>, OperatorError> {
	match OpBasic::new(symb) {
		Ok(b)  => Ok(Box::new(b)),
		Err(_) =>
			match OpFunction::new(symb) {
				Ok(f)  => Ok(Box::new(f)),
				Err(e) => Err(e),
			}
	}

}

impl Operator for OpBasic {
	fn new(symb: &str) -> Result<OpBasic, OperatorError> {
		match symb.to_lowercase().as_str() {
			"+"          => Ok(OpBasic::Addition),
			"plus"       => Ok(OpBasic::Addition),
			"add"        => Ok(OpBasic::Addition),
			"sum"        => Ok(OpBasic::Addition),

			"-"          => Ok(OpBasic::Subtraction),
			"minus"      => Ok(OpBasic::Subtraction),
			"subtract"   => Ok(OpBasic::Subtraction),
			"difference" => Ok(OpBasic::Subtraction),
			"diff"       => Ok(OpBasic::Subtraction),

			"*"          => Ok(OpBasic::Multiplication),
			"✕"          => Ok(OpBasic::Multiplication),
			"⋅"          => Ok(OpBasic::Multiplication),
			"multiply"   => Ok(OpBasic::Multiplication),
			"product"    => Ok(OpBasic::Multiplication),

			"/"          => Ok(OpBasic::Division),
			"÷"          => Ok(OpBasic::Division),
			"divide"     => Ok(OpBasic::Division),
			"quotient"   => Ok(OpBasic::Division),

			"^"          => Ok(OpBasic::Power),
			"pow"        => Ok(OpBasic::Power),
			"power"      => Ok(OpBasic::Power),

			_            => Err(OperatorError::InvalidSymbol),
		}
	}

	fn operate(&self, nums: Vec<f64>) -> Result<f64, OperatorError> {
		// if the values feild is empty, return None, otherwise calculate.

		let mut result = *nums.get(0).ok_or_else(|| OperatorError::BadSyntax)?;

		// loop through the elements and calculate a result

		for i in nums[1..].iter() {
			result = match self {
				OpBasic::Addition       => result + i,
				OpBasic::Subtraction    => result - i,
				OpBasic::Multiplication => result * i,
				OpBasic::Division       => result / i,
				OpBasic::Power          => result.powf(*i),
			}
		}

		Ok(result)
	}
}

impl Operator for OpFunction {
	fn new(symb: &str) -> Result<OpFunction, OperatorError> {
		match symb.to_lowercase().as_str() {
			"√"          => Ok(OpFunction::Root),
			"radical"    => Ok(OpFunction::Root),
			"root"       => Ok(OpFunction::Root),

			"logarithm"  => Ok(OpFunction::Logarithm),
			"log"        => Ok(OpFunction::Logarithm),

			"ln"         => Ok(OpFunction::Ln),

			"sine"       => Ok(OpFunction::Sine),
			"sin"        => Ok(OpFunction::Sine),

			"cosine"     => Ok(OpFunction::Cosine),
			"cos"        => Ok(OpFunction::Cosine),

			"tangent"    => Ok(OpFunction::Tangent),
			"tan"        => Ok(OpFunction::Tangent),

			"arcsine"    => Ok(OpFunction::Arcsine),
			"arcsin"     => Ok(OpFunction::Arcsine),
			"asin"       => Ok(OpFunction::Arcsine),

			"arccosine"  => Ok(OpFunction::Arccosine),
			"acos"       => Ok(OpFunction::Arccosine),

			"arctangent" => Ok(OpFunction::Arctangent),
			"atan"       => Ok(OpFunction::Arctangent),
			
			_            => Err(OperatorError::InvalidSymbol),
		}
	}

	fn operate(&self, nums: Vec<f64>) -> Result<f64, OperatorError> {
		let err = Err(OperatorError::BadSyntax);

		if nums.len() == 0 {
			return err
		}

		match self {
			OpFunction::Root =>
				// in order to calculate a root, there must be two arguments

				match nums.len()  {
					2 => Ok(nums[0].powf(1.0/nums[1])),
					1 => Ok(nums[0].powf(0.5)),
					_ => err,
				}
			OpFunction::Logarithm =>
				match nums.len() {
					2 => Ok(nums[1].log(nums[0])),
					1 => Ok(nums[0].log10()),
					_ => err
				}

			OpFunction::Ln          => Ok(nums[0].ln()),
			
			OpFunction::Sine        => Ok(nums[0].sin()),
			OpFunction::Cosine      => Ok(nums[0].cos()),
			OpFunction::Tangent     => Ok(nums[0].tan()),
			
			OpFunction::Arcsine     => Ok(nums[0].asin()),
			OpFunction::Arccosine   => Ok(nums[0].acos()),
			OpFunction::Arctangent  => Ok(nums[0].atan()),
		}
	}
}
