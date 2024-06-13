fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let case_count = line.trim().parse().unwrap();
	let lines = std::io::stdin().lines().take(case_count);

	for line in lines {
		let sum: u32 = line
			.unwrap()
			.split(' ')
			.map(|c| c.parse::<u32>().unwrap())
			.sum();
		println!("{sum}");
	}
}
