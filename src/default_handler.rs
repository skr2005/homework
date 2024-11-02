use salvo::{
    handler, Depot, FlowCtrl, Request, Response, Writer,
};

use crate::app_response::AppResponse;

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
    'res: {
        let Some(status_code) = res.status_code else {
            break 'res AppResponse::<String>::internal_server_error(
                "服务器错误：未返回有效信息",
            );
        };
        AppResponse {
            status_code,
            content: status_code
                .canonical_reason()
                .unwrap_or(&format!(
                    "未知的返回状态: {}",
                    status_code.as_str()
                ))
                .into(),
        }
    }
    .write(req, depot, res)
    .await;
    return;
}
