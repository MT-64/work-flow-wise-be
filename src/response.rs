use axum::{
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use http_serde::status_code;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct WebResponse {
    #[serde(with = "status_code")]
    pub code: StatusCode,
    pub message: String,
    pub data: Value,
    pub error: String,
}

#[allow(clippy::new_ret_no_self)]
impl WebResponse {
    pub fn new<'a>(
        code: StatusCode,
        message: impl ToString,
        data: impl Serialize + Deserialize<'a>,
        error: impl ToString,
    ) -> Response {
        (
            code,
            Json(WebResponse {
                code,
                message: message.to_string(),
                data: json!(&data),
                error: error.to_string(),
            }),
        )
            .into_response()
    }

    pub fn ok<'a>(message: impl ToString, data: impl Serialize + Deserialize<'a>) -> Response {
        WebResponse::new(StatusCode::OK, message, data, "")
    }

    pub fn created<'a>(message: impl ToString, data: impl Serialize + Deserialize<'a>) -> Response {
        WebResponse::new(StatusCode::CREATED, message, data, "")
    }

    pub fn no_content<'a>(
        message: impl ToString,
        data: impl Serialize + Deserialize<'a>,
    ) -> Response {
        WebResponse::new(StatusCode::NO_CONTENT, message, data, "")
    }

    pub fn unauthorized(message: impl ToString, error: impl ToString) -> Response {
        WebResponse::new(StatusCode::UNAUTHORIZED, message, json!(null), error)
    }

    pub fn forbidden(message: impl ToString, error: impl ToString) -> Response {
        WebResponse::new(StatusCode::NOT_FOUND, message, json!(null), error)
    }

    pub fn conflict(message: impl ToString, error: impl ToString) -> Response {
        WebResponse::new(StatusCode::CONFLICT, message, json!(null), error)
    }

    pub fn bad_request(message: impl ToString, error: impl ToString) -> Response {
        WebResponse::new(StatusCode::BAD_REQUEST, message, json!(null), error)
    }

    pub fn unprocessable_entity(message: impl ToString, error: impl ToString) -> Response {
        WebResponse::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            message,
            json!(null),
            error,
        )
    }

    pub fn not_found(message: impl ToString, error: impl ToString) -> Response {
        WebResponse::new(StatusCode::NOT_FOUND, message, json!(null), error)
    }

    pub fn internal_error(message: impl ToString, error: impl ToString) -> Response {
        WebResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            message,
            json!(null),
            error,
        )
    }
}

impl IntoResponse for WebResponse {
    fn into_response(self) -> Response {
        let mut headers = HeaderMap::new();
        headers.insert("Access-Control-Allow-Origin", "*".parse().unwrap());

        (self.code, Json(self)).into_response()
    }
}

#[derive(Serialize)]
pub struct WebResponseFilter {
    #[serde(with = "status_code")]
    pub code: StatusCode,
    pub message: String,
    pub data: Value,
    pub paging: Value,
    pub error: String,
}

#[allow(clippy::new_ret_no_self)]
impl WebResponseFilter {
    pub fn new<'a>(
        code: StatusCode,
        message: impl ToString,
        data: impl Serialize + Deserialize<'a>,
        paging: impl Serialize + Deserialize<'a>,
        error: impl ToString,
    ) -> Response {
        (
            code,
            Json(WebResponseFilter {
                code,
                message: message.to_string(),
                data: json!(&data),
                paging: json!(&paging),
                error: error.to_string(),
            }),
        )
            .into_response()
    }

    pub fn list<'a>(
        message: impl ToString,
        data: impl Serialize + Deserialize<'a>,
        paging: impl Serialize + Deserialize<'a>,
    ) -> Response {
        WebResponseFilter::new(StatusCode::OK, message, data, paging, "")
    }
}
