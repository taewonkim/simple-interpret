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
    println!("{}", txt.expr());
}
