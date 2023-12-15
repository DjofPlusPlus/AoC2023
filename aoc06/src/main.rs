fn main() {
	let input = [
		"Time:        49     78     79     80",
		"Distance:   298   1185   1066   1181"];
	let _input_test = [
		"Time:      7  15   30",
		"Distance:  9  40  200"];
	
	let input1 = &input;
	let p1_values: Vec<_> = input1[0]
		.split_once(": ")
		.unwrap()
		.1
		.split(' ')
		.filter(|&g| !g.is_empty())
		.map(|s| s.parse::<u32>().unwrap())
		.into_iter()
		.zip(input1[1]
			.split_once(": ")
			.unwrap()
			.1
			.split(' ')
			.filter(|&g| !g.is_empty())
			.map(|s| s.parse::<u32>().unwrap())
			.into_iter())
		.map(|(t, d)| (1..(t - 1))
			.map(|i| (t - i) * i)
			.filter(|x| *x > d)
			.count())
		.collect();

	let p1_result = p1_values
		.iter()
		.fold(1, |acc, i| acc * *i);
	println!("Part 1: {}", p1_result);
	assert_eq!(p1_result, 2269432);
	
	let input2 = &input;
	let t = input2[0]
		.split_once(": ")
		.unwrap()
		.1
		.replace(" ", "")
		.parse::<u64>().unwrap();
	let d = input2[1]
		.split_once(": ")
		.unwrap()
		.1
		.replace(" ", "")
		.parse::<u64>().unwrap();
	let p2_result = (1..(t - 1))
			.map(|i| (t - i) * i)
			.filter(|x| *x > d)
			.count();

	println!("Part 2: {}", p2_result);
	assert_eq!(p2_result, 35865985);

}
