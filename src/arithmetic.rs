use crate::errors::RoseError;
use std::convert::TryFrom;

// enum for basic mathematical operators

enum OpBasic {
	Addition,
	Subtraction,
	Multiplication,
	Division,
	Power,
	Modulus
}

// enum for mathematical functions

enum OpFunction {
	Root,
	Factorial,

	Logarithm,
	Ln,

	Sine,
	Cosine,
	Tangent,

	Arcsine,
	Arccosine,
	Arctangent,
}

// operator trait

pub trait Operator {
	// calculate an answer based on the operator and a string of values, also return the amount of values used

	fn operate(&self, nums: &[f64]) -> Result<(f64, usize), RoseError>;
}

// create a basic, or function operator from a symbol

pub fn new_operator(symb: &str) -> Result<Box<dyn Operator>, RoseError> {
	match OpBasic::try_from(symb) {
		Ok(b)  => Ok(Box::new(b)),
		_      => Ok(Box::new(OpFunction::try_from(symb)?))
	}
}

impl TryFrom<&str> for OpBasic {
	type Error = RoseError;

	fn try_from(symbol: &str) -> Result<Self, Self::Error> {
		match symbol.to_lowercase().as_str() {
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

			"%"          => Ok(OpBasic::Modulus),
			"mod"        => Ok(OpBasic::Modulus),
			"modulus"    => Ok(OpBasic::Modulus),

			_            => Err(RoseError::UnknownCommand),
		}
	}
}

impl Operator for OpBasic {
	fn operate(&self, nums: &[f64]) -> Result<(f64, usize), RoseError> {
		// if the values feild is empty, return an error, otherwise calculate.
		
		let mut result = *nums.get(0).ok_or(RoseError::InvalidSyntax)?;

		// loop through the elements and calculate a result

		for i in nums[1..].iter() {
			match self {
				OpBasic::Addition       => result += i,
				OpBasic::Subtraction    => result -= i,
				OpBasic::Multiplication => result *= i,
				OpBasic::Division       => {
					// throw an error if we divide by zero

					if i != &0.0 {
						result /= i
					} else {
						return Err(RoseError::StrangeArguments)
					}
				}
				OpBasic::Power          => result = result.powf(*i),
				OpBasic::Modulus        => result %= i
			}
		}

		Ok((result, nums.len()))
	}
}

impl TryFrom<&str> for OpFunction {
	type Error = RoseError;

	fn try_from(symbol: &str) -> Result<Self, Self::Error> {
		match symbol.to_lowercase().as_str() {
			"√"          => Ok(OpFunction::Root),
			"radical"    => Ok(OpFunction::Root),
			"root"       => Ok(OpFunction::Root),

			"!"          => Ok(OpFunction::Factorial),
			"fact"       => Ok(OpFunction::Factorial),
			"factorial"  => Ok(OpFunction::Factorial),

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
			
			_            => Err(RoseError::UnknownCommand),
		}
	}
}

impl Operator for OpFunction {
	fn operate(&self, nums: &[f64]) -> Result<(f64, usize), RoseError> {
		match self {
			OpFunction::Root =>
				// in order to calculate a root, there must be two arguments

				match nums.len()  {
					1     => Ok((nums[0].powf(0.5), 1)),
					2 | _ => Ok((nums[0].powf(1.0/nums[1]), 2)),

				}
			OpFunction::Factorial => Ok((factorial(nums[0] as u64) as f64, 1)),
			OpFunction::Logarithm =>
				match nums.len() {
					1     => Ok((nums[0].log10(), 1)),
					2 | _ => Ok((nums[1].log(nums[0]), 2)),
				}

			OpFunction::Ln          => Ok((nums[0].ln(), 1)),
			
			OpFunction::Sine        => Ok((nums[0].sin(), 1)),
			OpFunction::Cosine      => Ok((nums[0].cos(), 1)),
			OpFunction::Tangent     => Ok((nums[0].tan(), 1)),
			
			OpFunction::Arcsine     => Ok((nums[0].asin(), 1)),
			OpFunction::Arccosine   => Ok((nums[0].acos(), 1)),
			OpFunction::Arctangent  => Ok((nums[0].atan(), 1)),
		}
	}
}

fn factorial(n: u64) -> u64 {
	match n {
		0 | 1 => 1,
		_     => n * factorial(n - 1),
	}
}
