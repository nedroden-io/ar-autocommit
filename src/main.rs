use crate::cli::parse_args;

mod cli;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let run_config = parse_args()?;

    todo!("Continue implementing")
}