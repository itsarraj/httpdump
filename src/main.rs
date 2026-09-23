use anyhow::Result;
use axum::{extract::Request, routing::any, Router};
use clap::Parser;
use colored::*;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Port to listen on
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// Bind address
    #[arg(short, long, default_value = "127.0.0.1")]
    bind: String,
}

async fn handle_any(req: Request) -> &'static str {
    println!("\n{}", "=".repeat(50).blue());

    let method = req.method().clone();
    let uri = req.uri().clone();
    let version = req.version();

    println!(
        "{} {} {:?}",
        method.as_str().green(),
        uri.to_string().cyan(),
        version
    );

    println!("{}", "Headers:".yellow());
    for (name, value) in req.headers() {
        if let Ok(v) = value.to_str() {
            println!("  {}: {}", name.as_str().bold(), v);
        } else {
            println!("  {}: {:?}", name.as_str().bold(), value.as_bytes());
        }
    }

    let (_parts, body) = req.into_parts();

    if let Ok(bytes) = axum::body::to_bytes(body, usize::MAX).await {
        if !bytes.is_empty() {
            println!("{}", "Body:".yellow());
            if let Ok(text) = String::from_utf8(bytes.to_vec()) {
                println!("{}", text);
            } else {
                println!("{:?}", bytes.as_ref());
            }
        }
    }

    println!("{}", "=".repeat(50).blue());

    "OK\n"
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let app = Router::new().fallback(any(handle_any));

    let addr: SocketAddr = format!("{}:{}", args.bind, args.port).parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("{} {}", "httpdump listening on".green(), addr);

    axum::serve(listener, app).await?;

    Ok(())
}
