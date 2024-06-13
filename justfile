_default:
	@just --list -u

test:
	cargo test

test-solution1:
	cargo test --no-default-features --features solution1

test-solution2:
	cargo test --no-default-features --features solution2

test-solution3:
	cargo test --no-default-features --features solution3

fmt:
	cargo fmt

lint:
	cargo clippy
