use std::env;

use anyhow::{Result, bail};

use count::count_in_path;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: count <FILE>")
    }
    for path in args {
        let count = count_in_path(&path)?;
        println!("{path}: {} lines", count.lines);
    }
    Ok(())
}
