use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ws::WebSocket;
use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::any;
use axum::{routing::get, Router};
use axum_extra::{headers, TypedHeader};
use hub::Hub;
use tokio::net::TcpListener;
use tokio::signal;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

mod channel;
mod client;
mod hub;

#[tokio::main]
async fn main() {
    let format = tracing_subscriber::fmt::format().compact();
    tracing_subscriber::fmt().event_format(format).init();

    let hub = Arc::new(Hub::new());

    let app = Router::new()
        .route("/ping", get("pong"))
        .route("/ws", any(ws_handler))
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        )
        .with_state(hub.clone());

    let listener = TcpListener::bind("127.0.0.1:8081").await.unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(hub))
    .await
    .unwrap();
}

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("WebSocket connected: {:?}", addr);
    println!("User agent: {}", user_agent);
    return ws.on_upgrade(|socket| handle_socket(socket));
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        println!("Received message: {:?}", msg);
        socket.send(msg).await.unwrap();
    }
}

async fn shutdown_signal(hub: Arc<Hub>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            println!("\nCtrl+C received");
            hub.dispose();
        },
        _ = terminate => {
            println!("\nTerminate received");
            hub.dispose();
        },
    }
}
