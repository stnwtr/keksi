use axum::{
    Json,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;

use crate::entity::Keksi;

pub async fn create_cookie(cookie_jar: CookieJar, Json(keksi): Json<Keksi>) -> Response {
    cookie_jar.add(keksi).into_response()
}

pub async fn read_cookies(cookie_jar: CookieJar) -> Response {
    Json(
        cookie_jar
            .iter()
            .map(|cookie| cookie.into())
            .collect::<Vec<Keksi>>(),
    )
        .into_response()
}
