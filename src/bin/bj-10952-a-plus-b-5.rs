fn main() {
	let mut buf = String::new();
	let mut a_b_list = Vec::<(u32, u32)>::new();
	loop {
		std::io::stdin().read_line(&mut buf).unwrap();
		let input = buf.trim();
		if input == "0 0" {
			break;
		}
		let mut numbers = input.split(' ').map(|s| s.parse().unwrap());
		a_b_list.push((numbers.next().unwrap(), numbers.next().unwrap()));

		buf.clear();
	}

	for (a, b) in a_b_list {
		println!("{}", a + b);
	}
}
