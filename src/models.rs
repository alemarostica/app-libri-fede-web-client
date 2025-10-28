use serde::{Serialize, Deserialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub id: String,
    pub title: String,
    pub author: String,
    pub year: u16,
    pub house: String,
    pub volume: Option<u16>,
    pub topic: String,
    pub location: String,
    pub read: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct NewBook {
    pub title: String,
    pub author: String,
    pub year: u16,
    pub house: String,
    pub volume: Option<u16>,
    pub topic: String,
    pub location: String,
    pub read: bool,
}

