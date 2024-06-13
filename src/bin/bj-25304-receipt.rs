fn main() {
	let mut total_amount = String::new();
	std::io::stdin().read_line(&mut total_amount).unwrap();
	let total_amount: u32 = total_amount.trim().parse().unwrap();

	let mut items_count = String::new();
	std::io::stdin().read_line(&mut items_count).unwrap();
	let items_count = items_count.trim().parse().unwrap();

	let item_price_count_list = std::io::stdin().lines().take(items_count);
	let items: Vec<(u32, u32)> = item_price_count_list
		.map(|item| {
			match &item
				.unwrap()
				.split(' ')
				.map(|c| c.parse().unwrap())
				.collect::<Vec<u32>>()[..]
			{
				&[amount, count] => (amount, count),
				_ => panic!(),
			}
		})
		.collect();

	print!(
		"{}",
		if validate_total_amount(total_amount, &items) {
			"Yes"
		} else {
			"No"
		}
	);
}

fn validate_total_amount(expected_total_amount: u32, item_price_count_list: &[(u32, u32)]) -> bool {
	let actual_total_amount = item_price_count_list
		.iter()
		.fold(0, |acc, (price, count)| acc + price * count);
	actual_total_amount == expected_total_amount
}

#[cfg(test)]
mod tests {
	use crate::validate_total_amount;

	#[test]
	fn it_works() {
		assert!(validate_total_amount(
			260_000,
			&[(20000, 5), (30000, 2), (10000, 6), (5000, 8)]
		));
		assert!(!validate_total_amount(
			250_000,
			&[(20000, 5), (30000, 2), (10000, 6), (5000, 8)]
		));
	}
}
