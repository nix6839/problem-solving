fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<u32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (h, m) = (numbers[0], numbers[1]);

	let (h, m) = subtract_45_minute(h, m);
	print!("{h} {m}");
}

#[cfg(feature = "solution1")]
fn subtract_45_minute(hour: u32, minute: u32) -> (u32, u32) {
	if let Some(minute) = minute.checked_sub(45) {
		(hour, minute)
	} else {
		let hour = if hour == 0 { 23 } else { hour - 1 };
		(hour, (60 + minute) - 45)
	}
}

#[cfg(feature = "solution2")]
fn subtract_45_minute(hour: u32, minute: u32) -> (u32, u32) {
	let total_minute = (hour * 60) + minute;
	if let Some(total_minute) = total_minute.checked_sub(45) {
		(total_minute / 60, total_minute % 60)
	} else {
		(23, (60 + minute) - 45)
	}
}

// Best solution
#[cfg(feature = "solution3")]
fn subtract_45_minute(hour: u32, minute: u32) -> (u32, u32) {
	// 00:00 아래로 내려가면 23:59으로 내려감. 이를 위해 `(24 * 60)`으로 시간에 하루를 추가.
	let total_minute = (24 * 60) + (hour * 60) + minute - 45;
	// 만약 00:00 아래로 내려가지 않았을 경우를 위해 24로 나머지 연산을 해 하루 단위를 제거.
	((total_minute / 60) % 24, total_minute % 60)
}

#[cfg(test)]
mod tests {
	use crate::subtract_45_minute;

	#[test]
	fn it_works() {
		assert_eq!((9, 25), subtract_45_minute(10, 10));
		assert_eq!((23, 45), subtract_45_minute(0, 30));
		assert_eq!((22, 55), subtract_45_minute(23, 40));
	}
}
