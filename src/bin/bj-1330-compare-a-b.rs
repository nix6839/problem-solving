fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<i32> = line.split(' ').map(|n| n.trim().parse().unwrap()).collect();
	let (a, b) = (numbers[0], numbers[1]);

	print!("{}", get_a_b_compare_string(a, b));
}

fn get_a_b_compare_string(a: i32, b: i32) -> String {
	match a.cmp(&b) {
		std::cmp::Ordering::Less => "<".into(),
		std::cmp::Ordering::Equal => "==".into(),
		std::cmp::Ordering::Greater => ">".into(),
	}
}

#[cfg(test)]
mod tests {
	use crate::get_a_b_compare_string;

	#[test]
	fn it_works() {
		assert_eq!("<", get_a_b_compare_string(1, 2));
		assert_eq!(">", get_a_b_compare_string(10, 2));
		assert_eq!("==", get_a_b_compare_string(5, 5));
	}
}
