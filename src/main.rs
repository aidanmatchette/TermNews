mod theme;

use std::error::Error;
use newsapi::{get_articles, Articles};
use dotenv::dotenv;

fn render_articles(articles: &Articles) {
    let theme = theme::default();
    theme.print_text("# Top headlines\n\n");
    for i in &articles.articles {
        theme.print_text(&format!("`{}`", i.title));
        theme.print_text(&format!("> *{}*", i.url));
        theme.print_text("---");
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


