fn main() {
	let mut line = String::new();
	std::io::stdin().read_line(&mut line).unwrap();
	let score: u32 = line.trim().parse().unwrap();

	print!("{}", Grade::from(score));
}

enum Grade {
	A,
	B,
	C,
	D,
	F,
}

impl From<u32> for Grade {
	fn from(score: u32) -> Self {
		match score {
			90..=100 => Self::A,
			80..=89 => Self::B,
			70..=79 => Self::C,
			60..=69 => Self::D,
			_ => Self::F,
		}
	}
}

impl std::fmt::Display for Grade {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Grade::A => write!(f, "A"),
			Grade::B => write!(f, "B"),
			Grade::C => write!(f, "C"),
			Grade::D => write!(f, "D"),
			Grade::F => write!(f, "F"),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::Grade;

	#[test]
	fn it_works() {
		assert_eq!("A", Grade::from(100).to_string());
		assert_eq!("B", Grade::from(89).to_string());
		assert_eq!("F", Grade::from(30).to_string());
	}
}
