#![feature(slice_partition_dedup)]
#![feature(slice_group_by)]
#![feature(const_trait_impl)]
use std::fs;
use phf::phf_map;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<&str> = file
		.lines()
		.collect();
	let _input_test = [
		"32T3K 765",
		"T55J5 684",
		"KK677 28",
		"KTJJT 220",
		"QQQJA 483"];
	
	#[derive(Copy, Clone, Eq, PartialEq)]
	enum CardType {
		HighCard = 1,
		OnePair,
		TwoPair,
		TreeOfAKind,
		FullHouse,
		FourOfAKind,
		FiveOfAKind,
	}
	
	fn get_hand_type<'a>(hand: &'a str) -> CardType {
		let mut cards: Vec<_> = hand
			.chars()
			.collect();
		cards.sort();
		let mut group_sizes: Vec<_> = cards
			.group_by(|a, b| a == b)
			.map(|g| g.len())
			.collect();
		group_sizes.sort();
		group_sizes.reverse();
		match group_sizes.len() {
			1 => return CardType::FiveOfAKind,
			2 => match group_sizes[0] {
				4 => return CardType::FourOfAKind,
				3 => return CardType::FullHouse,
				_ => unreachable!("Hello this is bug!")
			}
			3 => match group_sizes[0] {
				3 => return CardType::TreeOfAKind,
				2 => return CardType::TwoPair,
				_ => unreachable!("Oh noo!")
			}
			4 => return CardType::OnePair,
			5 => CardType::HighCard,
			_ => unreachable!("You lose!")
		}
	}
	
	fn p1_get_hand_strenght<'a>(hand: &'a str) -> u64 {
		const PLACE: u64 = 20;
		static LABELS: phf::Map<char, u64> = phf_map! {
			'A' => 14u64,
			'K' => 13u64,
			'Q' => 12u64,
			'J' => 11u64,
			'T' => 10u64,
			'9' => 9u64, // Could use to_digit instead but having everything avoids branching
			'8' => 8u64,
			'7' => 7u64,
			'6' => 6u64,
			'5' => 5u64,
			'4' => 4u64,
			'3' => 3u64,
			'2' => 2u64};
		let hand_type = get_hand_type(hand);
		let hand_cards: Vec<_> = hand
			.chars()
			.enumerate()
			.map(|(i, c)| LABELS[&c.to_ascii_uppercase()] * PLACE.pow(5 - i as u32))
			.collect();
		hand_type as u64 * PLACE.pow(6) + hand_cards.iter().sum::<u64>()
	}

	let mut hands: Vec<_> = input
			.iter()
			.filter_map(|line| line
				.split_once(' ')
				.and_then(|(s1, s2)| Some((p1_get_hand_strenght(s1), s2.parse::<u64>().unwrap()))))
			.collect();
	hands.sort_by_key(|hand| hand.0);
	let p1_values: Vec<_> = hands
			.iter()
			.map(|(_, bet)| bet)
			.enumerate()
			.map(|(idx, bet)| (idx + 1) as u64 * bet)
			.collect();
	
	let p1_result = p1_values.iter().sum::<u64>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 251216224);

	
	fn p2_get_hand_type<'a>(hand: &'a str) -> CardType {
		let hand_type = get_hand_type(hand);
		if !hand.contains('J') || hand_type == CardType::FiveOfAKind {
			return hand_type;
		}
		// Replace J with most common card
		let mut non_jocker_cards: Vec<_> = hand.chars()
			.filter(|c| *c != 'J')
			.collect();
		non_jocker_cards.sort();
		match non_jocker_cards				
				.group_by(|a, b| a == b)
				.fold(None, |acc: Option<&[char]>, g| if acc == None || g.len() > acc?.len() { Some(g) } else { acc })
				.map(|g| g[0]) {
			None => unreachable!("404!"),
			Some(c) => return get_hand_type(&hand.replace('J', &c.to_string())),
		}
	}

	fn p2_get_hand_strenght<'a>(hand: &'a str) -> u64 {
		const PLACE: u64 = 20;
		static LABELS: phf::Map<char, u64> = phf_map! {
			'A' => 14u64,
			'K' => 13u64,
			'Q' => 12u64,
			'T' => 10u64,
			'9' => 9u64, // Could use to_digit instead but having everything avoids branching
			'8' => 8u64,
			'7' => 7u64,
			'6' => 6u64,
			'5' => 5u64,
			'4' => 4u64,
			'3' => 3u64,
			'2' => 2u64,
			'J' => 1u64,};
		let hand_type = p2_get_hand_type(hand);
		let hand_cards: Vec<_> = hand
			.chars()
			.enumerate()
			.map(|(i, c)| LABELS[&c.to_ascii_uppercase()] * PLACE.pow(5 - i as u32))
			.collect();
		hand_type as u64 * PLACE.pow(6) + hand_cards.iter().sum::<u64>()
	}

	let mut hands2: Vec<_> = input
			.iter()
			.filter_map(|line| line
				.split_once(' ')
				.and_then(|(s1, s2)| Some((p2_get_hand_strenght(s1), s2.parse::<u64>().unwrap()))))
			.collect();
	hands2.sort_by_key(|hand| hand.0);
	let p2_values: Vec<_> = hands2
			.iter()
			.map(|(_, bet)| bet)
			.enumerate()
			.map(|(idx, bet)| (idx + 1) as u64 * bet)
			.collect();

	let p2_result = p2_values.iter().sum::<u64>();
	println!("Part 2: {}", p2_result);
	//assert_eq!(p2_result, 5905);
}
