use std::env;

pub struct Config {
	pub prompt: String,

	pub fmt_prefix: String,
	pub fmt_postfix: String,
}

impl Config {
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
}
