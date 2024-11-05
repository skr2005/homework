//! 各个接口的handler

use salvo::handler;
use salvo::http::StatusCode;
use salvo::oapi::extract::JsonBody;
use salvo::oapi::ToParameters;
use salvo::Writer;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::app_response::{AppResponse, AppResult};
use crate::database::get_or_init_db_conn_pool;

#[derive(Deserialize, ToParameters)]
struct OnlyStudentId {
    student_id: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
struct Student {
    student_id: String,
    name: String,
}

/// 删除
#[handler]
pub async fn delete_student(
    json: JsonBody<OnlyStudentId>,
) -> AppResult<String> {
    let db = get_or_init_db_conn_pool().await;
    let delete_res = sqlx::query!(
        "DELETE FROM students WHERE student_id = ?",
        json.student_id
    )
    .execute(db)
    .await?;
    if delete_res.rows_affected() == 0 {
        AppResponse::new(StatusCode::NOT_FOUND, "未找到记录")
    } else {
        AppResponse::ok("请求成功")
    }
    .into()
}

/// 修改或新增
#[handler]
pub async fn put_student(json: JsonBody<Student>) -> AppResult<String> {
    let db = get_or_init_db_conn_pool().await;
    let mut tran = db.begin().await?;
    sqlx::query!(
        "
        UPDATE students
            SET name = ?
            WHERE student_id = ?;
        ",
        json.name,
        json.student_id
    )
    .execute(&mut *tran)
    .await?;
    let insert_result = sqlx::query!(
        "
        INSERT OR IGNORE INTO students(student_id, name)
            VALUES (?, ?);
        ",
        json.student_id,
        json.name
    )
    .execute(&mut *tran)
    .await?;
    tran.commit().await?;
    let content = "请求成功".to_string();
    if insert_result.rows_affected() > 0 {
        AppResponse::new(StatusCode::CREATED, content)
    } else {
        AppResponse::ok(content)
    }
    .into()
}

/// 查询
#[handler]
pub async fn get_student(query: OnlyStudentId) -> AppResult<Student> {
    let db = get_or_init_db_conn_pool().await;
    let optional_student = sqlx::query_as!(
        Student,
        "SELECT student_id, name FROM students WHERE student_id = ?",
        query.student_id
    )
    .fetch_optional(db)
    .await?;
    match optional_student {
        None => Err(AppResponse {
            status_code: StatusCode::NOT_FOUND,
            content: "未找到记录".to_string(),
        }),
        Some(student) => Ok(AppResponse::ok(student)),
    }
}

/// 返回所有student
#[handler]
pub async fn get_student_list() -> AppResult<Vec<Student>> {
    let db = get_or_init_db_conn_pool().await;
    AppResponse::ok(
        sqlx::query_as!(Student, "SELECT student_id, name FROM students;")
            .fetch_all(db)
            .await?,
    )
    .into()
}
