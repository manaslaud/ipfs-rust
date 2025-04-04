use actix_multipart::Multipart;
use actix_web::{HttpResponse, web};
use futures_util::stream::StreamExt as _;
use tokio::io::AsyncWriteExt;
use crate::constants::constants::_UPLOAD_DIR;
#[actix_web::post("/upload/{filename}")]
pub async fn upload(
    mut payload: Multipart,
    file_path: web::Path<std::path::PathBuf>,
) -> Result<HttpResponse, actix_web::error::Error> {
    let path: String = _UPLOAD_DIR.to_owned()+ file_path.into_inner().to_str().unwrap();
    let mut file = tokio::fs::File::create(path).await?;

    while let Some(field) = payload.next().await {
        let mut field = match field {
            Ok(field) => field,
            Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string())),
        };

        if field.name().unwrap() == "file" {
            // Write the file content to the file
            while let Some(chunk) = field.next().await {
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string())),
                };

                let _ = file.write_all(&chunk).await?;
            }
        }
    }

    Ok(HttpResponse::Ok().body("File saved successfully"))
}
