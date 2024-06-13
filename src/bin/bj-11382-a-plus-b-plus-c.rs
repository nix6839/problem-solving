fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<u64> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (a, b, c) = (numbers[0], numbers[1], numbers[2]);

	print!("{}", a_plus_b_plus_c(a, b, c));
}

fn a_plus_b_plus_c(a: u64, b: u64, c: u64) -> u64 {
	a + b + c
}

#[cfg(test)]
mod tests {
	use crate::a_plus_b_plus_c;

	#[test]
	fn it_works() {
		assert_eq!(7931, a_plus_b_plus_c(77, 77, 7777));
		assert_eq!(
			3_000_000_000_000,
			a_plus_b_plus_c(10_u64.pow(12), 10_u64.pow(12), 10_u64.pow(12))
		);
	}
}
