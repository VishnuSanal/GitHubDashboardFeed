use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RepoDetails {
    // pub id: u32,
    // pub name: String,
    pub description: Option<String>,
    // pub html_url: String,
    pub stargazers_count: u32,
    pub language: Option<String>,
}
