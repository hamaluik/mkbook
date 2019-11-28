use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct ParsedFrontMatter {
    pub title: Option<String>,
}

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
