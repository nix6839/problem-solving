fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let n: u32 = line.trim().parse().unwrap();

	for i in 1..=9 {
		println!("{} * {} = {}", n, i, n * i);
	}
}
