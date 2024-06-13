_default:
	@just --list -u

test:
	@just test-solution1

test-solution1:
	cargo test

test-solution2:
	cargo test --no-default-features --features solution2
