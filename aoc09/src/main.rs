#![feature(array_windows)]
use std::fs;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<&str> = file
		.lines()
		.collect();
	let _input_test1 = [
		"0 3 6 9 12 15",
		"1 3 6 10 15 21",
		"10 13 16 21 30 45"];

	fn get_diffs(list: &[i64]) -> Vec<i64> {
		list
			.array_windows::<2>()
			.map(|w| w[1] - w[0])
			.collect()
	}

	fn get_next(list: &[i64]) -> i64 {
		let diffs = get_diffs(list);
		if diffs.len() == 1 || diffs.iter().all(|x| *x == diffs[0]) {
			return list.last().unwrap() + diffs[0];
		} else {
			return list.last().unwrap() + get_next(&diffs);
		}
	}

	let p1_values: Vec<_> = input
		.iter()
		.map(|line| line.split_whitespace()
			.filter(|t| !t.is_empty())
			.map(|t| t.parse::<i64>().unwrap())
			.collect::<Vec<i64>>())
		.map(|v| get_next(&v))
		.collect();

	let p1_result = p1_values.iter().sum::<i64>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 1953784198);

	fn get_prev(list: &[i64]) -> i64 {
		let diffs = get_diffs(list);
		if diffs.len() == 1 || diffs.iter().all(|x| *x == diffs[0]) {
			return list.first().unwrap() - diffs[0];
		} else {
			return list.first().unwrap() - get_prev(&diffs);
		}
	}

	let p2_values: Vec<_> = input
		.iter()
		.map(|line| line.split_whitespace()
			.filter(|t| !t.is_empty())
			.map(|t| t.parse::<i64>().unwrap())
			.collect::<Vec<i64>>())
		.map(|v| get_prev(&v))
		.collect();

	let p2_result = p2_values.iter().sum::<i64>();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 957);

}
