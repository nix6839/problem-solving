fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let n: u32 = line.trim().parse().unwrap();

	print!("{}", sum_1_to_n(n));
}

fn sum_1_to_n(n: u32) -> u32 {
	(1..=n).sum()
}

#[cfg(test)]
mod tests {
	use crate::sum_1_to_n;

	#[test]
	fn it_works() {
		assert_eq!(6, sum_1_to_n(3));
	}
}
