use std::fs;
use std::collections::HashMap;
use std::hash::Hash;
use std::hash::DefaultHasher;
use std::hash::Hasher;
use itertools::Itertools;

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

	// Counts groups of # in the layout string
	fn count_matching_groups(cache: &mut HashMap<u64, u64>, layout: &str, expected_groups: &[usize], expect_bound: bool) -> u64 {
		if layout.is_empty() {
			// If both are empty, found solution
			return if expected_groups.is_empty() { 1 } else { 0 }
		} else if expected_groups.is_empty() {
			// If no group remaining, only valid solution is all '.'
			return if layout.chars().any(|c| c == '#') { 0 } else { 1 };
		}

		// Having a group requires at least 'group size' + 1 chars, except for the last. Don't bother recursing further if not
		if expected_groups.iter().sum::<usize>() + expected_groups.len() - 1 > layout.len() {
			return 0;
		}

		// Check cache if current search has been done before
		//let solution_key = layout.to_owned() + " - " + &expected_groups.iter().map(|g| g.to_owned()).join(",");
		let mut h = DefaultHasher::new();
		expected_groups.hash(&mut h);
		layout.hash(&mut h);
		let solution_key = h.finish();
		if let Some(v) = cache.get(&solution_key) {
			// Already checked this, don't bother again
			return *v;
		}

		match layout.chars().next().unwrap() {
			'.' => {
				let count = count_matching_groups(cache, &layout[1..], expected_groups, false);
				cache.insert(solution_key, count);
				return count;
			},
			'#' => {
				if !expect_bound && !layout.chars().take(expected_groups[0]).any(|c| c == '.') {
					// Got a potential group, but next char must be a boundary (end or '.')
					let count = count_matching_groups(cache, &layout[expected_groups[0]..], &expected_groups[1..], true);
					cache.insert(solution_key, count);
					return count;
				}
			},
			'?' => {
				return count_matching_groups(cache, &layout[1..], expected_groups, false) + if !expect_bound {
					let candidate = "#".to_owned() + &layout[1..];
					count_matching_groups(cache, &candidate, expected_groups, false)
				} else {
					0
				}
			},
			_ => unreachable!("Nope!"),
		}

		return 0;
	}

	let p1_values: Vec<_> = input
		.iter()
		.map(|line| -> u64 {
			let (layout_slice, groups_slice) = line.split_once(' ').unwrap();
			let expected_groups: Vec<_> = groups_slice
				.split(',')
				.map(|c| c.parse::<usize>().unwrap())
				.collect();
			let layout = layout_slice.trim_end_matches(|c| c == '.'); // End '.' don't affect results
			let mut cache: HashMap<u64, u64> = HashMap::new();
			return count_matching_groups(&mut cache, layout, &expected_groups, false);
		})
		.collect();

	let p1_result = p1_values.iter().sum::<u64>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 7922);

	let p2_values: Vec<_> = input
		.iter()
		.map(|line| -> u64 {
			let (layout_slice, groups_slice) = line
				.split_once(' ')
				.map(|(x, y)| ((0..5).map(|_| x).join("?"), (0..5).map(|_| y).join(",")))  // Unfold both parts 5 times
				.unwrap();
			let expected_groups: Vec<_> = groups_slice
				.split(',')
				.map(|c| c.parse::<usize>().unwrap())
				.collect();
			let layout = layout_slice.trim_end_matches(|c| c == '.'); // End '.' don't affect results
			let mut cache: HashMap<u64, u64> = HashMap::new();
			return count_matching_groups(&mut cache, layout, &expected_groups, false);
		})
		.collect();

	let p2_result = p2_values.iter().sum::<u64>();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 18093821750095);

}
