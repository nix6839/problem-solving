fn main() {
	let numbers: Vec<u32> = std::io::stdin()
		.lines()
		.take(9)
		.map(|line| line.unwrap().parse().unwrap())
		.collect();

	let (max, position) = find_max_and_position(&numbers).unwrap();
	print!("{max}\n{position}");
}

fn find_max_and_position(numbers: &[u32]) -> Option<(u32, usize)> {
	if numbers.is_empty() {
		return None;
	}
	let mut max = numbers[0];
	let mut max_index = 0;
	for (i, &n) in numbers.iter().enumerate() {
		if n > max {
			(max, max_index) = (n, i);
		}
	}
	Some((max, max_index + 1))
}

#[cfg(test)]
mod tests {
	use crate::find_max_and_position;

	#[test]
	fn it_works() {
		assert_eq!(
			Some((85, 8)),
			find_max_and_position(&[3, 29, 38, 12, 57, 74, 40, 85, 61,])
		);
	}
}
