use std::net::TcpListener;

use sqlx::postgres::PgPoolOptions;
use zero2prod::{configuration::get_configuration, email_client::EmailClient, startup, telemetry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = telemetry::get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.database.with_db());

    let sender_email = configuration
        .email_client
        .sender()
        .expect("Invalid sender emai address.");
    let base_url = configuration
        .email_client
        .base_url()
        .expect("Invalid base url.");
    let timeout = configuration.email_client.timeout();

    let email_client = EmailClient::new(
        sender_email,
        base_url,
        configuration.email_client.authorization_token,
        timeout,
    );

    let listener = TcpListener::bind(format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    ))?;
    startup::run(listener, connection_pool, email_client)?.await
}
