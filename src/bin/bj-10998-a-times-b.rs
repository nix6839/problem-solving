fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<i32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (a, b) = (numbers[0], numbers[1]);

	print!("{}", a_times_b(a, b));
}

fn a_times_b(a: i32, b: i32) -> i32 {
	a * b
}

#[cfg(test)]
mod tests {
	use crate::a_times_b;

	#[test]
	fn it_works() {
		assert_eq!(2, a_times_b(1, 2));
		assert_eq!(12, a_times_b(3, 4));
		assert_eq!(72, a_times_b(9, 8));
	}
}
