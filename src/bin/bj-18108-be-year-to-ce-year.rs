fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let be_year: u32 = line.trim().parse().unwrap();

	print!("{}", be_year_to_ce_year(be_year));
}

fn be_year_to_ce_year(be_year: u32) -> u32 {
	be_year - 543
}

#[cfg(test)]
mod tests {
	use crate::be_year_to_ce_year;

	#[test]
	fn it_works() {
		assert_eq!(1998, be_year_to_ce_year(2541));
	}
}
