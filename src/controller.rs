use axum::response::{IntoResponse, Response};
use axum::Json;
use axum_extra::extract::CookieJar;
use time::OffsetDateTime;

use crate::entity::Keksi;

pub async fn create_cookie(cookie_jar: CookieJar, Json(keksi): Json<Keksi>) -> Response {
    println!(
        "[{}] create cookie: {} = {}",
        OffsetDateTime::now_utc(),
        keksi.name(),
        keksi.value()
    );
    cookie_jar.add(keksi).into_response()
}

pub async fn read_cookies(cookie_jar: CookieJar) -> Response {
    println!("[{}] read cookies", OffsetDateTime::now_utc());
    Json(
        cookie_jar
            .iter()
            .map(|cookie| cookie.into())
            .collect::<Vec<Keksi>>(),
    )
    .into_response()
}
