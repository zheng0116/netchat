use axum::{
    extract::Multipart,
    response::{IntoResponse, Json},
};
use serde::Serialize;
use std::path::Path;
use tokio::fs;

#[derive(Serialize)]
pub struct UploadResponse {
    url: String,
}

pub async fn handle_upload(mut multipart: Multipart) -> impl IntoResponse {
    let upload_dir = std::env::var("UPLOAD_DIR").unwrap_or_else(|_| "uploads".to_string());
    if !Path::new(&upload_dir).exists() {
        fs::create_dir(&upload_dir).await.unwrap();
    }

    while let Some(field) = multipart.next_field().await.unwrap() {
        if field.name().unwrap() == "file" {
            let filename = field.file_name().unwrap().to_string();
            let filepath = format!("{}/{}", upload_dir, filename);

            let data = field.bytes().await.unwrap();
            fs::write(&filepath, data).await.unwrap();

            let url = format!("/uploads/{}", filename);
            return Json(UploadResponse { url });
        }
    }

    Json(UploadResponse {
        url: "error".to_string(),
    })
}
