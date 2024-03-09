use std::process::Command;

use colored::*;
use reqwest::header::USER_AGENT;
use reqwest::Error;
use serde::Deserialize;
use terminal_link::Link;

#[derive(Deserialize, Debug)]
struct Actor {
    // id: u32,
    display_login: String,
}

#[derive(Deserialize, Debug)]
struct Repo {
    // id: u32,
    name: String,
    // url: String,
}

#[derive(Deserialize, Debug)]
struct Event {
    // id: String,
    r#type: String,
    actor: Actor,
    repo: Repo,
    // created_at: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        println!("No username provided, falling back to global Git username");

        let output = Command::new("git config --global user.name")
            .output()
            .expect("No Git username set");

        if output.status.success() {
            let username = String::from_utf8(output.stdout);
            println!("{:?}", username);
        } else {
            panic!("Please provide the username as a command line argument!");
        }
    }

    let request_url = format!(
        "https://api.github.com/users/{user}/received_events",
        user = args[1]
    );

    // println!("{}", request_url);

    let response = reqwest::Client::new()
        .get(&request_url)
        .header(USER_AGENT, "github_dashboard_feed")
        .send()
        .await?;

    if !response.status().is_success() {
        panic!("Something went wrong!");
    }

    let users: Vec<Event> = response.json().await?;

    for user in users {
        let event = if user.r#type == "WatchEvent" {
            "starred"
        } else if user.r#type == "ReleaseEvent" {
            "released"
        } else if user.r#type == "ForkEvent" {
            "forked"
        } else if user.r#type == "CreateEvent" {
            "created"
        } else if user.r#type == "PublicEvent" {
            "created"
        } else {
            continue;
        };

        let actor_url = format!("https://github.com/{}", &user.actor.display_login);
        let repo_url = format!("https://github.com/{}", &user.repo.name);

        let actor_link = Link::new("", &actor_url);
        let repo_link = Link::new("", &repo_url);

        let actor = format!("{:15} ({:2})", user.actor.display_login.green(), actor_link);
        let repo = format!("{:20} ({:>2})", user.repo.name.red(), repo_link);

        println!("{:<20} {:<10} {:<60}", &actor, event.blue(), &repo,)
    }

    Ok(())
}
