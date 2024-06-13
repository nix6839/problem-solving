fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<i32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (a, b) = (numbers[0], numbers[1]);

	print!("{}", a_plus_b(a, b));
}

fn a_plus_b(a: i32, b: i32) -> i32 {
	a + b
}

#[cfg(test)]
mod tests {
	use crate::a_plus_b;

	#[test]
	fn it_works() {
		assert_eq!(3, a_plus_b(1, 2));
		assert_eq!(5, a_plus_b(2, 3));
		assert_eq!(18, a_plus_b(9, 9));
	}
}
