mod commands;
mod timer;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use commands::{BenchArgs, HeadArgs, add_to_list, get_from_list};
use tabby_common::config::{config_index_to_id, CodeRepository, HttpModelConfigBuilder, RateLimitBuilder};
use tabby_common::path;

use csv::{Writer, Reader};
use std::fs::File;
#[derive(Debug, serde::Deserialize, serde::Serialize, PartialEq)]
struct Row{
    repo: String,
    id: String
}
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the index directory
    #[clap(short, long)]
    index_dir: Option<PathBuf>,

    /// Query to search for
    #[clap(short, long)]
    query: Option<String>,

    #[clap(short, long)]
    repository: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    Index,
    Search,
    Inspect,
    Bench(BenchArgs),
    Head(HeadArgs),
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {


    let cli = Cli::parse();
    let index_dir = cli.index_dir.unwrap_or(tabby_common::path::index_dir());
    let repository = cli.repository.unwrap_or("https://github.com/lubj/tabby".to_string());
    let query = cli.query.unwrap_or(("").to_string());

    let repos = get_from_list(repository.clone());
    let id = config_index_to_id(repos.len());
 
    let http_config = HttpModelConfigBuilder::default()
        .kind("ollama/embedding".to_string())
        .api_endpoint(Some("http://localhost:11434".to_string()))
        .model_name(Some("nomic-embed-text".to_string()))
        .rate_limit(
            RateLimitBuilder::default()
                .request_per_minute(6000)
                .build()
                .expect("Failed to create RateLimit"),
        )
        .build()?;
    //
    println!("searching {} in {}", query, index_dir.display());

    let repo = CodeRepository::new(repository.as_str(), &id);
    match cli.command {
        Commands::Search => {
            let res = commands::run_query_cli(&query, &repo, &http_config).await.expect("TODO: panic message");
            println!("found: {}",res.as_str());
        }
        Commands::Index => {
            // let http_config = HttpModelConfigBuilder::default().build()?;
            commands::run_index_cli(&repo, &http_config).await.expect("TODO: panic message");

            let res = add_to_list(repos, &repo);


        }
        Commands::Inspect => {
            commands::run_inspect_cli(&index_dir)?;
        }
        Commands::Bench(args) => {
            commands::run_bench_cli(&index_dir, &args).map_err(|e| anyhow::anyhow!("{}", e))?;
        }
        Commands::Head(args) => {
            commands::run_head_cli(&index_dir, &args)?;
        }
    };






    Ok(())
}

