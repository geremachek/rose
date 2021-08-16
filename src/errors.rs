use std::fmt;

#[derive(Debug)]
pub enum RoseError {
	UnknownCommand,
	InvalidSyntax,
	StrangeArguments,
	Overflow,
	StackEmpty,
}

impl fmt::Display for RoseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let msg = match self {
			RoseError::UnknownCommand   => "unknown / out of place command",
			RoseError::InvalidSyntax    => "invalid syntax",
			RoseError::StrangeArguments => "strange / malformed arguments",
			RoseError::Overflow         => "float overflow",
			RoseError::StackEmpty       => "stack is emtpy"
		};

		write!(f, "rose: {}", msg)
	}
}
