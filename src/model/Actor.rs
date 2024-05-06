use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Actor {
    // pub id: u32,
    pub display_login: String,
}
