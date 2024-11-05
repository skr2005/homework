#[allow(dead_code)]
#[path = "src/config.rs"]
mod config;

#[path = "src/database.rs"]
mod database;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // 如果测试数据库不存在，就创建一个。从而允许sqlx静态检查
    database::DB_URL.set(config::db::TEST_DATABASE_URL).unwrap();
    database::get_or_init_db_conn_pool().await;
}
