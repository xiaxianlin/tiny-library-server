use crate::{
    entities::{book, prelude::*},
    models::SearchParams,
};
use chrono::Local;
use sea_orm::*;
use serde_json::Value;
use std::vec;

use super::oss::{upload, upload_by_url};

fn map_sort_field(order_field: Option<String>) -> book::Column {
    match order_field {
        Some(v) => match v.as_str() {
            "title" => book::Column::Title,
            "auhtor" => book::Column::Author,
            _ => book::Column::Id,
        },
        None => book::Column::Id,
    }
}

fn map_sort_type(order_type: Option<String>) -> Order {
    match order_type {
        Some(v) => match v.as_str() {
            "desc" => Order::Desc,
            _ => Order::Asc,
        },
        None => Order::Asc,
    }
}

pub async fn find_by_id(db: &DatabaseConnection, id: i64) -> Option<BookModel> {
    println!("[find_by_id] id: {}", id);
    let data = BookEntity::find_by_id(id).one(db).await;
    match data {
        Ok(book) => book,
        Err(_) => None,
    }
}

pub async fn find_by_isbn(db: &DatabaseConnection, isbn: String) -> Option<BookModel> {
    println!("[find_by_isbn] isbn: {}", isbn);
    let data = BookEntity::find()
        .filter(book::Column::Isbn.eq(isbn))
        .one(db)
        .await;
    match data {
        Ok(book) => book,
        Err(e) => {
            println!("{}", e.to_string());
            None
        }
    }
}

pub async fn search(db: &DatabaseConnection, params: SearchParams) -> Vec<BookModel> {
    println!("[search] isbn: {:?}", params);
    let data = BookEntity::find()
        .apply_if(params.keyword, |query, keyword| {
            query.filter(
                Condition::any()
                    .add(book::Column::Title.contains(&keyword))
                    .add(book::Column::Author.contains(&keyword))
                    .add(book::Column::Publisher.contains(&keyword)),
            )
        })
        .order_by(
            map_sort_field(params.order_field),
            map_sort_type(params.order_type),
        )
        .paginate(db, params.page_size.unwrap_or_else(|| 20))
        .fetch_page(params.current_page.unwrap_or_else(|| 1) - 1)
        .await;

    match data {
        Ok(books) => books,
        Err(e) => {
            println!("{}", e.to_string());
            vec![]
        }
    }
}

pub async fn create(db: &DatabaseConnection, data: BookModel) -> i64 {
    let isbn = data.isbn.unwrap();
    let exist_book = find_by_isbn(db, String::from(&isbn)).await;

    if exist_book.is_some() {
        return -1;
    }

    let image = match data.image {
        Some(url) => {
            if url.is_empty() {
                String::from("xcx/default.png")
            } else {
                upload_by_url(url, String::from(&isbn)).await
            }
        }
        None => String::from("xcx/default.png"),
    };

    let model = book::ActiveModel {
        title: Set(data.title.to_owned()),
        author: Set(data.author.to_owned()),
        image: Set(Some(image).to_owned()),
        isbn: Set(Some(isbn).to_owned()),
        pubdate: Set(data.pubdate.to_owned()),
        publisher: Set(data.publisher.to_owned()),
        ..Default::default()
    };

    BookEntity::insert(model)
        .exec(db)
        .await
        .unwrap()
        .last_insert_id
}

pub async fn delete(db: &DatabaseConnection, id: i64) -> u64 {
    let res = BookEntity::delete_by_id(id).exec(db).await.unwrap();
    res.rows_affected
}

pub async fn update(db: &DatabaseConnection, id: i64, value: Value) -> i64 {
    let data = BookEntity::find_by_id(id).one(db).await.unwrap();
    if data.is_none() {
        return -1;
    }

    let mut active: book::ActiveModel = data.unwrap().into();
    active.id = Set(id.to_owned());
    active.set_from_json(value).unwrap();

    println!("{:?}", active);

    let res = active.update(db).await;
    match res {
        Ok(_) => 1,
        Err(_) => 0,
    }
}

pub async fn update_cover(db: &DatabaseConnection, id: i64, content: Vec<u8>) -> String {
    if content.is_empty() {
        return String::from("");
    }

    let data = BookEntity::find_by_id(id).one(db).await.unwrap();
    if data.is_none() {
        return String::from("");
    }

    let model = data.unwrap();
    let name = format!("{}_{}.jpg", model.isbn.unwrap(), Local::now().timestamp());
    let image = upload(content, name).await;

    let active = book::ActiveModel {
        id: Set(id.to_owned()),
        image: Set(Some(String::from(&image))),
        ..Default::default()
    };
    let res = active.update(db).await;
    match res {
        Ok(_) => image,
        Err(_) => String::from(""),
    }
}

pub async fn replace_cover(db: &DatabaseConnection, id: i64) -> String {
    let data = BookEntity::find_by_id(id).one(db).await.unwrap();
    if data.is_none() {
        return String::from("");
    }
    let model = data.unwrap();
    let name = format!("{}_{}.jpg", model.isbn.unwrap(), Local::now().timestamp());
    let image = upload_by_url(model.image.unwrap(), name).await;

    let active = book::ActiveModel {
        id: Set(id.to_owned()),
        image: Set(Some(String::from(&image))),
        ..Default::default()
    };
    let res = active.update(db).await;
    match res {
        Ok(_) => image,
        Err(_) => String::from(""),
    }
}
