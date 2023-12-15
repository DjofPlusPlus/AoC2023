use std::fs;
use std::collections::HashMap;

fn main() {
	let file = fs::read_to_string("input.txt")
		.expect("Failed to read input");
	let input: Vec<&str> = file
		.lines()
		.collect();
	let _input_test1 = [
		"LLR",
		"",
		"AAA = (BBB, BBB)",
		"BBB = (AAA, ZZZ)",
		"ZZZ = (ZZZ, ZZZ)"];
	let _input_test2 = [
		"LR",
		"",
		"11A = (11B, XXX)",
		"11B = (XXX, 11Z)",
		"11Z = (11B, XXX)",
		"22A = (22B, XXX)",
		"22B = (22C, 22C)",
		"22C = (22Z, 22Z)",
		"22Z = (22B, 22B)",
		"XXX = (XXX, XXX)"];
	
	type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;
	fn built_map<'a>(lines: &'a [&str]) -> NodeMap<'a> {
		let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
		for line in lines {
			if !line.contains(" = ") {
				continue;
			}
			let (node, unparsed_children) = line.split_once(" = ").unwrap();
			let children = unparsed_children
				.trim_matches(|c: char| c == '(' || c == ')')
				.split_once(", ")
				.unwrap();
			map.insert(node, children);
		}
		return map;
	}

	let input1 = &input;
	let map1 = built_map(input1);
	let directions1: Vec<_> = input1.first().unwrap().chars().collect();
	let mut steps1 = 0_usize;
	let mut node = "AAA";
	while node != "ZZZ" {
		node = match directions1[steps1 % directions1.len()] {
			'L' => map1[node].0,
			'R' => map1[node].1,
			_ => unreachable!("no bueno"),
		};
		steps1 += 1;
	}

	println!("Part 1: {}", steps1);
	assert_eq!(steps1, 19783);

	fn p2_count_steps<'a>(map: &NodeMap<'a>, directions: &[char], start_node: &'a str) -> (usize, &'a str) {
		let mut steps = 0_usize;
		let mut node = start_node;
		loop {
			node = match directions[steps % directions.len()] {
				'L' => map[node].0 ,
				'R' => map[node].1 ,
				_ => unreachable!("you fool"),
			};
			steps += 1;
			if node.ends_with('Z') {
				break;
			}
		}
		return (steps, node);
	}

	let input2 = &input;
	let map2 = built_map(input2);
	let directions2: Vec<_> = input2.first().unwrap().chars().collect();
	let start_nodes: Vec<_> = map2
		.keys()
		.filter(|k| k.ends_with('A'))
		.map(|k| *k)
		.collect();
	let steps2 = start_nodes
		.iter()
		.map(|n| -> usize {
			// The disgusting trick to this problem is that it boils down to just a lowest common denominator calculation because:
			// - By "luck", entry node's exit is always exit node's exit
			// - By "luck", the steps(entry -> exit) = steps(exit -> exit)
			// as proven by this assert
			// I didn't realize the second coincidence at first, so I thought it was a LCM + offset problem, like in 2020, fml
			let (steps, last_node) = p2_count_steps(&map2, &directions2, n);
			assert_eq!(p2_count_steps(&map2, &directions2, last_node), (steps, last_node));
			return steps;
		})
		.fold(None, |acc: Option<_>, b| if let Some(a) = acc { Some(num::integer::lcm(a, b)) } else { Some(b) })
		.unwrap();

	println!("Part 2: {}", steps2);
	assert_eq!(steps2, 9177460370549);
	
}
