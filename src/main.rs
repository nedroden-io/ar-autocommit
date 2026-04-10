use crate::cli::parse_args;

mod cli;
mod app;
mod git;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let run_config = parse_args()?;

    let client = git::GitClient::new(run_config.target.to_str().unwrap());
    client.stage_changes()?;
    client.get_diff()?;

    todo!("Continue implementing")
}