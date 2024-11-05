use homework::{config, start_server};
use reqwest::{Client, RequestBuilder, Response, StatusCode};
use serde::Deserialize;
use std::{fmt::Display, thread, time::Duration};
use tokio::sync::OnceCell;

use config::db::TEST_DATABASE_URL;
use config::server::TEST_ADDR;

async fn init_server_and_get_client() -> Client {
    static SERVER_INIT_ONCE: OnceCell<()> = OnceCell::const_new();
    SERVER_INIT_ONCE
        .get_or_init(|| async {
            // 经测试这里用tokio::spawn会出问题
            thread::spawn(|| {
                tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(start_server(TEST_ADDR, TEST_DATABASE_URL))
            });
            // 手动等待服务器
            tokio::time::sleep(Duration::from_secs(1)).await;
            // Client:new() 没放进OnceCell
            // see https://github.com/rust-lang/rustup/issues/3852
        })
        .await;
    Client::new()
}

fn is_json(slice: &str) -> bool {
    serde_json::from_str::<serde::de::IgnoredAny>(slice).is_ok()
}

#[test]
fn test_is_json() {
    assert!(is_json(r#""""#));
    assert!(is_json(r#"{"fewq": 321, "fewqe": [{"www": 3.141}, null]}"#));
    assert!(!is_json(r#""#));
    assert!(!is_json(r#"<html></html>"#));
}

/// 测试是否能正确处理不正确的请求路径或方法
#[tokio::test]
async fn test_404() {
    let client = init_server_and_get_client().await;
    async fn test_invalid_api(
        invalid_path: impl Display,
        or_invalid_method: impl Fn(String) -> RequestBuilder,
    ) {
        let res = or_invalid_method(format!(
            "http://{}{}",
            TEST_ADDR, invalid_path
        ))
        .send()
        .await
        .unwrap();
        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        // should return json, instead of default salvo 404 page
        assert!(is_json(res.text().await.unwrap().as_str()))
    }
    test_invalid_api("/non-existent-api", |p| client.get(p)).await;
    test_invalid_api("/student/non-existent-api", |p| client.get(p)).await;
    test_invalid_api("/student", |p| client.post(p)).await;
}

#[derive(Deserialize, PartialEq, Eq, Clone)]
struct Student {
    name: String,
    student_id: String,
}

/// 获取student列表并返回，且断言未发生服务器错误
async fn get_student_list() -> Vec<Student> {
    let client = init_server_and_get_client().await;
    let res = client
        .get(format!("http://{}/student/list", TEST_ADDR))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success());
    res.json().await.unwrap()
}

/// 新增/修改记录，并测试参数验证，且断言未发生服务器错误
#[must_use]
async fn put_student(
    stu_id: impl ToString,
    name: impl ToString,
) -> Response {
    let stu_id = stu_id.to_string();
    let name = name.to_string();
    let client = init_server_and_get_client().await;
    assert_eq!(
        client
            .put(format!("http://{}/student", TEST_ADDR))
            .json(&serde_json::json!({
                "student_id": stu_id,
            }))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    let res = client
        .put(format!("http://{}/student", TEST_ADDR))
        .json(&serde_json::json!({
            "student_id": stu_id,
            "name": name,
        }))
        .send()
        .await
        .unwrap();
    assert!(res.status().is_success(), "{:?}", res.text().await);
    res
}

/// 删除记录，并测试参数验证，且断言未发生服务器错误
#[must_use]
async fn delete_student(student_id: impl ToString) -> Response {
    let student_id = student_id.to_string();
    let client = init_server_and_get_client().await;
    assert_eq!(
        client
            .delete(format!("http://{}/student", TEST_ADDR))
            .json(&serde_json::json!({}))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    let response = client
        .delete(format!("http://{}/student", TEST_ADDR))
        .json(&serde_json::json!({
            "student_id": student_id
        }))
        .send()
        .await
        .unwrap();
    assert!(!response.status().is_server_error());
    response
}

/// 获取记录，并测试参数验证，且断言未发生服务器错误
#[must_use]
async fn get_student(student_id: impl ToString) -> Response {
    let student_id = student_id.to_string();
    let client = init_server_and_get_client().await;
    assert_eq!(
        client
            .get(format!("http://{}/student", TEST_ADDR))
            .query(&[("student_is", student_id.clone())])
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::BAD_REQUEST
    );
    let res = client
        .get(format!("http://{}/student", TEST_ADDR))
        .query(&[("student_id", student_id.clone())])
        .send()
        .await
        .unwrap();
    assert!(!res.status().is_server_error());
    res
}

/// 测试是否存在某条数据
///
/// 注意：没有原子性，测试时避免竞争
async fn has_student_record(
    student_id: impl ToString,
    name: impl ToString,
) -> bool {
    let student_id = student_id.to_string();
    let name = name.to_string();
    let stu_list = get_student_list().await;
    let get_stu_res = get_student(student_id.clone()).await;
    let test_stu = Student {
        student_id: student_id.clone(),
        name: name.clone(),
    };
    match get_stu_res.status() {
        status if status.is_success() => {
            get_stu_res.json::<Student>().await.unwrap() == test_stu
                && stu_list.contains(&test_stu)
        }
        StatusCode::NOT_FOUND => false,
        other => {
            panic!("unexpected status {} when getting student", other)
        }
    }
}

#[tokio::test]
async fn test_add_find_update_delete_student() {
    let test_stu_id = line!();
    let test_name = column!();

    let _ = delete_student(test_stu_id).await;
    let del_res = delete_student(test_stu_id).await;
    // 前面已经删除过一次，这里一定是未找到
    assert!(!has_student_record(test_stu_id, test_name).await);
    assert_eq!(del_res.status(), StatusCode::NOT_FOUND);

    let put_res = put_student(test_stu_id, test_name).await;
    // 前面已经删除过，这里一定是新增
    assert_eq!(put_res.status(), StatusCode::CREATED);

    assert!(has_student_record(test_stu_id, test_name).await);

    // 前面已经添加过，这里一定是修改
    assert_eq!(
        put_student(test_stu_id, test_name).await.status(),
        StatusCode::OK
    );
    let test_name2 = line!() + column!();
    assert_eq!(
        put_student(test_stu_id, test_name2).await.status(),
        StatusCode::OK
    );

    assert!(!has_student_record(test_stu_id, test_name).await);
    assert!(has_student_record(test_stu_id, test_name2).await);

    let del_res = delete_student(test_stu_id).await;
    // 前面已经添加，这里一定是成功删除
    assert!(del_res.status().is_success());

    assert!(!has_student_record(test_stu_id, test_name).await);
    assert!(!has_student_record(test_stu_id, test_name2).await);
}
