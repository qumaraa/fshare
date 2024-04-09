use actix_multipart::Multipart;
use actix_web::{web, HttpRequest, HttpResponse};
use futures::{StreamExt, TryStreamExt};
use std::{fs::File, io::Write, path::PathBuf};

pub async fn upload(mut payload: Multipart) -> HttpResponse {
    // Iterate over multipart items
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let filename = content_disposition.get_filename().unwrap_or("unnamed");

        // Create a destination path for the file
        let dest_path = PathBuf::from(format!("uploads/{}", filename));

        // Open a file at the destination path
        let mut f = match File::create(&dest_path) {
            Ok(f) => f,
            Err(_) => return HttpResponse::InternalServerError().finish(),
        };

        // Write the content of the field (file) to the destination file
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            if let Err(_) = f.write_all(&data) {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    HttpResponse::Ok().finish()
}

pub async fn upload_html() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("./upload.html"))
}
