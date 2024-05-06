use serde::Deserialize;

#[path = "Actor.rs"]
mod actor;
use actor::Actor;

#[path = "Repo.rs"]
mod repo;
use repo::Repo;

#[derive(Deserialize, Debug)]
pub struct Event {
    // pub id: String,
    pub r#type: String,
    pub actor: Actor,
    pub repo: Repo,
    // pub created_at: String,
}
