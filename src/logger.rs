use tracing_subscriber::{fmt, layer::SubscriberExt};

pub fn set_logger() {
    let subscriber = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(
            fmt::layer()
                .with_timer(fmt::time::ChronoLocal::rfc_3339())
                .with_writer(std::io::stdout),
        );
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
