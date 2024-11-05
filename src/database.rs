//! 数据库交互模块
//!
//! 这个模块同时被build.rs包含。

use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

use crate::config;

/// 数据库地址，应该在使用`get_or_init_db_conn_pool`前设置。
pub static DB_URL: tokio::sync::OnceCell<&'static str> =
    tokio::sync::OnceCell::const_new();

/// 获取连接池，如果连接池/数据库/表不存在则新建。
pub async fn get_or_init_db_conn_pool() -> &'static Pool<Sqlite> {
    let db_url = DB_URL.get().expect("database URL is unspecified");
    static DATABASE_CONN_POOL_ONCE: tokio::sync::OnceCell<Pool<Sqlite>> =
        tokio::sync::OnceCell::const_new();
    DATABASE_CONN_POOL_ONCE
        .get_or_init(|| async {
            if !Sqlite::database_exists(db_url).await.unwrap() {
                Sqlite::create_database(db_url).await.unwrap();
            }
            let conn_pool = SqlitePool::connect(db_url).await.unwrap();
            sqlx::query(config::db::DATABASE_INIT_QUERY)
                .execute(&conn_pool)
                .await
                .unwrap();
            conn_pool
        })
        .await
}
