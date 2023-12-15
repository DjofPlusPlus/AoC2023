use std::fs;
use std::collections::HashMap;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<&str> = file
		.lines()
		.collect();
	let _input_test = [
		"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
		"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
		"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
		"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
		"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
	];

	let p1_values: Vec<u32> = input
		.iter()
		.enumerate()
		.filter_map(|(idx, line)| -> Option<u32> {
			let (_, results) = line.split_once(": ").unwrap(); // Discard game ID, already got line index
			if results
				.split(|c| c == ';' || c == ',') // Each draw, each draw-color
				.map(|draw_color| -> bool {
					let (count_str, color) = draw_color.trim().split_once(' ').unwrap();
					let count = count_str.parse::<u32>().unwrap();
					match color {
						"red" => return count <= 12,
						"green" => return count <= 13,
						"blue" => return count <= 14,
						_ => return false
					}
				}).all(|possible| possible) {
				return Some(idx as u32 + 1);
			}
			return None;
		})
		.collect();

	let p1_result = p1_values.iter().sum::<u32>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 2377);
	

	let p2_values: Vec<u32> = input
		.iter()
		.map(|line| -> u32 {
			let (_, results) = line.split_once(": ").unwrap(); // Discard game ID, unneeded
			let (r, g, b) = results
				.split(';')
				.map(|draw| -> (u32,u32,u32) {
					let draw_array: Vec<(&str,u32)> = draw
						.split(',')
						.map(|draw_color| -> (&str, u32) {
							let (count_str, color) = draw_color.trim().split_once(' ').unwrap();
							let count = count_str.parse::<u32>().unwrap();
							(color, count)
						})
						.collect();
					let draw_map: HashMap<&str,u32> = draw_array.into_iter().collect();
					(*draw_map.get("red").unwrap_or(&0), *draw_map.get("green").unwrap_or(&0), *draw_map.get("blue").unwrap_or(&0))
				})
				.fold((0, 0, 0), |acc, (r, g, b)| (
					if acc.0 > r { acc.0 } else { r },
					if acc.1 > g { acc.1 } else { g },
					if acc.2 > b { acc.2 } else { b }));
			return r * g * b;
		})
		.collect();

	let p2_result = p2_values.iter().sum::<u32>();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 71220);
}
