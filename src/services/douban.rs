use std::collections::HashMap;

use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;

use crate::entities::prelude::BookModel;

#[derive(Debug, Deserialize)]
struct Info {
    pub title: String,
    pub url: String,
}

async fn get_list_data(isbn: String) -> Option<String> {
    let url = format!(
        "https://search.douban.com/book/subject_search?search_text={}&cat=1001",
        isbn
    );
    let body = reqwest::get(url.as_str())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let re = Regex::new(r#"window.__DATA__ = "(.*)";"#).unwrap();

    if re.is_match(&body) {
        let cap = re.captures(&body).unwrap();
        Some(String::from(&cap[1]))
    } else {
        None
    }
}

async fn parse_list_data(data: String) -> Option<Info> {
    let mut map = HashMap::new();
    map.insert("data", &data);

    let client = reqwest::Client::new();
    let res = client
        .post("http://182.254.212.248:7891")
        .json(&map)
        .send()
        .await
        .unwrap();

    let data = res.json::<Vec<Info>>().await;
    match data {
        Ok(books) => {
            if books.len() > 0 {
                Some(Info {
                    title: books[0].title.to_string(),
                    url: books[0].url.to_string(),
                })
            } else {
                None
            }
        }
        Err(_) => None,
    }
}

async fn parse_page(book: Info) -> HashMap<String, String> {
    let body = reqwest::get(book.url.as_str())
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let re = Regex::new(r#"<div class="subject clearfix"[^>]*>([\s\S])*?<span>[\s\S]*?<\/div>"#).unwrap();
    let caps = re.find(&body).unwrap();
    
    let document = Html::parse_document(caps.as_str());
    let mut map = HashMap::new();
    map.insert("标题".to_string(), book.title);

    let cover_selector = Selector::parse("#mainpic a img").unwrap();
    for c in document.select(&cover_selector) {
        let img = c.value().attr("src").unwrap();
        map.insert("封面".to_string(), img.to_string());
    }

    let selector = Selector::parse("#info .pl").unwrap();
    for e in document.select(&selector) {
        let key = e.inner_html().replace(" ", "").replace(":", "");
        let mut value = String::from("");
        for next in e.next_siblings() {
            let node = next.value();
            if node.is_element() {
                let er = ElementRef::wrap(next).unwrap();
                if er.value().attr("class").unwrap_or_default() == "pl" || er.children().count() > 1
                {
                    break;
                }
                value.push_str(er.inner_html().trim());
            }
            if node.is_text() {
                value.push_str(node.as_text().unwrap().trim());
            }
        }
        map.insert(key, value.replace(":", ""));
    }
    map
}

pub async fn get_book_by_douban(isbn: String) -> Option<BookModel> {
    let list_data = get_list_data(isbn).await;
    if let Some(data) = list_data {
        let list_book = parse_list_data(data).await;
        if let Some(book) = list_book {
            let map = parse_page(book).await;
            return Some(BookModel {
                id: 0,
                isbn: map.get("ISBN").map(|s| s.to_string()),
                title: map.get("标题").map(|s| s.to_string()),
                author: map.get("作者").map(|s| s.to_string()),
                image: map.get("封面").map(|s| s.to_string()),
                pubdate: map.get("出版年").map(|s| s.to_string()),
                publisher: map.get("出版社").map(|s| s.to_string()),
            });
        }
    }
    None
}
