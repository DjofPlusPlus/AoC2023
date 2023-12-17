use std::fs;
use itertools::Itertools;

#[repr(C, align(16))]
pub struct AlignedU128(u128);

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<&str> = file
		.lines()
		.collect();
	let _input_test1 = [
		"???.### 1,1,3",
		".??..??...?##. 1,1,3",
		"?#?#?#?#?#?#?#? 1,3,1,6",
		"????.#...#... 4,1,1",
		"????.######..#####. 1,6,5",
		"?###???????? 3,2,1"];
	
	// I'm lazy, generate all possible permutations and see if they match
	fn find_matching_permutations(layout_str: &str, count_str: &str) -> usize {
		let expected_groups: Vec<_> = count_str
			.split(',')
			.map(|c| c.parse::<u32>().unwrap())
			.collect();
		let replace_positions: Vec<_> = layout_str
			.chars()
			.enumerate()
			.filter(|(_, c)| *c == '?')
			.map(|(i, _)| i)
			.collect();
		let permutations = 2_usize.pow(replace_positions.len() as u32);
		let mut matches = 0_usize;
		for mut i in 0..permutations {
			let mut candidate = layout_str.as_bytes().to_owned();
			for pos in &replace_positions {
				candidate[*pos] = if (i & 1) == 1 { b'.' } else { b'#' };
				i >>= 1;
			}
			let broken_groups: Vec<_> = candidate
				.split(|b| *b == b'.')
				.filter(|g| !g.is_empty())
				.map(|g| g.len() as u32)
				.collect();
			if broken_groups == expected_groups {
				matches += 1;
			}
		}
		matches
	}

	let p1_values: Vec<_> = input
		.iter()
		.map(|line| -> usize {
			let (layout_str, count_str) = line.split_once(' ').unwrap();
			return find_matching_permutations(layout_str, count_str);
		})
		.collect();

	let p1_result = p1_values.iter().sum::<usize>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 7922);

	// I knew brute forcing part 1 was a bad idea for AoC, it never scales
	// But I'm doubling down, so let's brute force extra hard
	let p2_values: Vec<_> = _input_test1
		.iter()
		.map(|line| -> usize {
			// I'm lazy, generate all possible permutations and see if they match
			let (layout_str, count_str) = line
				.split_once(' ')
				.map(|(x, y)| ((0..5).map(|_| x).join("?"), (0..5).map(|_| y).join(",")))  // Unfold each string 5 times
				.unwrap();
			return find_matching_permutations(&layout_str, &count_str);
		})
		.collect();

	let p2_result = p2_values.iter().sum::<usize>();
	println!("Part 2: {}", p2_result);
	assert_eq!(p1_result, 525152);

}
