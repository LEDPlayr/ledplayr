use std::fs;
use utoipa::OpenApi;
use utoipauto::utoipauto;

use ledplayr::models;
use ledplayr::patterns;
use ledplayr::web;

#[utoipauto]
#[derive(OpenApi)]
#[openapi()]
struct ApiDoc;

fn main() {
    let doc = ApiDoc::openapi().to_pretty_json().unwrap();
    fs::write("./openapi.json", doc).unwrap();
}
