//! 真·主模块
//! 
//! 分离出一个lib.rs是为了写集成测试

mod app_response;
pub mod config;
mod database;
mod default_handler;
mod interfaces;
mod service;


use salvo::{conn::TcpListener, Listener, Server};
use tokio::net::ToSocketAddrs;

pub async fn start_server(
    listen_at: impl ToSocketAddrs + Send,
    db_url: &'static str,
) {
    database::DB_URL.set(db_url).expect(
        "database URL has been set; have you called start_server twice?",
    );
    let service = service::app_service();
    let acceptor = TcpListener::new(listen_at).bind().await;
    Server::new(acceptor).serve(service).await;
}
