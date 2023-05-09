use std::error::Error;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;
extern crate dotenv;

use dotenv::dotenv;


static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);
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
pub struct NewsApi{
    base_url:String,
    Country:String,
    total_page:String,
    current_page:String,
}
impl NewsApi{
    pub fn new(country:&str, total_page:u32,current_page:u32) -> NewsApi{
        NewsApi { 
            base_url:"https://newsapi.org/v2".to_string(),
            Country: country.to_string(), 
            total_page: total_page.to_string(), 
            current_page: current_page.to_string(),
        }
    }
    pub fn update(&mut self,country:&str, total_page:u32,current_page:u32){
        self.Country=country.to_string();
        self.total_page=total_page.to_string();
        self.current_page=current_page.to_string();
    }
    #[tokio::main]
    pub async fn get_api(&self,api_key:String)-> Result<NewsAPIResponse, ApiError>{
        // let api_key = dotenv::var("APIKEY").map_err(|e|ApiError::ApiError(e))?;
        let params = [
            ("country", self.Country.as_str()), 
            ("sortBy", "popularity"), 
            ("pageSize", self.total_page.as_str()), 
            ("page", self.current_page.as_str())];
        let mut url = Url::parse(&self.base_url)?;
        url.path_segments_mut()
        .unwrap()
        .push("top-headlines");
        let client = reqwest::Client::new();
        let resp = client
            .request(reqwest::Method::GET, url)
            .query(&params)
            .header("User-Agent", APP_USER_AGENT)
            .header("Authorization",api_key)
            .build()
            .map_err(|e| ApiError::AsyncRequestFailed(e))?;
        let response = client    
            .execute(resp)
            .await.unwrap()
            .json::<NewsAPIResponse>()
            .await.map_err(|e| ApiError::AsyncRequestFailed(e))?;
        Ok(response)
    }
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

