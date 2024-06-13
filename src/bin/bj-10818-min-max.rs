fn main() {
	let mut numbers_count = String::new();
	std::io::stdin().read_line(&mut numbers_count).unwrap();
	let numbers_count = numbers_count.trim().parse().unwrap();

	let mut numbers = String::new();
	std::io::stdin().read_line(&mut numbers).unwrap();
	let numbers: Vec<i32> = numbers
		.trim()
		.split(' ')
		.take(numbers_count)
		.map(|s| s.parse().unwrap())
		.collect();

	let (min, max) = find_min_and_max(&numbers).unwrap();
	print!("{min} {max}");
}

#[cfg(feature = "solution1")]
fn find_min_and_max(numbers: &[i32]) -> Option<(i32, i32)> {
	if numbers.is_empty() {
		return None;
	}
	Some(
		numbers
			.iter()
			.fold((numbers[0], numbers[0]), |(min, max), &n| {
				(n.min(min), n.max(max))
			}),
	)
}

// Best solution
#[cfg(feature = "solution2")]
fn find_min_and_max(numbers: &[i32]) -> Option<(i32, i32)> {
	match (numbers.iter().min(), numbers.iter().max()) {
		(Some(&min), Some(&max)) => Some((min, max)),
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use crate::find_min_and_max;

	#[test]
	fn it_works() {
		assert_eq!(Some((7, 35)), find_min_and_max(&[20, 10, 35, 30, 7]));
	}
}
