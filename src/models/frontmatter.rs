use serde::Deserialize;

#[derive(Deserialize, Default, Debug)]
pub struct ParsedFrontMatter {
    pub title: Option<String>,
}

#[derive(Debug)]
pub struct FrontMatter {
    pub title: String,
}

impl ParsedFrontMatter {
    pub fn into_front(&self, file_name: &str) -> FrontMatter {
        FrontMatter {
            title: match &self.title {
                Some(title) => title.clone(),
                None => file_name.to_owned(),
            },
        }
    }
}
