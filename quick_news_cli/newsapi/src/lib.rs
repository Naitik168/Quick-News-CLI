#[cfg(feature = "async")]
use reqwest::Method;
use serde::Deserialize;
use url::Url;

const BASE_URL: &str = "https://newsapi.org/v2";

//enum for api error types
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed(#[from] ureq::Error),
    #[error("Failed converting response to string")]
    FailedResponseToString(#[from] std::io::Error),
    #[error("Article Parsing Failed")]
    ArticleParseFailed(#[from] serde_json::Error),
    #[error("Url Parsing Failed")]
    UrlParsingFailed(#[from] url::ParseError),
    #[error("Request failed: {0}")]
    BadRequest(&'static str),
    #[error("Async Request Failed")]
    #[cfg(feature = "async")]
    AsyncRequestFailed(#[from] reqwest::Error)
}

//structure to store response from API (vector of articles)
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

//structure of each article
pub struct Article {
    title: String,
    url: String,
}

impl Article {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> &str {
        &self.url
    }
}

//enum types for setting endpoint for API response
pub enum Endpoint {
    TopHeadlines,
}

impl ToString for Endpoint {
    fn to_string(&self) -> String {
        match self {
            Self::TopHeadlines => "top-headlines".to_string(),
        }
    }
}

//enum types for setting country for API response 
pub enum Country {
    In,
}

impl ToString for Country {
    fn to_string(&self) -> String {
        match self {
            Self::In => "in".to_string(),
        }
    }
}

//structure of news api (to build url)
pub struct NewsAPI {
    api_key: String,
    endpoint: Endpoint,
    country: Country,
}

impl NewsAPI {
    pub fn new(api_key: &str) -> NewsAPI {
        NewsAPI {
            api_key: api_key.to_string(),
            endpoint: Endpoint::TopHeadlines,
            country: Country::In,
        }
    }

    pub fn endpoint(&mut self, endpoint: Endpoint) -> &mut NewsAPI {
        self.endpoint = endpoint;
        self
    }

    pub fn country(&mut self, country: Country) -> &mut NewsAPI {
        self.country = country;
        self
    }

    //preparing url for api
    fn prepare_url(&self) -> Result<String, NewsApiError> {
        let mut url = Url::parse(BASE_URL)?;
        url.path_segments_mut().unwrap().push(&self.endpoint.to_string());

        let country = format!("country={}", self.country.to_string());
        url.set_query(Some(&country));

        Ok(url.to_string())
    }
    
    //fetching data from api
    pub fn fetch(&self) -> Result<NewsAPIResponse, NewsApiError> {

        let url = self.prepare_url()?;

        let req = ureq::get(&url).set("Authorization", &self.api_key);

        let response: NewsAPIResponse = req.call()?.into_json()?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code)),
        }
    }

    #[cfg(feature = "async")]
    //implementation of asynchronization, allows program to work further while waiting for api response
    pub async fn fetch_async(&self) -> Result<NewsAPIResponse, NewsApiError> {

        let url = self.prepare_url()?;

        let client = reqwest::Client::new();

        let request = client.request(Method::GET, url).header("Authorization", &self.api_key).header("User-Agent", "clinews").build().map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        let response: NewsAPIResponse = client.execute(request).await?.json().await.map_err(|e| NewsApiError::AsyncRequestFailed(e))?;

        match response.status.as_str() {
            "ok" => return Ok(response),
            _ => return Err(map_response_err(response.code)),
        }
    }
}

//for mapping error during api procedures
fn map_response_err(code: Option<String>) -> NewsApiError {

    if let Some(code) = code {
        match code.as_str() {
            "apiKeyDisabled" => NewsApiError::BadRequest("Your API key has been disabled"),
            _ => NewsApiError::BadRequest("Unknown error"),
        }
    } else {
        NewsApiError::BadRequest("Unknown error")
    }

}