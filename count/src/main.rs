use std::{io::stdin, process};

use count::count_lines;

fn main() {
    let res = count_lines(stdin().lock());
    match res {
        Ok(lines) => println!("{lines} lines"),
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    }
}
