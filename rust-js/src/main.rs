use axum::extract::State;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
    Router,
};
use futures::stream::StreamExt;
use futures::SinkExt;
use libsql::{Builder, Connection};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::broadcast;

mod env;
mod templates;

struct AppState {
    tx: broadcast::Sender<String>,
    db_conn: Connection,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let env = env::env();

    let db_conn = Builder::new_remote(env.libsql_url, env.libsql_auth_token)
        .build()
        .await
        .map_err(|e| e.to_string())?
        .connect()?;

    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState { tx, db_conn });

    let app = Router::new()
        .route("/", get(templates::index))
        .route("/script.js", get(|| async { include_str!("script.js") }))
        .route("/ws", get(ws_handler))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:42069").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();

    let mut rx = state.tx.subscribe();
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let _ = tx.send(text);
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort()
    };
}
