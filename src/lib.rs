mod config;
mod service;
mod interfaces;
mod app_response;
mod default_handler;

use salvo::{conn::TcpListener, Listener, Server};

pub async fn start_server() {
    let service = service::app_service();
    let acceptor =
        TcpListener::new(config::server::LISTEN_AT).bind().await;
    Server::new(acceptor).serve(service).await;
}
