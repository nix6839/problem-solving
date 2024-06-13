fn main() {
	let numbers: Vec<i32> = std::io::stdin()
		.lines()
		.take(2)
		.map(|line| line.unwrap().parse().unwrap())
		.collect();
	let (x, y) = (numbers[0], numbers[1]);

	println!("{}", get_quadrant(x, y).unwrap());
}

fn get_quadrant(x: i32, y: i32) -> Option<u32> {
	if x > 0 && y > 0 {
		Some(1)
	} else if x < 0 && y > 0 {
		Some(2)
	} else if x < 0 && y < 0 {
		Some(3)
	} else if x > 0 && y < 0 {
		Some(4)
	} else {
		None
	}
}

#[cfg(test)]
mod tests {
	use crate::get_quadrant;

	#[test]
	fn it_works() {
		assert_eq!(Some(1), get_quadrant(12, 5));
		assert_eq!(Some(4), get_quadrant(9, -13));
		assert_eq!(Some(2), get_quadrant(-12, 5));
		assert_eq!(Some(3), get_quadrant(-12, -5));
		assert_eq!(Some(4), get_quadrant(12, -5));
	}
}
