//! Module for the asset handler which receives the request object returns the static file content.

use crate::{
    http::{request::Request, response::Response},
    settings,
};
use log::{info, warn};
use std::fs;

/// Checks whether a static file is being requested.
pub fn is_static_file(request: &Request) -> bool {
    match settings::get_string("static.folders") {
        Ok(static_folders) => {
            for static_folder in static_folders.trim().split(",") {
                let clean_static_folder = kalgan_string::strip(&static_folder.trim(), '/');
                if request.get_uri().contains(&clean_static_folder)
                    && request
                        .get_uri()
                        .find(format!("/{}/", &clean_static_folder).as_str())
                        == Some(0)
                {
                    return true;
                }
            }
            false
        }
        Err(_e) => false,
    }
}
/// Returns the content of the static file.
pub fn serve_static(request: &Request) -> Vec<u8> {
    info!("Processing static file...");
    let uri = get_clean_static_file(&request.get_uri());
    match fs::read(&uri) {
        Ok(mut contents) => {
            let result = Response::new()
                .set_status(200)
                .set_content_type(get_content_type(&uri))
                .create();
            let mut bytes = result;
            bytes.append(&mut "\n\n".as_bytes().to_vec());
            bytes.append(&mut contents);
            bytes
        }
        Err(e) => {
            warn!("Error processing static file \"{}\".", &uri);
            warn!("{}", e);
            Vec::new()
        }
    }
}
/// Returns the content type of the static file.
fn get_content_type(uri: &str) -> &str {
    let chunks: Vec<&str> = uri.split(".").collect();
    match chunks[chunks.len() - 1] {
        "aac" => "audio/aac",
        "avi" => "video/x-msvideo",
        "bmp" => "image/bmp",
        "css" => "text/css",
        "csv" => "text/csv",
        "gif" => "image/gif",
        "html" => "text/html",
        "ico" => "image/x-icon",
        "jpg" => "image/jpg",
        "jpeg" => "image/jpeg",
        "js" => "text/javascript",
        "json" => "application/json",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        "mpeg" => "video/mpeg",
        "otf" => "font/otf",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "ttf" => "font/ttf",
        "txt" => "text/plain",
        "wav" => "audio/wav",
        "weba" => "audio/webm",
        "webm" => "video/webm",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "xhtml" => "application/xhtml+xml",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "xml" => "application/xml",
        _ => "application/octet-stream",
    }
}
/// Returns the path of the static file.
fn get_clean_static_file(uri: &str) -> &str {
    match uri.find("?") {
        Some(pos) => kalgan_string::strip_left(&uri[..pos], '/'),
        None => kalgan_string::strip_left(&uri, '/'),
    }
}
