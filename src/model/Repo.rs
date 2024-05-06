use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repo {
    // pub id: u32,
    pub name: String,
    pub url: String,
}
