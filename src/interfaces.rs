use salvo::http::StatusCode;
use salvo::oapi::extract::JsonBody;
use salvo::Writer;
use salvo::{handler, Response};
use serde::Deserialize;

#[derive(Deserialize)]
struct DeleteStudentReq {
    student_id: String,
}

#[handler]
pub fn delete_student(
    json: JsonBody<DeleteStudentReq>,
    res: &mut Response,
) {
    res.status_code(StatusCode::ACCEPTED);
}

#[derive(Deserialize)]
struct PutStudentReq {
    student_id: String,
    name: String,
}

#[handler]
pub fn put_student(res: &mut Response) {
    res.status_code(StatusCode::ACCEPTED);
}

#[derive(Deserialize)]
struct GetStudentReq {
    student_id: String,
}

#[handler]
pub fn get_student(res: &mut Response, json: JsonBody<GetStudentReq>) {
    res.status_code(StatusCode::SERVICE_UNAVAILABLE);
}

#[handler]
pub fn get_student_list(res: &mut Response) {
    res.status_code(StatusCode::SERVICE_UNAVAILABLE);
}
