#![feature(slice_partition_dedup)]
#![feature(anonymous_lifetime_in_impl_trait)]
use std::fs;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<&str> = file
		.lines()
		.collect();
	let _input_test = [
		"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
		"Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
		"Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
		"Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
		"Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
		"Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
	];
	
	let p1_values: Vec<u32> = input
		.iter()
		.filter_map(|line| -> Option<u32> {
			let (_, nums_text) = line.split_once(": ").unwrap(); // Discard card ID
			let mut nums: Vec<u32> = nums_text
				.split([' ', '|']) // Just throw all the numbers together weee
				.filter(|&g| !g.is_empty())
				.map(|s| s.parse::<u32>().unwrap())
				.collect();
			nums.sort();
			let (_, dups) = nums[..].partition_dedup(); // Feels like cheating, but since there's no duplicate in the card numbers, it works
			if dups.is_empty() {
				None
			} else {
				Some(2_u32.pow(dups.len() as u32 - 1))
			}
		})
		.collect();
	
	let p1_result = p1_values.iter().sum::<u32>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 23028);

	fn p2_get_copies_from_cards<'a>(all_cards: &'a [&str], selected_cards: &'a [&str], offset: usize) -> Vec<u32> {
		selected_cards
			.into_iter()
			.enumerate()
			.filter_map(|(idx, line)| -> Option<Vec<u32>> {
				let card = (offset + idx + 1) as u32; // +1 to convert row index to card ID
				let (_, nums_text) = line.split_once(": ").unwrap(); // Discard card ID, already got it
				let mut nums: Vec<u32> = nums_text
					.split([' ', '|']) // Just throw all the numbers together weee
					.filter(|&g| !g.is_empty())
					.map(|s| s.parse::<u32>().unwrap())
					.collect();
				nums.sort();
				let (_, dups) = nums[..].partition_dedup(); // Feels like cheating, but since there's no duplicate in the card numbers, it works
				if dups.is_empty() {
					Some([card].to_vec())
				} else {
					let copies_from = offset + idx + 1; // +1 for next card
					let copies_end = copies_from + dups.len();
					let mut cards = p2_get_copies_from_cards(all_cards, &all_cards[copies_from..copies_end], copies_from);
					cards.push(card);
					Some(cards)
				}
			})
			.collect::<Vec<Vec<u32>>>()
			.concat()
	}

	let mut p2_values = p2_get_copies_from_cards(&input, &input, 0);
	p2_values.sort();
	
	let p2_result = p2_values.len();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 9236992);
}
