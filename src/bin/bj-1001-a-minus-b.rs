fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<i32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (a, b) = (numbers[0], numbers[1]);

	print!("{}", a_minus_b(a, b));
}

fn a_minus_b(a: i32, b: i32) -> i32 {
	a - b
}

#[cfg(test)]
mod tests {
	use crate::a_minus_b;

	#[test]
	fn it_works() {
		assert_eq!(1, a_minus_b(3, 2));
		assert_eq!(8, a_minus_b(9, 1));
		assert_eq!(3, a_minus_b(5, 2));
	}
}
