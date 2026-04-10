mod cli;
mod app;
mod git;
mod azure;
mod app_settings;

#[tokio::main]
async fn main() {
    if let Err(e) = app::run().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}