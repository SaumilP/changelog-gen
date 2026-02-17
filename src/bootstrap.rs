use tracing_subscriber::{fmt, EnvFilter};

pub fn init_tracing(json: bool) {
    let filter = EnvFilter::from_default_env();

    if json {
        fmt()
            .with_env_filter(filter)
            .json()
            .init();
    } else {
        fmt()
            .with_env_filter(filter)
            .init();
    }
}
