use salvo::{handler, Depot, FlowCtrl, Request, Response, Writer};

use crate::app_response::AppResponse;

/// 中间件，处理任何无返回体的结果
/// 
/// 主要用途：
/// 1.  在请求不存在的接口时返回错误信息
/// 2.  在接口（错误地）没有返回体的时候返回错误信息
#[handler]
pub async fn default_response_middleware(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    ctrl.call_next(req, depot, res).await;
    let body_size = res.body.size().unwrap_or(0);
    if body_size > 0 {
        return;
    }

    match res.status_code {
        None => AppResponse::<String>::internal_server_error(
            "服务器错误：未返回任何有效信息",
        ),
        Some(status_code) => AppResponse {
            status_code,
            content: status_code
                .canonical_reason()
                .unwrap_or(&format!(
                    "空结果与未知返回状态: {}",
                    status_code.as_str()
                ))
                .into(),
        },
    }
    .write(req, depot, res)
    .await;
}
