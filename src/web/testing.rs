use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    models::*,
    patterns::{self, Color, Sequence},
};

#[derive(Deserialize, IntoParams)]
pub struct LengthQuery {
    length: usize,
}

/// Get the pattern of LED colors for the given test
#[utoipa::path(
    post,
    path = "/api/test_pattern",
    request_body(content = Sequence),
    params(
        ("length" = usize, Query, description = "Lenght of the LED chain")
    ),
    responses(
        (status = 200, description = "The pattern of RGB to display", body = Vec<Color>),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Testing"
)]
pub async fn get_test_pattern(
    q: Query<LengthQuery>,
    Json(seq): Json<patterns::Sequence>,
) -> Response {
    let data = seq.as_vec(q.length);
    let data = data
        .chunks(3)
        .map(|c| patterns::Color {
            r: c[0],
            g: c[1],
            b: c[2],
        })
        .collect::<Vec<_>>();

    Json(data).into_response()
}
