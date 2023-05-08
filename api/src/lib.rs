use std::error::Error;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;
extern crate dotenv;

use dotenv::dotenv;


const BASE_URL: &str = "https://newsapi.org/v2/top-headlines?";
#[derive(Deserialize, Debug)]
pub struct NewsAPIResponse {
    status: String,
    articles: Vec<Article>,
    code: Option<String>,
}

impl NewsAPIResponse {
    pub fn articles(&self) -> &Vec<Article> {
        &self.articles
    }
}

#[derive(Deserialize, Debug)]
pub struct Articles{
    pub articles:Vec<Article>
}
#[derive(Deserialize, Debug)]
pub struct Article{
    author:String,
    pub title:String,
    pub url:String,
    // publishedat:String,
    pub publishedAt:String
}
enum Country {
    Kr,
    Us
}
#[derive(thiserror::Error, Debug)]
pub enum ApiError{
    #[error("Fail fetching article")]
    RequestFailed(ureq::Error),
    #[error("Fail converting response to string")]
    FailedResponseToString(std::io::Error),
    #[error("Article Parsing failed")]
    ArticleParsingFail(serde_json::Error),
    #[error("API KEY Error")]
    ApiError(dotenv::Error),
    #[error("Url parsing failed")]
    UrlParsing(#[from] url::ParseError),
    #[error("Async request failed")]
    AsyncRequestFailed(#[from] reqwest::Error)
}
impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::Kr => "kr".to_string(),
            Self::Us => "us".to_string()
        }
    }
}
pub fn get_articles()->Result<Articles,ApiError>{
    dotenv().ok();
    let api_key = dotenv::var("APIKEY").map_err(|e|ApiError::ApiError(e))?;
    let url = "https://newsapi.org/v2/";
    let response = ureq::get(url).call().map_err(|e|ApiError::RequestFailed(e))
    ?.into_string().map_err(|e|ApiError::FailedResponseToString(e))?;
    let articles:Articles =  serde_json::from_str(&response).map_err(|e|ApiError::ArticleParsingFail(e))?;
    Ok(articles)
    // dbg!(articles);
    // todo!()
}
// static APP_USER_AGENT: &str = concat!(
//     env!("CARGO_PKG_NAME"),
//     "/",
//     env!("CARGO_PKG_VERSION"),
// );
#[tokio::main]
pub async fn get_articless() -> Result<NewsAPIResponse, ApiError> {
    dotenv().ok();
    let api_key = dotenv::var("APIKEY").map_err(|e|ApiError::ApiError(e))?;
    let params = [("country", "kr"), ("sortBy", "popularity"), ("pageSize", "20"), ("page", "1")];
    let client = reqwest::Client::new();
    let mut url = Url::parse("https://newsapi.org/v2")?;
    url.path_segments_mut()
    .unwrap()
    .push("top-headlines");
    
    let resp = client
        .request(reqwest::Method::GET, url)
        .query(&params)
        .header("User-Agent", "yumD")
        .header("Authorization",&api_key)
        .build()
        .map_err(|e| ApiError::AsyncRequestFailed(e))?;
    
    let response = client    
        .execute(resp)
        .await.unwrap()
        .json::<NewsAPIResponse>()
        .await.map_err(|e| ApiError::AsyncRequestFailed(e))?;
        
    Ok(response)
    // println!("{:#?}", response);
    // Ok(())
}
