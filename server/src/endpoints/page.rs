use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Page{
    pub filename: String,
    pub comic: Uuid,
    pub data: Vec<u8>
}