use serde::Deserialize;
use chrono::prelude::*;

#[derive(Deserialize, Default)]
pub struct ParsedBook {
    pub title: Option<String>,
    pub author: Option<String>,
    pub pubdate: Option<toml::value::Datetime>,
    pub url: Option<String>,
    pub description: Option<String>,
}

pub struct Book {
    pub title: String,
    pub author: String,
    pub pubdate: DateTime<Utc>,
    pub url: String,
    pub description: String,
}

impl From<ParsedBook> for Book {
    fn from(pb: ParsedBook) -> Book {
        Book {
            title: match pb.title {
                Some(title) => title.clone(),
                None => "My Cool Book".to_owned(),
            },
            author: match pb.author {
                Some(author) => author.clone(),
                None => "Anonymous".to_owned(),
            },
            pubdate: match pb.pubdate {
                Some(pubdate) => DateTime::from(DateTime::parse_from_rfc3339(&pubdate.to_string()).expect("valid rfc3339 datetime")),
                None => Utc::now(),
            },
            url: match pb.url {
                Some(url) => url.clone(),
                None => "".to_owned(),
            },
            description: match pb.description {
                Some(description) => super::super::format_markdown(&description).expect("book description is valid markdown"),
                None => "".to_owned(),
            },
        }
    }
}
