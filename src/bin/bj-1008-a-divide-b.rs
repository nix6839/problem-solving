fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<i32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (a, b) = (numbers[0], numbers[1]);

	print!("{}", a_divide_b(a as f64, b as f64));
}

fn a_divide_b(a: f64, b: f64) -> f64 {
	a / b
}

#[cfg(test)]
mod tests {
	use crate::a_divide_b;

	#[test]
	fn it_works() {
		assert!((0.333_333_333_333_333_3 - a_divide_b(1_f64, 3_f64)).abs() < f64::EPSILON);
		assert!((0.8 - a_divide_b(4_f64, 5_f64)).abs() < f64::EPSILON);
		assert!((3_f64 - a_divide_b(6_f64, 2_f64)).abs() < f64::EPSILON);
	}
}
