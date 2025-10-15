mod commands;
mod timer;

use std::path::PathBuf;
use clap::{Parser, Subcommand};
use commands::{BenchArgs, HeadArgs};
use tabby_common::config::{config_index_to_id, CodeRepository, HttpModelConfigBuilder, RateLimitBuilder};

pub async fn code_ingest(repository: String) -> Result<String, String> {
    let index_dir = tabby_common::path::index_dir();

    if let Ok(http_config) = HttpModelConfigBuilder::default()
        .kind("ollama/embedding".to_string())
        .api_endpoint(Some("http://localhost:11434".to_string()))
        .model_name(Some("nomic-embed-text".to_string()))
        .rate_limit(
            RateLimitBuilder::default()
                .request_per_minute(6000)
                .build()
                .expect("Failed to create RateLimit"),
        )
        .build(){
        let repo = CodeRepository::new(&repository.to_string(), &config_index_to_id(0));
        let res = commands::run_index_cli(&repo, &http_config).await;

        return res;

        // Err("Failed to index repository".to_string())

        // println!("found: {}",res.as_str());
    }
    else{
        // println!("Failed to create http config");
        Err("Failed to create http config".to_string())
    }


}
pub async fn code_search(query: String, repository: String) -> Result<String, String> {
    let index_dir = tabby_common::path::index_dir();

    if let Ok(http_config) = HttpModelConfigBuilder::default()
        .kind("ollama/embedding".to_string())
        .api_endpoint(Some("http://localhost:11434".to_string()))
        .model_name(Some("nomic-embed-text".to_string()))
        .rate_limit(
            RateLimitBuilder::default()
                .request_per_minute(6000)
                .build()
                .expect("Failed to create RateLimit"),
        )
        .build(){
        let repo = CodeRepository::new(&repository.to_string(), &config_index_to_id(0));
        let res = commands::run_query_cli(&query, &repo, &http_config).await;
        // println!("found: {}",res.as_str());
        // Ok(res.to_string())
        return res
    }
    else{
        println!("Failed to create http config");
        Err("".to_string())
    }



}