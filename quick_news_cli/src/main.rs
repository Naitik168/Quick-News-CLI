mod theme;

use dotenv::dotenv;
use newsapi::{Article, Country, Endpoint, NewsAPI};
use std::error::Error;

// to render the articles
fn render_articles(articles: &Vec<Article>) {
    //setting theme for ouput
    let theme = theme::default();

    //organising output
    theme.print_text("# QuickNews \n\n");
    for i in articles {
        theme.print_text(&format!("`# {}`", i.title()));
        theme.print_text(&format!("> *{}*", i.url()));
        theme.print_text("---");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    //fetching api key form environment 
    let api_key = std::env::var("API_KEY")?;

    //preparing url 
    let mut newsapi = NewsAPI::new(&api_key);
    newsapi.endpoint(Endpoint::TopHeadlines).country(Country::In);

    //fetching reponse from newsapi 
    let newsapi_response = newsapi.fetch_async().await?;
    
    //calling rendering fuction with received response from the website
    render_articles(&newsapi_response.articles());

    Ok(())
}