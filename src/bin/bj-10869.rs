fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<i32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (a, b) = (numbers[0], numbers[1]);

	for n in a_operations_b(a, b) {
		println!("{}", n);
	}
}

/// a, b에 대한 사칙연산과 나머지 연산의 결과를 담은 배열을 반환.<br>
/// 순서는 덧셈, 뺄셈, 곱셈, 나눗셈(나머지 버림), 나눗셈의 나머지 순.
fn a_operations_b(a: i32, b: i32) -> [i32; 5] {
	[a + b, a - b, a * b, a / b, a % b]
}

#[cfg(test)]
mod tests {
	use crate::a_operations_b;

	#[test]
	fn it_works() {
		assert_eq!([10, 4, 21, 2, 1], a_operations_b(7, 3));
		assert_eq!(
			[6_500, 2_500, 9_000_000, 2, 500],
			a_operations_b(4_500, 2_000)
		);
		assert_eq!(
			[20_000, 0, 100_000_000, 1, 0],
			a_operations_b(10_000, 10_000)
		);
	}
}
