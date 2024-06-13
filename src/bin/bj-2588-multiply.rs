fn main() {
	let numbers: Vec<u32> = std::io::stdin()
		.lines()
		.take(2)
		.map(|line| line.unwrap().parse().unwrap())
		.collect();
	let (n1, n2) = (numbers[0], numbers[1]);
	for n in get_3_4_5_6(n1, n2) {
		println!("{n}");
	}
}

fn get_3_4_5_6(n1: u32, n2: u32) -> [u32; 4] {
	let n2_digits = (n2 % 10, (n2 % 100) / 10, n2 / 100);
	[
		n1 * n2_digits.0,
		n1 * n2_digits.1,
		n1 * n2_digits.2,
		n1 * n2,
	]
}

#[cfg(test)]
mod tests {
	use crate::get_3_4_5_6;

	#[test]
	fn it_works() {
		assert_eq!([2360, 3776, 1416, 181_720], get_3_4_5_6(472, 385));
	}
}
