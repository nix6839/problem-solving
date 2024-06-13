fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let year: u32 = line.trim().parse().unwrap();

	print!("{}", i32::from(is_leap_year(year)));
}

fn is_leap_year(year: u32) -> bool {
	((year % 4 == 0) && (year % 100 != 0)) || (year % 400 == 0)
}

#[cfg(test)]
mod tests {
	use crate::is_leap_year;

	#[test]
	fn it_works() {
		assert!(is_leap_year(2000));
		assert!(!is_leap_year(1999));
		assert!(is_leap_year(1988));
		assert!(!is_leap_year(1700));
	}
}
