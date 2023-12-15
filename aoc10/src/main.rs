use std::fs;
use std::collections::HashSet;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: [[u8; 140]; 140] = file
		.lines()
		.map(|row| row.as_bytes().try_into().unwrap())
		.collect::<Vec<[u8; 140]>>()
		.try_into()
		.unwrap();
	let _input_test1: [[u8; 5]; 5] = [
		*b".....",
		*b".S-7.",
		*b".|.|.",
		*b".L-J.",
		*b"....."];
	let _input_test2: [[u8; 5]; 5] = [
		*b"7-F7-",
		*b".FJ|7",
		*b"SJLL7",
		*b"|F--J",
		*b"LJ.LJ"];
	let _input_test3: [[u8; 20]; 10] = [
		*b".F----7F7F7F7F-7....",
		*b".|F--7||||||||FJ....",
		*b".||.FJ||||||||L7....",
		*b"FJL7L7LJLJ||LJ.L-7..",
		*b"L--J.L7...LJS7F-7L7.",
		*b"....F-J..F7FJ|L7L7L7",
		*b"....L7.F7||L7|.L7L7|",
		*b".....|FJLJ|FJ|F7|.LJ",
		*b"....FJL-7.||.||||...",
		*b"....L---J.LJ.LJLJ..."];

	#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
	struct Point {
		x: usize,
		y: usize
	}

	fn find_connected<const W: usize, const H: usize>(grid: &[[u8; W]; H], at: Point) -> Point {
		if at.y >= grid.len() || at.x >= grid[0].len() {
			panic!("Busted!");
		}
		let xfrom = (at.x as isize - 1).clamp(0, W as isize) as usize;
		let xend = (at.x + 2).clamp(at.x, W);
		let yfrom = (at.y as isize - 1).clamp(0, H as isize) as usize;
		let yend = (at.y + 2).clamp(at.y, H);
		for yy in yfrom..yend {
			for xx in xfrom..xend {
				match grid[yy][xx] {
					b'|' => if at.x != xx { continue; },
					b'-' => if at.y != yy { continue; },
					b'L' => if at.y != yy || at.x - 1 != xx { continue; },
					b'J' => if at.y != yy || at.x + 1 != xx { continue; },
					b'7' => if at.y != yy || at.x + 1 != xx { continue; },
					b'F' => if at.y != yy || at.x - 1 != xx { continue; },
					_ => continue,
				}
				return Point { x: xx, y: yy };
			}
		}
		unreachable!("Nothing here but chicken!");
	}

	fn get_pipe_path<const W: usize, const H: usize>(grid: &[[u8; W]; H], mut at: Point, mut last: Point) -> Vec<Point> {
		// Path is too long for recursion, so building this vector
		let mut path:Vec<Point> = Vec::new();
		loop {
			if at.y >= grid.len() || at.x >= grid[0].len() {
				panic!("Wrong!");
			}
			let next = match grid[at.y][at.x] {
				b'|' => if at.y < last.y { Some(Point { x: at.x, y: at.y - 1 }) } else { Some(Point { x: at.x, y: at.y + 1 }) },
				b'-' => if at.x < last.x { Some(Point { x: at.x - 1, y: at.y }) } else { Some(Point { x: at.x + 1, y: at.y }) },
				b'L' => if at.x < last.x { Some(Point { x: at.x, y: at.y - 1 }) } else { Some(Point { x: at.x + 1, y: at.y }) },
				b'J' => if at.x > last.x { Some(Point { x: at.x, y: at.y - 1 }) } else { Some(Point { x: at.x - 1, y: at.y }) },
				b'7' => if at.x > last.x { Some(Point { x: at.x, y: at.y + 1 }) } else { Some(Point { x: at.x - 1, y: at.y }) },
				b'F' => if at.x < last.x { Some(Point { x: at.x, y: at.y + 1 }) } else { Some(Point { x: at.x + 1, y: at.y }) },
				b'S' => return path,
				_ => unreachable!("Can't park there, mate!"),
			};
			assert!(!next.is_none());
			path.push(at);
			last = at;
			at = next.unwrap();
		}
	}

	let input1 = &input;
	let start1: Point = input1
		.iter()
		.enumerate()
		.filter_map(|(y, row)| -> Option<Point> {
			if let Some(x) = row.iter().position(|x| *x == b'S') {
				return Some(Point { x: x, y: y });
			}
			return None;
		})
		.next()
		.unwrap();

	let pipe_path1 = get_pipe_path(input1, find_connected(input1, start1), start1);
	let p1_result = (pipe_path1.len() / 2) + 1; // Find middle of the path
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 6773);

	let input2 = &input;
	let start2: Point = input2
		.iter()
		.enumerate()
		.filter_map(|(y, row)| -> Option<Point> {
			if let Some(x) = row.iter().position(|x| *x == b'S') {
				return Some(Point { x: x, y: y });
			}
			return None;
		})
		.next()
		.unwrap();

	// Second part is a basic is point in poligon problem, do a really simple horizontal ray-casting and alternate in/out "every" pipe (horizonal exception, see below)
	let pipe_path2 = [get_pipe_path(input2, find_connected(input2, start2), start2), vec![start2]].concat();
	let pipe_points_set: HashSet<&Point> = HashSet::from_iter(pipe_path2.iter());
	let mut p2_result = 0_usize;
	for y in 0..input2.len() {
		let mut is_inside = false;
		let mut last_pipe = 0_u8;
		for x in 0..input2[0].len() {
			if pipe_points_set.contains(&Point {x: x, y: y }) {
				let pipe = input2[y][x];
				match pipe {
					b'-' => continue, // Scanning horizontally, horizontal pipes don't change the inside state
					b'J' => if last_pipe == b'F' { continue; }, // FJ act as single |, or a diagonal /
					b'7' => if last_pipe == b'L' { continue; }, // L7 act as a single |, or a diagonal \
					_ => {}
				}
				last_pipe = pipe;
				is_inside = !is_inside;
			} else if is_inside {
				p2_result += 1;
			}
		}
		assert!(!is_inside); // What goes in must come out, the pipe path is a closed poligon
	}

	
	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 493);
}
