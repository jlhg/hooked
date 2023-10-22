use axum::extract::Json;
use axum::http::StatusCode;
use serde::{Serialize};
use serde_json::{json, Value};

#[derive(Serialize)]
pub struct ResultValue<'a, T = ResultType> {
    pub r#type: T,
    pub message: &'a str
}

#[derive(Serialize)]
pub enum ResultType {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "info")]
    Info
}

pub fn render_data(status_code: StatusCode, data: Value) -> (StatusCode, Json<Value>) {
    (status_code, Json(json!({"data": data})))
}

pub fn render_result(status_code: StatusCode, result: ResultValue) -> (StatusCode, Json<Value>) {
    render_data(status_code, json!({"result": result}))
}

pub fn render_success(status_code: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    render_result(status_code, ResultValue{r#type: ResultType::Success, message: message})
}

pub fn render_error(status_code: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    render_result(status_code, ResultValue{r#type: ResultType::Error, message: message})
}

pub fn render_warning(status_code: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    render_result(status_code, ResultValue{r#type: ResultType::Warning, message: message})
}

pub fn render_info(status_code: StatusCode, message: &str) -> (StatusCode, Json<Value>) {
    render_result(status_code, ResultValue{r#type: ResultType::Info, message: message})
}

pub fn render_internal_server_error(message: &str) -> (StatusCode, Json<Value>) {
    render_error(StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn render_forbidden(message: &str) -> (StatusCode, Json<Value>) {
    render_error(StatusCode::FORBIDDEN, message)
}

pub fn render_bad_request(message: &str) -> (StatusCode, Json<Value>) {
    render_error(StatusCode::BAD_REQUEST, message)
}
