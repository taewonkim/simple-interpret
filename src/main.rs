use std::io::{self, Write};

mod text;

fn main() {
	let mut input = String::from("");
	print!("calc> ");
    io::stdout()
        .flush()
        .unwrap();
	io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let mut txt = text::Line::new(input);
    println!("{}", expr(&mut txt));
}

fn expr(txt: &mut text::Line) -> i32 {
    let left = (txt.next().to_string()).parse::<i32>().unwrap();
    let op = txt.next().to_string();
    let right = (txt.next().to_string()).parse::<i32>().unwrap();

    if op == "+" {
        left + right
    }
    else
    if op == "-" {
        left - right
    }
    else
    if op == "*" {
        left * right
    }
    else
    if op == "/" {
        left / right
    }
    else
    if op == "%" {
        left % right
    }
    else {
        0
    }
}
