use std::fs;
use itertools::Itertools;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: [[u8; 140]; 140] = file
		.lines()
		.map(|row| row.as_bytes().try_into().unwrap())
		.collect::<Vec<[u8; 140]>>()
		.try_into()
		.unwrap();
	let _input_test1: [[u8; 10]; 10] = [
		*b"...#......",
		*b".......#..",
		*b"#.........",
		*b"..........",
		*b"......#...",
		*b".#........",
		*b".........#",
		*b"..........",
		*b".......#..",
		*b"#...#....."];

	#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
	struct Point {
		x: usize,
		y: usize
	}
	
	fn get_galaxies<const W: usize, const H: usize>(grid: &[[u8; W]; H], expansion: usize) -> Vec<Point> {
		// Find empty rows/columns to expand
		let mut xempty = vec![true; W];
		let mut yempty = vec![true; H];
		for y in 0..H {
			for x in 0..W {
				if grid[y][x] == b'#' {
					xempty[x] = false;
					yempty[y] = false;
				}
			}
		}

		// Get galaxies coordinates
		let mut galaxies: Vec<Point> = Vec::new();
		let mut yoffset = 0_usize;
		for y in 0..H {
			if yempty[y] {
				yoffset += expansion;
				continue;
			}
			let mut xoffset = 0_usize;
			for x in 0..W {
				if xempty[x] {
					xoffset += expansion;
					continue;
				}
				if grid[y][x] == b'#' {
					galaxies.push(Point { x: x + xoffset, y: y + yoffset });
				}
			}
		}
		return galaxies;
	}

	let galaxies1 = get_galaxies(&input, 1);
	let p1_values: Vec<_> = galaxies1
		.iter()
		.combinations(2)
		.map(|pair| (pair[0], pair[1]))
		.map(|(g1, g2)| (g1.x as isize - g2.x as isize).abs() as usize + (g1.y as isize - g2.y as isize).abs() as usize) // No complex pathing, only x and y distance needed
		.collect();
	
	let p1_result = p1_values.iter().sum::<usize>();
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 9693756);
	
	let galaxies2 = get_galaxies(&input, 1000000 - 1);
	let p2_values: Vec<_> = galaxies2
		.iter()
		.combinations(2)
		.map(|pair| (pair[0], pair[1]))
		.map(|(g1, g2)| (g1.x as isize - g2.x as isize).abs() as usize + (g1.y as isize - g2.y as isize).abs() as usize) // No complex pathing, only x and y distance needed
		.collect();
	
	let p2_result = p2_values.iter().sum::<usize>();
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 717878258016);

}
