fn main() {
	let mut n_count = String::new();
	std::io::stdin().read_line(&mut n_count).unwrap();
	let n_count = n_count.trim().parse().unwrap();

	let mut numbers = String::new();
	std::io::stdin().read_line(&mut numbers).unwrap();
	let numbers: Vec<i32> = numbers
		.trim()
		.split(' ')
		.take(n_count)
		.map(|s| s.parse().unwrap())
		.collect();

	let mut v = String::new();
	std::io::stdin().read_line(&mut v).unwrap();
	let v = v.trim().parse().unwrap();

	print!("{}", count_v_from_slice(v, &numbers));
}

fn count_v_from_slice(v: i32, slice: &[i32]) -> usize {
	slice.iter().filter(|n| **n == v).count()
}

#[cfg(test)]
mod tests {
	use crate::count_v_from_slice;

	#[test]
	fn it_works() {
		assert_eq!(3, count_v_from_slice(2, &[1, 4, 1, 2, 4, 2, 4, 2, 3, 4, 4]));
		assert_eq!(0, count_v_from_slice(5, &[1, 4, 1, 2, 4, 2, 4, 2, 3, 4, 4]));
	}
}
