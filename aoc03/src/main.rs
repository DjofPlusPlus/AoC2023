use std::fs;
use std::collections::HashMap;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: [[u8; 140]; 140] = file
		.lines()
		.map(|row| row.as_bytes().try_into().unwrap())
		.collect::<Vec<[u8; 140]>>()
		.try_into()
		.unwrap();
	let _input_test: [[u8; 10]; 10] = [
		"467..114..".as_bytes().try_into().unwrap(),
		"...*......".as_bytes().try_into().unwrap(),
		"..35..633.".as_bytes().try_into().unwrap(),
		"......#...".as_bytes().try_into().unwrap(),
		"617*......".as_bytes().try_into().unwrap(),
		".....+.58.".as_bytes().try_into().unwrap(),
		"..592.....".as_bytes().try_into().unwrap(),
		"......755.".as_bytes().try_into().unwrap(),
		"...$.*....".as_bytes().try_into().unwrap(),
		".664.598..".as_bytes().try_into().unwrap(),
	];

	let mut p1_values: Vec<u32> = Vec::new();
	let rows = input;
	for (y, row) in rows.iter().enumerate() {
		let num_groups: Vec<&[u8]> = row.split(|v| !v.is_ascii_digit()).filter(|&g| !g.is_empty()).collect();
		for num_group in num_groups {
			let x = num_group.as_ptr() as usize - row.as_ptr() as usize; // Using the address of the split reference within the row array as a cheapo enumerate().split(), sue me
			let xfrom = (x as isize - 1).clamp(0, row.len() as isize) as usize;
			let xend = (x + num_group.len() + 1).clamp(x, row.len());
			let yfrom = (y as isize - 1).clamp(0, rows.len() as isize) as usize;
			let yend = (y + 2).clamp(y, row.len());
			for scan_row in &rows[yfrom..yend] {
				if scan_row[xfrom..xend].iter().any(|vv| !vv.is_ascii_digit() && vv != &('.' as u8)) {
					p1_values.push(std::str::from_utf8(num_group).unwrap().parse().unwrap());
					break;
				}
			}
		}
	}
	
	let p1_result = p1_values.iter().sum::<u32>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 537732);

	let mut gears: HashMap<usize, Vec<u32>> = HashMap::new();
	let rows = input;
	for (y, row) in rows.iter().enumerate() {
		let num_groups: Vec<&[u8]> = row.split(|v| !v.is_ascii_digit()).filter(|&g| !g.is_empty()).collect();
		for num_group in num_groups {
			let x = num_group.as_ptr() as usize - row.as_ptr() as usize; // Using the address of the split reference within the row array as a cheapo enumerate().split(), sue me
			let xfrom = (x as isize - 1).clamp(0, row.len() as isize) as usize;
			let xend = (x + num_group.len() + 1).clamp(x, row.len());
			let yfrom = (y as isize - 1).clamp(0, rows.len() as isize) as usize;
			let yend = (y + 2).clamp(y, row.len());
			for scan_row in &rows[yfrom..yend] {
				if let Some(star_pos) = scan_row[xfrom..xend].iter().position(|vv| vv == &('*' as u8)) {
					let value = std::str::from_utf8(num_group).unwrap().parse().unwrap();
					gears
						.entry(scan_row.as_ptr() as usize + xfrom + star_pos) // Don't know which row index so use ptr as ID, whatever
						.or_insert_with(Vec::new)
						.push(value);
					break;
				}
			}
		}
	}

	let p2_values: Vec<u32> = gears
		.iter()
		.filter(|group| group.1.len() == 2)
		.map(|group| group.1.first().unwrap() * group.1.last().unwrap())
		.collect();
	
	let p2_result = p2_values.iter().sum::<u32>();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 84883664);
}
