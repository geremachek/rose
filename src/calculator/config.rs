use std::env;

pub struct Config {
	pub prompt: String,

	fmt_prefix: String,
	fmt_postfix: String,
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

	pub fn format_result(&self, result: f64) {
		println!("{}{}{}",
			&self.fmt_prefix,
			result,
			&self.fmt_postfix);
	}
}
