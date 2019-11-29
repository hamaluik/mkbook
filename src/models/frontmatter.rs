use serde::Deserialize;
use chrono::prelude::*;

#[derive(Deserialize, Default, Debug)]
pub struct ParsedFrontMatter {
    pub title: Option<String>,
    pub author: Option<String>,
    pub pubdate: Option<toml::value::Datetime>,
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct FrontMatter {
    pub title: String,
    pub author: String,
    pub pubdate: DateTime<Utc>,
    pub url: String,
}

impl ParsedFrontMatter {
    pub fn into_front(&self, root_matter: &FrontMatter, file_name: &str, url: &str) -> FrontMatter {
        FrontMatter {
            title: match &self.title {
                Some(title) => title.clone(),
                None => file_name.to_owned(),
            },
            author: match &self.author {
                Some(author) => author.clone(),
                None => root_matter.author.clone(),
            },
            pubdate: match &self.pubdate {
                Some(pubdate) => DateTime::from(DateTime::parse_from_rfc3339(&pubdate.to_string()).expect("valid rfc3339 datetime")),
                None => Utc::now(),
            },
            url: match &self.url {
                Some(url) => url.clone(),
                None => url.to_owned(),
            },
        }
    }
}

impl FrontMatter {
    pub fn from_root(root: ParsedFrontMatter) -> FrontMatter {
        let ParsedFrontMatter { title, author, pubdate, url } = root;

        FrontMatter {
            title: title.unwrap_or("My Cool Book".to_owned()),
            author: author.unwrap_or("Anonymous".to_owned()),
            pubdate: match pubdate {
                Some(pubdate) => DateTime::from(DateTime::parse_from_rfc3339(&pubdate.to_string()).expect("valid rfc3339 datetime")),
                None => Utc::now(),
            },
            url: url.unwrap_or("".to_owned()),
        }
    }
}

impl Default for FrontMatter {
    fn default() -> FrontMatter {
        FrontMatter {
            title: "My Cool Book".to_owned(),
            author: "Anonymous".to_owned(),
            pubdate: Utc::now(),
            url: String::new(),
        }
    }
}