use actix_web::dev::Response;
use serde::{Deserialize, Serialize};
use actix_web::get;
use chrono::DateTime;
use chrono::Utc;
use crate::endpoints::tag::Tag;
use actix_web::http::header::{CONTENT_TYPE, ContentType};
use actix_web::HttpResponse;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Comic{
    pub id: Uuid,
    pub name: String,
    pub release: i64,
    pub last_updated: i64,
    pub tags: Vec<Tag>
}

impl Comic {
    pub fn new(id: Uuid, name: String, release: i64, last_updated: i64, tags: Vec<Tag>) -> Self {
        Self { id, name, release, last_updated, tags }
    }
}


pub struct ComicRequest{
    pub id: Option<String>
}

impl ComicRequest {
    pub fn new(id: Option<String>) -> Self {
        Self { id }
    }
}

#[get("/")]
pub async fn read() -> HttpResponse{
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json({"test"})
}

