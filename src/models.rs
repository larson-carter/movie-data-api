use serde::Serialize;
use chrono::NaiveDate;

#[derive(Serialize, Clone)]
pub struct Movie {
    pub movie_id: i32,
    pub movie_name: String,
    pub movie_length: Option<i32>,
    pub movie_lang: Option<String>,
    pub release_date: Option<chrono::NaiveDate>,
    pub age_certificate: Option<String>,
    pub director_id: Option<i32>,
}

#[derive(Serialize, Clone)]
pub struct SearchResult {
    pub movie_id: i32,
    pub movie_name: String,
    pub actors: Option<Vec<String>>,
    pub director: Option<String>,
    pub release_date: Option<NaiveDate>,
}
