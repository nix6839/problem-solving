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
	write_a_plus_b_list_with_count_and_expression(&mut buf, &a_b_list);

	print!("{}", std::str::from_utf8(&buf).unwrap());
}

fn write_a_plus_b_list_with_count_and_expression<W: Write>(
	writer: &mut W,
	a_b_list: &[(u32, u32)],
) {
	for (i, (a, b)) in a_b_list.iter().enumerate() {
		let count = i + 1;
		let sum = a + b;
		writeln!(writer, "Case #{count}: {a} + {b} = {sum}").unwrap();
	}
}

#[cfg(test)]
mod tests {
	use crate::write_a_plus_b_list_with_count_and_expression;

	#[test]
	fn it_works() {
		let mut buf = Vec::new();
		write_a_plus_b_list_with_count_and_expression(
			&mut buf,
			&[(1, 1), (2, 3), (3, 4), (9, 8), (5, 2)],
		);
		assert_eq!(
			r"Case #1: 1 + 1 = 2
Case #2: 2 + 3 = 5
Case #3: 3 + 4 = 7
Case #4: 9 + 8 = 17
Case #5: 5 + 2 = 7
",
			std::str::from_utf8(&buf).unwrap()
		);
	}
}
