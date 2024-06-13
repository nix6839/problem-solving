fn main() {
	let mut first_line = String::new();
	std::io::stdin().read_line(&mut first_line).unwrap();
	let mut second_line = String::new();
	std::io::stdin().read_line(&mut second_line).unwrap();

	let numbers: Vec<u32> = first_line
		.trim()
		.split(' ')
		.map(|n| n.parse().unwrap())
		.collect();
	let (n1, n2, n3) = (numbers[0], numbers[1], numbers[2]);

	print!("{}", calc_prize(n1, n2, n3));
}

fn calc_prize(n1: u32, n2: u32, n3: u32) -> u32 {
	if n1 == n2 && n2 == n3 {
		10_000 + n1 * 1_000
	} else if n1 != n2 && n1 != n3 && n2 != n3 {
		*[n1, n2, n3].iter().max().unwrap() * 100
	} else {
		let same_n = if n1 == n2 || n1 == n3 { n1 } else { n2 };
		1_000 + same_n * 100
	}
}

#[cfg(test)]
mod tests {
	use crate::calc_prize;

	#[test]
	fn it_works() {
		assert_eq!(1300, calc_prize(3, 3, 6));
		assert_eq!(12000, calc_prize(2, 2, 2));
		assert_eq!(600, calc_prize(6, 2, 5));
	}
}
