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
	let (a, b) = (numbers[0], numbers[1]);
	let c: u32 = second_line.trim().parse().unwrap();

	let (ended_hour, ended_minute) = add_minute(a, b, c);
	print!("{} {}", ended_hour, ended_minute);
}

fn add_minute(hour: u32, minute: u32, to_add_minute: u32) -> (u32, u32) {
	let total_minutes = (hour * 60) + minute + to_add_minute;

	((total_minutes / 60) % 24, total_minutes % 60)
}

#[cfg(test)]
mod tests {
	use crate::add_minute;

	#[test]
	fn it_works() {
		assert_eq!((14, 50), add_minute(14, 30, 20));
		assert_eq!((19, 0), add_minute(17, 40, 80));
		assert_eq!((0, 13), add_minute(23, 48, 25));
	}
}
