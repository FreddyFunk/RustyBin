use std::io::ErrorKind;

use actix_multipart::{form::{MultipartForm, tempfile::TempFile}};
use actix_web::{
    delete,
    get,
    post,
    put,
    HttpResponse,
    Responder,
    web,
};

use crate::repository::{config::Config, storage};

#[get("/file/{file_id}")]
pub async fn get_file(file_id: web::Path<String>, config: web::Data<Config>) -> impl Responder {
    let id = match uuid::Uuid::parse_str(&file_id) {
        Err(_) => return HttpResponse::BadRequest().body(format!("Not a UUID V4: {}", file_id)),
        Ok(id) => id, 
    };

    match storage::load_file(config.data_path.clone(), id).await {
        Ok((_file_name, file_content)) => return HttpResponse::Ok().body(file_content),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return HttpResponse::NotFound().body(format!("File with UUID {} not found.", file_id)),
            // TODO: logging errors
            _ => return HttpResponse::InternalServerError().finish(),
        }
    };
}

#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}

#[post("/file")]
pub async fn post_file(
    config: web::Data<Config>, form: MultipartForm<Upload>) -> impl Responder {
    let temp_file_path = form.file.file.path();
    let file_name = form
        .file
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");

    match storage::store_file(config.data_path.clone(), temp_file_path, file_name.to_string()).await {
        Ok(id) => HttpResponse::Ok().body(format!("{}", id)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/file/{file_id}")]
pub async fn put_file(file_id: web::Path<String>, 
    config: web::Data<Config>, form: MultipartForm<Upload>) -> impl Responder {
    if !config.allow_overwrite {
        return HttpResponse::Forbidden().finish();
    }

    let id = match uuid::Uuid::parse_str(&file_id) {
        Err(_) => return HttpResponse::BadRequest().body(format!("Not a UUID V4: {}", file_id)),
        Ok(id) => id, 
    };

    let temp_file_path = form.file.file.path();
    let file_name: &str = form
        .file
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");
    
    match storage::overwrite_file(config.data_path.clone(), temp_file_path, file_name.to_string(), id).await {
        Ok(_) => HttpResponse::Ok().body(format!("Successfully update file_id {} with {}.", file_id, file_name)),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return HttpResponse::NotFound().body(format!("File with UUID {} not found.", file_id)),
            // TODO: logging errors
            _ => return HttpResponse::InternalServerError().finish(),
        }
    }
}

#[delete("/file/{file_id}")]
pub async fn delete_file(file_id: web::Path<String>, config: web::Data<Config>) -> impl Responder {
    if !config.allow_delete {
        return HttpResponse::Forbidden().finish();
    }

    let id = match uuid::Uuid::parse_str(&file_id) {
        Err(_) => return HttpResponse::BadRequest().body(format!("Not a UUID V4: {}", file_id)),
        Ok(id) => id, 
    };
    
    match storage::delete_file(config.data_path.clone(), id).await {
        Ok(_) => HttpResponse::Ok().body(format!("Successfully delete file_id {}.", file_id)),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => return HttpResponse::NotFound().body(format!("File with UUID {} not found.", file_id)),
            // TODO: logging errors
            _ => return HttpResponse::InternalServerError().finish(),
        }
    }
}
