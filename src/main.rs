use ::reqwest;
use serde_json::Value;
use std::io::{stdin, BufRead, BufReader};

const BASE_BOOK_URL: &str = "https://www.googleapis.com/books/v1";

#[derive(serde::Serialize)]
struct BookInfo {
    title: String,
    identifier: String,
    description: String,
}

#[tokio::main]
async fn main() {
    let lines = BufReader::new(stdin().lock()).lines();

    let mut writer = csv::Writer::from_writer(vec![]);

    for line in lines {
        if let Ok(content) = line {
            let search_terms: Vec<&str> = content.split(" ").collect();
            let req = format!("{}/volumes?q={}", BASE_BOOK_URL, search_terms.join("+"));
            match reqwest::get(req).await {
                Ok(resp) => {
                    let json = resp.json::<Value>().await.expect("json retrieved");
                    let book_info = parse_json_to_book_info(&json);
                    writer.serialize(book_info).expect("serializes book info");
                }
                Err(err) => {
                    //println!("Reqwest Error: {}", err)
                }
            }
        }
    }

    print!(
        "{}",
        String::from_utf8(writer.into_inner().expect("get csv format")).expect("build string")
    );
}

fn parse_json_to_book_info(json: &Value) -> BookInfo {
    let book = &json["items"][0];
    let id = book["volumeInfo"]["industryIdentifiers"]
        .as_array()
        .expect("vals")
        .first()
        .expect("has idenfier")
        .get("identifier")
        .expect("identifier present in isbn listing")
        .as_str()
        .expect("isbn id is string");
    let title = &book["volumeInfo"]["title"]
        .as_str()
        .expect("title is string");
    let description = &book["volumeInfo"]["description"]
        .as_str()
        .expect("description is string");
    BookInfo {
        title: title.to_string(),
        identifier: id.to_string(),
        description: description.to_string(),
    }
}
