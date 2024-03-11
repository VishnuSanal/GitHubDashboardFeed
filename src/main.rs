/*use std::process::Command;

use colored::*;
use reqwest::Error;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use terminal_link::Link;

use std::io::{self, stdout};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};

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
    name: String,
    description: Option<String>,
    // html_url: String,
    stargazers_count: i32,
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

    println!("{:=<70}", "");

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

        println!(" => {:<40} {:<10} {:<60} ", &actor, event.blue(), &repo);

        let response = reqwest::Client::new()
            .get(&user.repo.url)
            .header(USER_AGENT, "github_dashboard_feed")
            .send()
            .await?;

        if response.status().is_success() {
            let repo_details: RepoDetails = response.json().await?;

            println!("{:>70}", &repo_details.description.expect(""));
            println!("{:>70}", format!(" {}", &repo_details.stargazers_count));
            println!("{:>70}", &repo_details.language.expect(""));
        }
    }

    println!("{:=<70}", "");

    Ok(())
}*/

use std::io::{self, stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{prelude::*, widgets::*};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    print!("\x1B[2J\x1B[1;1H"); // clear screen

    Terminal::new(CrosstermBackend::new(stdout()))?.draw(ui)?;

    disable_raw_mode()?;
    Ok(())
}

fn ui(frame: &mut Frame) {
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Percentage(100),
            Constraint::Length(1),
        ],
    ).split(frame.size());

    frame.render_widget(
        Block::new().borders(Borders::TOP).title("GitHub Dashboard Feed"),
        main_layout[0],
    );

    frame.render_widget(
        Block::new().borders(Borders::TOP),
        main_layout[2],
    );

    let row_container = Layout::new(
        Direction::Vertical,
        [0, 1, 2, 3, 4, 5, 6, 7, 8, 9].iter().map(|_c| Constraint::Fill(1)),
    ).split(main_layout[1]);

    for i in 0..10 {
        let single_row = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(100)],
        ).split(row_container[i]);

        let span1 = Span::raw("Hello ");

        let span2 = Span::styled(
            "World",
            Style::new()
                .fg(Color::Green)
                .bg(Color::White)
                .add_modifier(Modifier::BOLD),
        );

        let span3 = Span::raw("!");

        let text: Text = Text::from(vec![Line::from(vec![span1, span2, span3])]);

        frame.render_widget(
            Paragraph::new(text)
                .block(Block::new().borders(Borders::ALL).title("Title Bar")),
            single_row[0],
        );
    }
}