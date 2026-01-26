use anyhow::{Result, bail};

use std::env;

use client::get_weather;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: weather <LOCATION>");
    }
    let location = args.join(" ");
    let api_key = env::var("FAKE_API_KEY")?;
    let weather = get_weather(&location, &api_key)?;
    println!("{weather}");
    Ok(())
}
