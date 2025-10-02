//    IMPORTS    \\
use crate::rail_network;
pub use crate::rail_network::{Day,Train,City};

//     USES      \\
use axum::{extract::Multipart, http::StatusCode, response::IntoResponse, Json};
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::SocketAddr;
use strum_macros::{EnumVariantNames, EnumIter};
use strum::IntoEnumIterator;

pub async fn upload_csv(mut multipart: Multipart) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        // Typically you check field.name(), skip others if needed
        if let Some(filename) = field.file_name() {
            let data = field.bytes().await.unwrap();

            // Overwrite file (e.g., "uploaded.csv")
            let mut file = File::create("uploaded.csv").unwrap();
            if let Err(e) = file.write_all(&data) {
                eprintln!("File write error: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write file");
            }

            return (StatusCode::OK, format!("File '{}' uploaded successfully", filename));
        }
    }
    (StatusCode::BAD_REQUEST, "No file in multipart")
}
pub async fn get_enum_cities() -> Json<Vec<String>> {
    let enum_values = Day::iter().map(|d| d.as_ref().to_string()).collect();
    Json(enum_values)
}

pub async fn get_enum_trains() -> Json<Vec<String>> {
    let enum_values = Train::iter().map(|d| d.as_ref().to_string()).collect();
    Json(enum_values)
}

pub async fn get_enum_days() -> Json<Vec<String>> {
    let enum_values = Day::iter().map(|d| d.as_ref().to_string()).collect();
    Json(enum_values)
}