fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let id = line.trim();

	print!("{}", add_shock_suffix_to_id(id));
}

const SHOCK_STRING: &str = "??!";
fn add_shock_suffix_to_id<S: Into<String>>(id: S) -> String {
	format!("{}{SHOCK_STRING}", id.into())
}

#[cfg(test)]
mod tests {
	use crate::{add_shock_suffix_to_id, SHOCK_STRING};

	#[test]
	fn it_works() {
		assert_eq!(
			format!("joonas{SHOCK_STRING}"),
			add_shock_suffix_to_id("joonas")
		);
		assert_eq!(
			format!("baekjoon{SHOCK_STRING}"),
			add_shock_suffix_to_id("baekjoon")
		);
		assert_eq!(format!("me{SHOCK_STRING}"), add_shock_suffix_to_id("me"));
	}
}
