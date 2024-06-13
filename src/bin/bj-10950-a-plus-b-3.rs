fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let case_count = line.trim().parse().unwrap();
	let lines: Vec<_> = std::io::stdin()
		.lines()
		.take(case_count)
		// 즉시 평가하기 위해 사용
		.collect();

	for line in lines {
		let numbers: Vec<u32> = line
			.unwrap()
			.split(' ')
			.map(|c| c.parse().unwrap())
			.collect();
		let (a, b) = (numbers[0], numbers[1]);
		println!("{}", a + b);
	}
}
