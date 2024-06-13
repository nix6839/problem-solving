fn main() {
	let mut line_count = String::new();
	std::io::stdin().read_line(&mut line_count).unwrap();
	let line_count = line_count.trim().parse().unwrap();

	let mut buf = Vec::new();
	write_star(&mut buf, line_count);
	print!("{}", std::str::from_utf8(&buf).unwrap());
}

fn write_star<W: std::io::Write>(writer: &mut W, line_count: usize) {
	for i in 0..line_count {
		writeln!(writer, "{:>line_count$}", "*".repeat(i + 1)).unwrap();
	}
}

#[cfg(test)]
mod tests {
	use crate::write_star;

	#[test]
	fn it_works() {
		let mut buf = Vec::new();
		write_star(&mut buf, 5);
		assert_eq!(
			r"    *
   **
  ***
 ****
*****
",
			std::str::from_utf8(&buf).unwrap()
		);

		buf.clear();
		write_star(&mut buf, 3);
		assert_eq!(
			r"  *
 **
***
",
			std::str::from_utf8(&buf).unwrap()
		);

		buf.clear();
		write_star(&mut buf, 8);
		assert_eq!(
			r"       *
      **
     ***
    ****
   *****
  ******
 *******
********
",
			std::str::from_utf8(&buf).unwrap()
		);
	}
}
