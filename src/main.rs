use clap::Parser;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::cmp::Ordering;
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Parser)]
struct Cli {
    /// Timeout in milliseconds
    #[arg(short, long, default_value_t = 3000)]
    timeout: u64,
}

fn main() {
    let cli = Cli::parse();
    let result = get_gad(&cli);
    match result {
        Ok(gad) => {
            // NOTE: a code of 2 is supposed to be white, but i prefer keeping it cyan
            let color = match gad.code.cmp(&0) {
                Ordering::Less => "\x1b[38;2;255;0;255m",    // magenta
                Ordering::Equal => "\x1b[38;2;255;255;0m",   // yellow
                Ordering::Greater => "\x1b[38;2;0;255;255m", // cyan
            };
            println!("{color}GAD::'{}'\x1b[0m", gad.status);
        }
        Err(err) => {
            println!("\x1b[38;2;255;0;0mERROR::'{err}'");
        }
    }
}

#[derive(Deserialize)]
struct GadState {
    code: i32,
    status: String,
}

fn get_gad(cli: &Cli) -> Result<GadState, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(Duration::from_millis(cli.timeout))
        .build()?;
    let text = client.get("https://state.corru.network").send()?.text()?;
    let gad = serde_json::from_str(&text)?;
    Ok(gad)
}
