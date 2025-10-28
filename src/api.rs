use crate::models::*;
use reqwasm::{http::Request, Error};
use serde_json::to_string;

const BASE_URL: &str = "http://localhost:60000";

pub async fn fetch_books() -> Result<Vec<Book>, Error> {
    Request::get(&format!("{BASE_URL}/books")).send().await.unwrap().json().await
}

// TODO: Ha senso mettere come parametro un oggetto Book? Boh, vedrò dopo
pub async fn create_book(book: &NewBook) -> Result<Book, Error> {
    let json_body = to_string(book)?;
    
    Request::post(&format!("{BASE_URL}/books"))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .unwrap()
        .json()
        .await
}

pub async fn toggle_read(id: String) -> Result<Book, Error> {
    Request::patch(&format!("{BASE_URL}/books/{id}")).send().await.unwrap().json().await
}

pub async fn delete_book(id: String) -> Result<Book, Error> {
    Request::delete(&format!("{BASE_URL}/books/{id}")).send().await.unwrap().json().await
}

pub async fn update_book(book: &NewBook, id: String) -> Result<Book, Error> {
    let json_body = to_string(book)?;

    Request::put(&format!("{BASE_URL}/books/{id}"))
        .header("Content-Type", "application/json")
        .body(json_body)
        .send()
        .await
        .unwrap()
        .json()
        .await
}
