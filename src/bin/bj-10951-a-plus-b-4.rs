fn main() {
	let sums = std::io::stdin().lines().map(|line| {
		line.unwrap()
			.split(' ')
			.map(|s| s.parse::<u32>().unwrap())
			.sum::<u32>()
	});
	for sum in sums {
		println!("{sum}");
	}
}
