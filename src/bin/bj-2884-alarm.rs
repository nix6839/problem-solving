fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<u32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (h, m) = (numbers[0], numbers[1]);

	let (h, m) = minus_45_minute(h, m);
	print!("{} {}", h, m);
}

#[cfg(feature = "solution1")]
fn minus_45_minute(hour: u32, minute: u32) -> (u32, u32) {
	if let Some(minute) = minute.checked_sub(45) {
		(hour, minute)
	} else {
		let hour = if hour == 0 { 23 } else { hour - 1 };
		(hour, (60 + minute) - 45)
	}
}

#[cfg(feature = "solution2")]
fn minus_45_minute(hour: u32, minute: u32) -> (u32, u32) {
	let total_minute = (hour * 60) + minute;
	if let Some(total_minute) = total_minute.checked_sub(45) {
		(total_minute / 60, total_minute % 60)
	} else {
		(23, (60 + minute) - 45)
	}
}

#[cfg(test)]
mod tests {
	use crate::minus_45_minute;

	#[test]
	fn it_works() {
		assert_eq!((9, 25), minus_45_minute(10, 10));
		assert_eq!((23, 45), minus_45_minute(0, 30));
		assert_eq!((22, 55), minus_45_minute(23, 40));
	}
}
