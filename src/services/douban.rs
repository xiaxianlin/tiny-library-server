use std::collections::HashMap;

use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::Deserialize;

use crate::entities::prelude::BookModel;

#[derive(Debug, Deserialize)]
struct Info {
    pub url: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
struct InfoRes {
    pub status: u64,
    pub data: Info,
}

async fn get_book_info(isbn: String) -> Option<Info> {
    let url = format!("http://182.254.212.248:7891/{}", isbn);

    let res = reqwest::get(url.as_str()).await.unwrap();
    let info_res = res.json::<InfoRes>().await;

    match info_res {
        Ok(data) => match data.status {
            0 => Some(data.data),
            _ => None,
        },
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

    let re = Regex::new(r#"<div class="subject clearfix"[^>]*>([\s\S])*?<span>[\s\S]*?<\/div>"#)
        .unwrap();
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
    let book_info = get_book_info(isbn).await;
    if let Some(info) = book_info {
        let map = parse_page(info).await;
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
    None
}
