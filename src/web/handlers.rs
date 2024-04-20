use askama::Template;
use axum::response::IntoResponse;

use crate::web::html_template::HtmlTemplate;

pub async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn login() -> impl IntoResponse {
    let template = LoginTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate;

pub async fn register() -> impl IntoResponse {
    let template = RegisterTemplate {};
    HtmlTemplate(template)
}

#[derive(Template)]
#[template(path = "register.html")]
struct RegisterTemplate;
