use anyhow::{Result, bail};

use std::env;

use client::Weatherstack;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: weather <LOCATION>");
    }
    let location = args.join(" ");
    let api_key = "fake_api_key";
    let ws = Weatherstack::new(api_key);
    let weather = ws.get_weather(&location)?;
    println!("{weather}");
    Ok(())
}
