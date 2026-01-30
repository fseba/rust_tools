use anyhow::Result;

use clap::Parser;
use slim::Slimmer;

#[derive(Debug, Parser)]
#[command(bin_name = "cargo")]
enum CargoCommand {
    Slim(Args),
}

#[derive(clap::Args, Debug)]
struct Args {
    #[arg(default_value = ".")]
    paths: Vec<String>,
    #[arg(long)]
    dry_run: bool,
}

fn main() -> Result<()> {
    let CargoCommand::Slim(args) = CargoCommand::parse();
    let mut slimmer = Slimmer::new();
    slimmer.dry_run = args.dry_run;
    for path in args.paths {
        let output = slimmer.slim(path)?;
        print!("{output}");
    }
    Ok(())
}
