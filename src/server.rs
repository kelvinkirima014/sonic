use std::net::SocketAddr;

use axum::{response::{IntoResponse, Response}, Router, routing::get, Json};
use axum_macros::debug_handler;
use hyper::{header, Body, StatusCode};
use serde_json::json;

#[debug_handler]
pub async fn download() -> Result<impl IntoResponse, DownloadError> {
    let download_data = vec![10; 1024 * 10];

    let response = Response::builder()
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(header::CONTENT_DISPOSITION, "attachment; filename=test_data")
        .body(Body::from(download_data))
        .map_err(|err| {
            eprintln!("Failed to send response: {}", err);
            DownloadError::Error
        })?;

        Ok(response)

}

pub async fn run() -> Result<(), hyper::Error> {
    let app: Router<(), Body> = Router::new()
        .route("/download", get(download));

    let addr: SocketAddr = ([127, 0, 0, 1], 8080).into();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}


pub enum DownloadError {
    Error,
}

impl IntoResponse for DownloadError {
    fn into_response(self) -> Response {
      let (status, err_message) = match self {
          Self::Error => (StatusCode::INTERNAL_SERVER_ERROR, "Download failed")
      };

      (status, Json(json!({"error": err_message}))).into_response()
    }
}