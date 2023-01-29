use std::error::Error;
use colour::{cyan, yellow};
use newsapi::{get_articles, Articles};
use dotenv::dotenv;

fn render_articles(articles: &Articles) {

    for article in &articles.articles {
        cyan!("-> {}\n", article.title);
        yellow!("{}\n\n", article.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;

    let api_key = std::env::var("API_KEY")?;

    let url_base: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=";

    let url: String = format!("{}{}", url_base, api_key);

    let articles: Articles = get_articles(&url)?;

    render_articles(&articles);

    Ok(())
}


