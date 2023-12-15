use std::fs;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<_> = file
		.lines()
		.collect();
	let _input_test = [
		"seeds: 79 14 55 13", // "seeds: 0 100",
		"",
		"seed-to-soil map:",
		"50 98 2",
		"52 50 48",
		"",
		"soil-to-fertilizer map:",
		"0 15 37",
		"37 52 2",
		"39 0 15",
		"",
		"fertilizer-to-water map:",
		"49 53 8",
		"0 11 42",
		"42 0 7",
		"57 7 4",
		"",
		"water-to-light map:",
		"88 18 7",
		"18 25 70",
		"",
		"light-to-temperature map:",
		"45 77 23",
		"81 45 19",
		"68 64 13",
		"",
		"temperature-to-humidity map:",
		"0 69 1",
		"1 0 69",
		"",
		"humidity-to-location map:",
		"60 56 37",
		"56 93 4"];

	let input1 = &input;
	let mut p1_values: Vec<u64> = input1
		.first()
		.unwrap()
		.split_once(": ")
		.unwrap()
		.1
		.split(" ")
		.map(|num_txt| num_txt.parse::<u64>().unwrap())
		.collect();

	// Iterate whole array but manually to remain in control of index, skip to number rows
	let mut i = 0;
	while i < input1.len() {
		let mut line = input1[i];
		if line.is_empty() || !line.chars().next().unwrap().is_ascii_digit() {
			i += 1;
			continue;
		}

		// Iterate single section, apply transform. Output to copy to not double apply tranforms
		let mut outputs = p1_values.clone();
		while i < input1.len() {
			line = input1[i];
			i += 1;
			if line.is_empty() {
				break;
			}

			let line_values: [u64; 3] = line
				.split(" ")
				.map(|s| s.parse::<u64>().unwrap())
				.collect::<Vec<_>>()
				.try_into()
				.unwrap();
			let out_offset = line_values[0];
			let in_base = line_values[1];
			let in_len = line_values[2];
			for j in 0..p1_values.len() {
				let v = p1_values[j];
				if in_base <= v && v < in_base + in_len {
					outputs[j] = v - in_base + out_offset;
				}
			}
		}
		// Save transforms
		p1_values = outputs;
	}
	
	p1_values.sort();
	let p1_result = *p1_values.first().unwrap();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 214922730);

	let input2 = &input;
	let mut p2_ranges: Vec<_> = input2
		.first()
		.unwrap()
		.split_once(": ")
		.unwrap()
		.1
		.split(" ")
		.map(|num_txt| num_txt.parse::<u64>().unwrap())
		.collect::<Vec<u64>>()
		.chunks_exact(2)
		.map(|c| c[0]..c[0] + c[1]) // Create range from base to base + len
		.collect();

	// Iterate whole array but manually to remain in control of index, skip to number rows
	let mut i = 0;
	while i < input2.len() {
		let mut line = input2[i];
		if line.is_empty() || !line.chars().next().unwrap().is_ascii_digit() {
			i += 1;
			continue;
		}

		// Iterate single section, apply transform. Output to copy to not double apply tranforms
		let mut matched: Vec<std::ops::Range<u64>> = Vec::new();
		while i < input2.len() {
			line = input2[i];
			i += 1;
			if line.is_empty() {
				break;
			}

			let line_values: [u64; 3] = line
				.split(" ")
				.map(|s| s.parse::<u64>().unwrap())
				.collect::<Vec<_>>()
				.try_into()
				.unwrap();
			let out_offset = line_values[0];
			let m = line_values[1]..(line_values[1] + line_values[2]); // Matching input range
			let mut remains: Vec<std::ops::Range<u64>> = Vec::new();
			for r in &p2_ranges {
				// This should really be intersect()
				if (r.start <= m.end) && (r.end >= m.start) {
					// Full match
					if (r.start >= m.start) && (r.end <= m.end) {
						matched.push(r.start - m.start + out_offset .. r.end - m.start + out_offset);
						continue;
					}
					// Partial match: middle
					if (m.start >= r.start) && (m.end <= r.end) {
						remains.push(r.start .. m.start);
						matched.push(m.start - m.start + out_offset .. m.end - m.start + out_offset);
						remains.push(m.end .. r.end);
						continue;
					}
					// Partial match: left/right
					if m.start < r.start {
						matched.push(r.start - m.start + out_offset .. m.end - m.start + out_offset);
						remains.push(m.end .. r.end);
					} else {
						remains.push(r.start .. m.start);
						matched.push(m.start - m.start + out_offset .. r.end - m.start + out_offset);
					}
				} else {
					// No match
					// Note: `std::ops::Range` isn't really meant to be used directly, doesn't implement copy, so requires manual clone()
					remains.push(r.clone());
				}
			}

			// Save remaining for next transform within section
			p2_ranges = remains
				.into_iter()
				.filter(|r| !r.is_empty())
				.collect()
		}

		// Section done, save remaining + matched ranges together
		p2_ranges.extend(matched
			.into_iter()
			.filter(|r| !r.is_empty()));
	}
	
	let mut p2_values: Vec<_> = p2_ranges
		.iter()
		.map(|r| r.start )
		.collect();
	p2_values.sort();
	let p2_result = *p2_values.first().unwrap();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 148041808);

}
