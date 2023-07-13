#![deny(missing_docs)]
//! Sample Rust application for interacting with a Convex backend.
use clap::{Args, Parser, Subcommand};
use convex::{ConvexClient, FunctionResult, Value};
use futures::StreamExt;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    /// Convex deployment URL
    deployment_url: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Follow along with chat messages as they occur
    Follow(FollowArgs),

    /// Send a chat message
    Send(SendArgs),
}

#[derive(Args)]
struct FollowArgs;

#[derive(Args)]
struct SendArgs {
    /// Author name
    author: String,

    /// Message body
    body: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Follow(_) => follow(cli.deployment_url.as_str()).await,
        Commands::Send(args) => send(cli.deployment_url.as_str(), &args.author, &args.body).await,
    }
}

/// Subscribe to chat messages and show them in a console UI.
async fn follow(deployment_url: &str) {
    let mut ui = crate::ui::UI::new();
    let mut convex = ConvexClient::new(deployment_url).await.unwrap();
    let mut sub = convex
        .subscribe("messages:collect", maplit::btreemap! {})
        .await
        .unwrap();
    while let Some(result) = sub.next().await {
        if let FunctionResult::Value(Value::Array(messages)) = result {
            ui.update(messages);
        }
    }
}

/// Send a single chat message using a Convex mutation and exit.
async fn send(deployment_url: &str, author: &str, body: &str) {
    let mut convex = ConvexClient::new(deployment_url).await.unwrap();
    convex
        .mutation(
            "messages:add",
            maplit::btreemap! {
                    "author".to_owned() => Value::String(author.to_owned()),
                    "body".to_owned() => Value::String(body.to_owned()),
            },
        )
        .await
        .unwrap();
}

mod ui;
