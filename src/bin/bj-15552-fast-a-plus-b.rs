use std::io::{Read, Write};

fn main() {
	let mut case_count = String::new();
	std::io::stdin().read_line(&mut case_count).unwrap();
	let case_count = case_count.trim().parse().unwrap();

	let mut a_b_list = String::new();
	std::io::stdin().read_to_string(&mut a_b_list).unwrap();
	let a_b_list: Vec<_> = a_b_list
		.lines()
		.take(case_count)
		.map(|a_b| {
			let mut a_b = a_b.split(' ').map(|c| c.parse::<u32>().unwrap());
			(a_b.next().unwrap(), a_b.next().unwrap())
		})
		.collect();

	let mut buf = Vec::new();
	write_a_plus_b_list(&mut buf, &a_b_list);

	print!("{}", std::str::from_utf8(&buf).unwrap());
}

fn write_a_plus_b_list<W: Write>(writer: &mut W, a_b_list: &[(u32, u32)]) {
	for (a, b) in a_b_list {
		writeln!(writer, "{}", a + b).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use crate::write_a_plus_b_list;

	#[test]
	fn it_works() {
		let mut buf = Vec::new();
		write_a_plus_b_list(
			&mut buf,
			&[(1, 1), (12, 34), (5, 500), (40, 60), (1000, 1000)],
		);
		assert_eq!(
			r"2
46
505
100
2000
",
			std::str::from_utf8(&buf).unwrap()
		);
	}
}
