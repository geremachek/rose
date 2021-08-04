use std::env;

// config structure

pub struct Config {
	pub prompt: String,

	fmt_prefix: String,
	fmt_postfix: String,
}

impl Config {
	// create a new Config structure, reading from enviroment variables

	pub fn new() -> Config {
		Config {
			prompt: env::var("ROSE_PROMPT")
				.unwrap_or_else(|_| "".to_string()),
			fmt_prefix: env::var("ROSE_FORMAT_PREFIX")
				.unwrap_or_else(|_| "  -> ".to_string()),
			fmt_postfix: env::var("ROSE_FORMAT_POSTFIX")
				.unwrap_or_else(|_| "".to_string()),
		}
	}

	// format a numerical result from the parser

	pub fn format_result(&self, result: f64) {
		println!("{}{}{}",
			&self.fmt_prefix,
			result,
			&self.fmt_postfix);
	}
}
