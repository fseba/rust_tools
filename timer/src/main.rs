use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    program: String,
    args: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let output = timer::time(&args.program, &args.args)?;
    println!("output: {}", output.stdout);
    println!("output: {}", output.stderr);
    println!("That took: {:?}", output.elapsed);
    Ok(())
}
