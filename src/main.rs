use std::io::{self, Write};

mod text;

fn main() {
    let mut input = String::from("");

    print!("calc> ");
    // 이걸 반드시 해줘야 할까?
    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut txt = text::Line::new(input);
    println!("{}", txt.expr());
}
