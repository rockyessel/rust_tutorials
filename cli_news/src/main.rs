use std::error::Error;

use colour::{dark_green, yellow};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Articles {
    articles: Vec<Article>,
}

#[derive(Deserialize, Debug)]
struct Article {
    title: String,
    url: String,
}

fn get_articles(url: &str) -> Result<Articles, Box<dyn Error>> {
    let response = ureq::get(url).call()?.into_string()?;

    let articles: Articles = serde_json::from_str(&response)?;

    Ok(response)

}

fn render_articles(articles: &Articles) {
    for i in articles.articles {
        dark_green!("> {}\n", i.title);
        yellow!("> {}\n", i.url);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let url ="https://newsapi.org/v2/top-headlines?country=us&apiKey=00ed29f168a8477e84e16a3ae48166f2";
    let articles = get_articles(url);

    render_articles(&articles);

    Ok(())
}
