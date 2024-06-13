fn main() {
	print!("{}", hello_world());
}

fn hello_world() -> String {
	"Hello World!".to_string()
}

#[cfg(test)]
mod tests {
	use crate::hello_world;

	#[test]
	fn it_works() {
		assert_eq!("Hello World!", hello_world());
	}
}
