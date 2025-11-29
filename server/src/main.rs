use std::fmt::Debug;

use axum::{Router, routing::get};
use std::net::SocketAddr;
use tracing::*;
use tracing_subscriber::{FmtSubscriber, fmt::SubscriberBuilder, prelude::*};

mod app;
mod components;
mod net;
mod types;
use components::*;
mod players;
mod systems;

use types::Config;

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let tracing_sub = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .pretty()
        .finish();
    tracing::subscriber::set_global_default(tracing_sub)
        .expect("setting default subscriber failed");

    let config = Config::load();

    let app: Router<()> = Router::new().route("/ws", get(net::ws_handler));

    // let addr = SocketAddr::from(([127, 0, 0, 1], config.unwrap().tick_rate as u16));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
