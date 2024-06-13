fn main() {
	let mut byte_size = String::new();
	std::io::stdin().read_line(&mut byte_size).unwrap();
	let byte_size: usize = byte_size.trim().parse().unwrap();

	println!("{}", get_type_name(byte_size));
}

fn get_type_name(byte_size: usize) -> String {
	format!("{}int", "long ".repeat(byte_size / 4))
}

#[cfg(test)]
mod tests {
	use crate::get_type_name;

	#[test]
	fn it_works() {
		assert_eq!("long int", get_type_name(4));
		assert_eq!("long long long long long int", get_type_name(20));
	}
}
