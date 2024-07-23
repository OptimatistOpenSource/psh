use std::io;
use tracing_subscriber::{
    prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, EnvFilter,
};

/// It is also possible to set the `RUST_LOG` environment variable for other level.
pub fn log_init() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));
    let stderr_layer = tracing_subscriber::fmt::layer().with_writer(io::stderr);

    tracing_subscriber::Registry::default()
        .with(stderr_layer)
        .with(env_filter)
        .init();
}
