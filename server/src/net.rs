use axum::{
    Router,
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
    routing::get,
};
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio::task;
use uuid::Uuid;

use crate::app::APP_JOB_SENDER;

#[derive(Default)]
pub struct ConnectionRegistry {
    pub conns: HashMap<Uuid, UnboundedSender<String>>,
}

pub fn init_network_resources(app: &mut crate::app::App) {
    app.insert_resource(ConnectionRegistry::default());
}

// think about later
pub fn router() -> Router {
    Router::new().route("/ws", get(ws_handler))
}

pub async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let client_id = Uuid::new_v4();

    // create an mpsc we will store in the ConnectionRegistry and that the
    // write-task will listen to
    let (tx, mut rx) = unbounded_channel::<String>();

    APP_JOB_SENDER
        .send(Box::new(move |app| {
            let registry = app
                .get_resource_mut::<ConnectionRegistry>()
                .expect("ConnectionRegistry not initialized");
            registry.conns.insert(client_id, tx);

            // spawn the player entity and register mapping
            crate::players::spawn_player(app, client_id);
        }))
        .expect("send to app job queue failed");

    let (mut sender, mut receiver) = socket.split();

    let read_task = task::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(t) => {
                    tracing::debug!("client {} -> {}", client_id, t);
                }
                Message::Binary(_b) => {
                    // this is the important one jaja
                }
                Message::Close(_) => {
                    tracing::info!("client {} disconnected (close)", client_id);
                    break;
                }
                _ => {}
            }
        }

        // removal
        let _ = APP_JOB_SENDER.send(Box::new(move |app| {
            if let Some(reg) = app.get_resource_mut::<ConnectionRegistry>() {
                reg.conns.remove(&client_id);
            }
            if let Some(players) = app.get_resource_mut::<crate::players::PlayerMap>() {
                players.map.remove(&client_id);
            }
        }));
    });

    let write_task = task::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(Message::Text(msg.into())).await.is_err() {
                break; // breaks socket, maybe later add a limit, (maybe fluke or some bullshit)
            }
        }
    });

    let _ = tokio::join!(read_task, write_task);
}
