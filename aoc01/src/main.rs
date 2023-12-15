use std::fs;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<&str> = file
		.split("\r\n")
		.collect();
	let _input_test1 = [
		"1abc2",
		"pqr3stu8vwx",
		"a1b2c3d4e5f",
		"treb7uchet"];
	let _input_test2 = [
		"two1nine",
		"eightwothree",
		"abcone2threexyz",
		"xtwone3four",
		"4nineeightseven2",
		"zoneight234",
		"7pqrstsixteen"];

	let p1_values = input
		.iter()
		.map(|line| -> u32 {
			let trim = line.trim_matches(char::is_alphabetic);
			let num = trim[..1].to_owned() + &trim[trim.len()-1..];
			num.parse().unwrap()
		});
		
	let p1_result = p1_values.sum::<u32>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 53386);

	let repl = [
		"one",
		"two",
		"three",
		"four",
		"five",
		"six",
		"seven",
		"eight",
		"nine"];
	let p2_values: Vec<u32> = input
		.iter()
		.map(|&line| -> u32 {
			let digits: Vec<u32> = line
				.chars()
				.enumerate()
				.filter_map(|(i,_)| -> Option<u32> {
					let slice = &line[i..];
					if let Some(m) = repl.iter().position(|&r| slice.starts_with(r)) {
						return Some(m as u32 + 1); // "one" at index 0
					}
					slice.chars().next().unwrap().to_digit(10)
				})
				.collect();
			let num = digits.iter().next().unwrap().to_string() + &digits.iter().last().unwrap().to_string();
			num.parse().unwrap()
		})
		.collect();

	let p2_result = p2_values.iter().sum::<u32>();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 53312);
}
