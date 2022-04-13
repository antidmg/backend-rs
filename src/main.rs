use std::{collections::HashMap, str::FromStr, string::ParseError};

use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

/// Used to build Docker scratch image
/// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    let addr = "0.0.0.0:8080";
    println!("Server started on port 8080");
    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(router().into_make_service())
        .await
        .unwrap();
}

fn router() -> Router {
    Router::new()
        .route("/status", get(status))
        .route("/headlines", get(get_top_headlines))
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct NewsResponse {
    #[serde(default)]
    status: String,
    #[serde(default)]
    total_results: u64,
    #[serde(default)]
    articles: Vec<Article>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct Article {
    #[serde(default)]
    source: ArticleSource,
    #[serde(default)]
    author: Option<String>,
    #[serde(default)]
    title: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    url_to_image: Option<String>,
    #[serde(default)]
    published_at: String,
    #[serde(default)]
    content: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct ArticleSource {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default)]
    country: String,
    #[serde(default)]
    category: String,
}

#[derive(Debug)]
pub enum AppError {
    InternalError,
    NewsApiError(reqwest::Error),
}

impl From<reqwest::Error> for AppError {
    fn from(inner: reqwest::Error) -> Self {
        AppError::NewsApiError(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::NewsApiError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error while querying News API",
            ),
            AppError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"),
        };
        (status, err_msg).into_response()
    }
}

async fn get_top_headlines(Query(params): Query<Params>) -> Result<Json<NewsResponse>, AppError> {
    // TODO: encrypt this and store in DB
    let api_key = "bbcf86da23f043cca81cdfd53cf16217";

    let url = format!(
        "https://newsapi.org/v2/top-headlines?country={}&category={}&apiKey={}",
        params.country, params.category, api_key,
    );

    let res = reqwest::get(url).await?;
    // let res = match client.get(url).send().await {
    //     Ok(res) => res,
    //     Err(e) => {
    //         println!("{:?}", e);
    //         return Err(AppError::NewsApiError(e));
    //     }
    // };
    // let res: NewsResponse = res.json::<NewsResponse>().await?;
    let text = res.text().await?;

    let deserializer = &mut serde_json::Deserializer::from_str(&text);
    let result: Result<NewsResponse, _> = serde_path_to_error::deserialize(deserializer);
    let result = match result {
        Ok(res) => res,
        Err(e) => {
            panic!("{}", e);
        }
    };

    // let text = res.text().await.unwrap();
    //println!("query text: {:?}", text);
    // let res: NewsResponse = text.parse::<NewsResponse>()?;
    Ok(Json(result))
}

// impl FromStr for NewsResponse {
//     type Err = ParseError;
//     fn from_str(str: &str) -> Result<Self, Self::Err> {}
// }

async fn status() -> impl IntoResponse {
    println!("Status OK");
    StatusCode::OK
}
