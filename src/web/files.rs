use axum::{
    http::{header, StatusCode, Uri},
    response::{Html, IntoResponse, Response},
};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "web/build"]
struct StaticAssets;

static INDEX_HTML: &str = "index.html";

/// Handle static files from the rust_embed
///
/// Applies logic to serve the asset directory as a SPA
pub async fn static_handler(uri: Uri) -> Response {
    let path = uri.path().trim_start_matches('/');

    if path.is_empty() || path == INDEX_HTML {
        return index_html().await;
    }

    match StaticAssets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();

            ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
        }
        None => {
            if path.contains('.') {
                return not_found().await;
            }

            index_html().await
        }
    }
}

/// Handle access to index.html
async fn index_html() -> Response {
    match StaticAssets::get(INDEX_HTML) {
        Some(content) => Html(content.data).into_response(),
        None => not_found().await,
    }
}

/// Basic 404 page
async fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404").into_response()
}
