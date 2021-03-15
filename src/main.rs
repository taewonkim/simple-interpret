use std::io;

fn expr(line: &str) -> i32 {
	let mut digits: [i32; 2] = [0; 2];
    let mut op: char = '+';
	let mut index: usize = 0;

	for s in line.chars() {
		if s >= '0' && s <= '9' {
			digits[index] = (s.to_string()).parse::<i32>().unwrap();
			index += 1;
		}
		else
		if s == '+' || s == '-' {
			op = s
		}
		else {
			panic!("oops!");
		}
	}
	
	if op == '+' {
		digits[0] + digits[1]
	}
	else
	if op == '-' {
		digits[0] - digits[1]
	}
	else {
		panic!("oops! unsupported operation");
	}
}

fn main() {
	let mut input = String::from("");
	println!("calc> ");
	io::stdin().read_line(&mut input).expect("Failed to read line");
	let result = expr(&input.as_str());
	println!("{}", result);
}
