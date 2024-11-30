use std::sync::Arc;

use axum::{
    extract::{self, Query},
    response::{IntoResponse, Response},
    Json,
};
use parking_lot::Mutex;
use serde::Deserialize;
use utoipa::IntoParams;

use crate::{
    models::*,
    patterns::{self, Color, Sequence, TestSpec},
    state::State,
    web::error::APIError,
};

#[derive(Deserialize, IntoParams)]
pub struct LengthQuery {
    length: usize,
}

/// Run LED test patterns
#[utoipa::path(
    post,
    path = "/api/test/run",
    request_body(content = TestSpec),
    responses(
        (status = 200, description = "Tests started ok", body = Status),
        (status = 500, description = "Something went wrong", body = Status)
    ),
    tag = "Testing"
)]
pub async fn run_test(
    extract::State(state): extract::State<Arc<Mutex<State>>>,
    Json(test_spec): Json<TestSpec>,
) -> Response {
    let ctrl;
    {
        let state = state.lock();
        ctrl = state.player_ctrl.clone();
    }

    if let Err(e) = ctrl.send(PlayerState::Testing(test_spec)).await {
        tracing::error!("Could not start tests: {e}");
        return APIError::UnexpectedError(e.into()).into_response();
    }

    APIError::Ok.into_response()
}

/// Get the pattern of LED colors for the given test
#[utoipa::path(
    post,
    path = "/api/test/sequence",
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
pub async fn get_test_sequence(
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
