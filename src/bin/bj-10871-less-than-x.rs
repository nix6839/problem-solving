fn main() {
	let mut numbers_count_and_x = String::new();
	std::io::stdin()
		.read_line(&mut numbers_count_and_x)
		.unwrap();
	let mut numbers_count_and_x = numbers_count_and_x.trim().split(' ');
	let numbers_count = numbers_count_and_x.next().unwrap().parse().unwrap();
	let x = numbers_count_and_x.next().unwrap().parse().unwrap();

	let mut numbers = String::new();
	std::io::stdin().read_line(&mut numbers).unwrap();
	let numbers: Vec<u32> = numbers
		.trim()
		.split(' ')
		.take(numbers_count)
		.map(|s| s.parse().unwrap())
		.collect();

	print!(
		"{}",
		find_rather_than_x(x, &numbers)
			.iter()
			.map(std::string::ToString::to_string)
			.collect::<Vec<_>>()
			.join(" ")
	);
}

fn find_rather_than_x(x: u32, slice: &[u32]) -> Vec<u32> {
	slice.iter().filter(|&&n| n < x).copied().collect()
}

#[cfg(test)]
mod tests {
	use crate::find_rather_than_x;

	#[test]
	fn it_works() {
		assert_eq!(
			[1, 4, 2, 3],
			&find_rather_than_x(5, &[1, 10, 4, 9, 2, 3, 8, 5, 7, 6])[..]
		);
	}
}
