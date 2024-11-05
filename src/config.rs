//! 配置模块，包括一系列常量
//!
//! 这个模块同时被build.rs包含。

pub mod server {
    /// 服务器监听地址
    pub const LISTEN_ADDR: &str = "0.0.0.0:80";

    /// 集成测试中服务器监听及访问地址
    pub const TEST_ADDR: &str = "localhost:8000";
}

pub mod log {
    /// 是否在接口中返回未处理错误的详细信息
    pub const EXPOSE_ERR: bool = true;
}

pub mod db {
    /// 程序运行时真正连接的数据库
    pub const DATABASE_URL: &str = "sqlite://students.db";

    /// 测试数据库，用于集成测试以及sqlx静态检查。
    /// 参见根目录下的`.env`文件
    pub const TEST_DATABASE_URL: &str =
        dotenvy_macro::dotenv!("DATABASE_URL");

    /// 用于初始化测试数据库和正式数据库的查询。
    pub const DATABASE_INIT_QUERY: &str = "
        CREATE TABLE IF NOT EXISTS students (
            record_id   INTEGER     PRIMARY KEY AUTOINCREMENT,
            name        CHAR(100)   NOT NULL,
            student_id  CHAR(100)   NOT NULL UNIQUE
        );
        ";
}
