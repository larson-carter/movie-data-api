use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use serde::Deserialize;

use crate::models::SearchResult;

#[derive(Deserialize)]
pub struct SearchQuery {
    query: String,
}

// src/handlers.rs

pub async fn search_movies(pool: web::Data<PgPool>, params: web::Query<SearchQuery>) -> impl Responder {
    let query = params.query.trim();

    if query.is_empty() {
        return HttpResponse::BadRequest().body("Query parameter cannot be empty");
    }

    // Replace spaces with & to perform an AND search
    let ts_query = query.split_whitespace().collect::<Vec<&str>>().join(" & ");

    let results = sqlx::query_as!(
        SearchResult,
        r#"
        SELECT 
            m.movie_id,
            m.movie_name,
            COALESCE(d.first_name || ' ' || d.last_name, '') AS director,
            m.release_date,
            COALESCE(ARRAY(
                SELECT a.first_name || ' ' || a.last_name
                FROM actors a
                JOIN movies_actors ma ON a.actor_id = ma.actor_id
                WHERE ma.movie_id = m.movie_id
            ), '{}') AS actors
        FROM movies m
        LEFT JOIN directors d ON m.director_id = d.director_id
        WHERE m.search_vector @@ to_tsquery('english', $1)
        ORDER BY m.release_date DESC
        "#,
        ts_query
    )
        .fetch_all(pool.get_ref())
        .await;

    match results {
        Ok(movies) => HttpResponse::Ok().json(movies),
        Err(e) => {
            eprintln!("Search error: {:?}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}
