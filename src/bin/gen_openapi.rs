use ledplayr::web::router;
use std::fs;
use utoipa::OpenApi;

fn main() {
    let doc = router::ApiDoc::openapi().to_pretty_json().unwrap();
    fs::write("./openapi.json", doc).unwrap();
}
