fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let numbers: Vec<u32> = line.trim().split(' ').map(|n| n.parse().unwrap()).collect();
	let (a, b, c) = (numbers[0], numbers[1], numbers[2]);

	for n in modulo_distributive(a, b, c) {
		println!("{}", n);
	}
}

/// a, b, c에 대한 나머지 연산의 분배법칙 관련된 값을 반환.
/// 순서는 다음과 같다:
/// - `(a + b) % c`
/// - `((a % c) + (b % c)) % c`
/// - `(a * b) % c`
/// - `((a % c) * (b % c)) % c`
fn modulo_distributive(a: u32, b: u32, c: u32) -> [u32; 4] {
	[
		(a + b) % c,
		((a % c) + (b % c)) % c,
		(a * b) % c,
		((a % c) * (b % c)) % c,
	]
}

#[cfg(test)]
mod tests {
	use crate::modulo_distributive;

	#[test]
	fn it_works() {
		assert_eq!([1, 1, 0, 0], modulo_distributive(5, 8, 4));
	}
}
