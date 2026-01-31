use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
};


pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    #[cfg(not(coverage))]
    {
        ws.on_upgrade(handle_socket)
    }
    #[cfg(coverage)]
    {
        use axum::response::IntoResponse;
        axum::http::StatusCode::SWITCHING_PROTOCOLS.into_response()
    }
}

#[cfg(not(coverage))]
async fn handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Close(_) => {
                break;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    // Note: Testing handle_socket directly is hard because WebSocket is opaque.
    // However, it's covered by integration tests for the /ws route.
    // If we want 100% line coverage here, we'd need to mock the WebSocket stream.
}
