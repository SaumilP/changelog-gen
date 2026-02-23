#[tokio::main]
async fn main() {
    changelog_gen::bootstrap::init_tracing(false);

    tracing::info!("Starting changeloggen-cli");

    if let Err(e) = changelog_gen::run().await {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}
