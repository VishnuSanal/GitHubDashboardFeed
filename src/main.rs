use std::process::Command;

use colored::*;
use reqwest::Error;
use reqwest::header::USER_AGENT;
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
    url: String,
}

#[derive(Deserialize, Debug)]
struct Event {
    // id: String,
    r#type: String,
    actor: Actor,
    repo: Repo,
    // created_at: String,
}

#[derive(Deserialize, Debug)]
struct RepoDetails {
    // id: u32,
    // name: String,
    description: Option<String>,
    // html_url: String,
    stargazers_count: u32,
    language: Option<String>,
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

    println!("{:-<100}", "");

    for user in users {
        let repo_response = reqwest::Client::new()
            .get(&user.repo.url)
            .header(USER_AGENT, "github_dashboard_feed")
            .send()
            .await?;

        let mut repo_details: RepoDetails;

        if repo_response.status().is_success() {
            repo_details = repo_response.json().await?;
        } else {
            repo_details = RepoDetails {
                description: Some("not found".to_string()),
                stargazers_count: 0,
                language: Some("not found".to_string()),
            };
        }

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

        let repo_lang = repo_details.language.expect("");
        let repo_stars = repo_details.stargazers_count.to_string();

        let actor = format!("{} ({:2})", user.actor.display_login.red(), actor_link);

        let repo = format!(
            "{} ( {}) ( {}) ({:>2})",
            user.repo.name.red(),
            repo_stars.trim().purple(),
            repo_lang.trim().bright_blue(),
            repo_link
        );

        let repo_desc = repo_details.description.get_or_insert("not found".to_string());

        println!(" => {} {} {}", &actor, event.blue(), &repo);

        println!("\t=> {:<100}", &repo_desc.bright_green());

        println!("{:-<100}", "");
    }

    Ok(())
}
