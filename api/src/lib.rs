use std::error::Error;
use serde::{Deserialize, Deserializer};
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
    pub totalResults:u32,
    articles: Vec<Article>,
    // code: Option<String>,
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
    #[serde(deserialize_with="author_default")]
    author:String,
    pub title:String,
    #[serde(deserialize_with="url_default")]
    pub url:String,
    // publishedat:String,
    #[serde(deserialize_with="date_default")]
    pub publishedAt:String
}
fn author_default<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("Noauthor".to_string())
        })
    }

fn url_default<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("NoUrl".to_string())
        })
    }
fn date_default<'de, D>(d: D) -> Result<String, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|x: Option<_>| {
            x.unwrap_or("NoUrl".to_string())
        })
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
            .await?
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
#[derive(Deserialize, Debug)]
pub struct TwitchRespone{
    pub data:Vec<TwitchInfo>
}
impl  TwitchRespone{
    pub fn data(&self) -> &Vec<TwitchInfo> {
        &self.data
    }
}
#[derive(Deserialize, Debug)]
pub struct TwitchInfo{
    pub id:String,
    login:String,
    pub display_name:String,
    pub created_at:String
}
#[derive(Deserialize, Debug)]
pub struct TwitchToken{
    pub access_token:String,
    token_type:String,
    pub expires_in:u64,
    // client_id:String,
    // client_secret:String,
    // userid:String
}
#[derive(Deserialize, Debug)]
pub struct  TwitchFollowRespone{
    data:Vec<FollowInfo>
}
impl  TwitchFollowRespone{
    pub fn data(&self) -> &Vec<FollowInfo> {
        &self.data
    }
    #[tokio::main]
    pub async fn twitch_get_follow(user_id:&str, token:&str, client_id:&str)->Result<Vec<FollowInfo>,TwitchError>{
        let mut url: Result<Url, url::ParseError>=Url::parse("https://api.twitch.tv/helix/users/follows");
        let params = [("from_id", user_id),("first","100")];
        let client = reqwest::Client::new();
        let resp=client
            .request(reqwest::Method::GET, url.unwrap())
            .query(&params)
            .header("User-Agent","yumD")
            .header("Authorization",format!("Bearer {}",token))
            .header("Client-Id",client_id)
            .send()
            .await?
            .json::<TwitchFollowRespone>()
            .await.map_err(|e|TwitchError::TwitchFolowlistFailed("Fail Get Follow".to_string()))?;
        // resp.data();
        Ok(resp.data)
    }
}
#[derive(Deserialize, Debug)]
pub struct  FollowInfo{
    pub to_id:String,
    pub to_login:String,
    pub to_name:String,
    pub followed_at:String
}
#[derive(thiserror::Error, Debug)]
pub enum TwitchError{
    #[error("Twitch login failed")]
    TwitchLoginFailed(#[from] reqwest::Error),
    #[error("Twitch get follow list fail")]
    TwitchFolowlistFailed(String),
}
impl TwitchToken{
    #[tokio::main]
    pub async fn new(client_id:&str,client_secret:&str)->Result<Self,TwitchError>{
        let grant_type="client_credentials".to_string();
        let params = [
            ("client_id", client_id), 
            ("client_secret", client_secret),  
            ("grant_type", "client_credentials")];
        let mut url: Result<Url, url::ParseError>=Url::parse("https://id.twitch.tv/oauth2/token");
        let client = reqwest::Client::new();
        let resp=client
            .request(reqwest::Method::POST, url.unwrap())
            .header("User-Agent", "yumD")
            .query(&params)
            .send()
            .await?
            .json::<TwitchToken>()
            .await.map_err(|e|TwitchError::TwitchLoginFailed((e)))?;
        Ok(resp)
    }
   
    #[tokio::main]
    pub async fn user_login(&mut self,user_id:&str,token:&str,client_id:&str)->Result<TwitchRespone,TwitchError>{
        let mut url: Result<Url, url::ParseError>=Url::parse("https://api.twitch.tv/helix/users");
        let params = [("login", user_id),];
        let client = reqwest::Client::new();
        let resp=client
        .request(reqwest::Method::GET, url.unwrap())
        .query(&params)
        .header("User-Agent","yumD")
        .header("Authorization",format!("Bearer {}",token))
        .header("Client-Id",client_id)
        .send()
        .await?
        .json::<TwitchRespone>()
        .await?;
    // let data = resp.data[0].;
    Ok(resp)
    }
}
