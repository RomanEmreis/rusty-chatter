use volga::App;
use tracing_subscriber::prelude::*;
use self::users::Users;

mod users;
mod chat;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug,volga=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let mut app = App::new()
        .with_content_root("static");

    app.add_singleton(Users::default());
    app.use_static_files();
    app.map_ws("/chat", chat::user_connected);
    app.map_err(async |err| tracing::error!("{}", err));

    app.run().await
}
