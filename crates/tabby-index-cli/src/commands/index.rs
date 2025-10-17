use std::fs::File;
use tabby_common::config::{config_index_to_id, CodeRepository, HttpModelConfig};
use tabby_index::public::CodeIndexer;

use std::sync::Arc;
use csv::{Reader, Writer};
use http_api_bindings::create_embedding;
use tabby_common::path;
use tabby_inference::Embedding;
use crate::Row;
use std::path::PathBuf;
use serde::Serialize;

pub async fn run_index_cli(repository: &CodeRepository, http_model_config: &HttpModelConfig) -> Result<String,String> {
    let mut code = CodeIndexer::default();
    let emb: Arc<dyn Embedding> = create_embedding(http_model_config).await;
    if let Err(err) = code.refresh(emb.clone(), repository).await {
        println!("Failed to refresh code index: {}", err);
        return Err("Error:".to_string() + &err.to_string());
    }

    // if let Err(err) = index_commits::refresh(embedding, &repository).await {
    //     // logkit::warn!("Failed to refresh commit index: {}", err);
    //     return Err(err);
    // }

    Ok(repository.git_url.clone() + " indexed")
}

pub fn get_list_dir() -> PathBuf{
    path::repositories_dir().join("list.csv")
}

pub fn list_repos() -> String{
    let repo_dir = get_list_dir();
    if repo_dir.exists() {
        // return anyhow::Error::msg("Repository directory does not exist");

        let mut reader = Reader::from_path(repo_dir.clone()).unwrap();
        let records = reader.deserialize::<Row>();
        return records.map(|row| row.unwrap()).map(|row|
            ("link: ".to_string() + &row.repo.clone() + ", id:" + &row.id.clone() + "\n")).collect();

    }
    return "".to_string()
}

pub fn get_from_list(repository: String) -> Vec<Row> {
    let repo_dir = get_list_dir();
    if repo_dir.exists() {
        // return anyhow::Error::msg("Repository directory does not exist");

        let mut reader = Reader::from_path(repo_dir.clone()).unwrap();
        let records = reader.deserialize::<Row>();
        let mut repos = Vec::new();
        for record in records {
            let rec:Row = record.unwrap();
            if rec.repo == repository {
                println!("repository already indexed!");
                return repos;
            }
            repos.push(rec);
        }
        return repos;
        /* `std::vec::Vec<Row>` value */


    }
    else {
        let mut file = File::create(repo_dir.clone());
        return Vec::new();
    }// Commands::Index => {

}

pub fn add_to_list(repos: Vec<Row>, repository: &CodeRepository) -> Result<(), ()> {
    let repo_dir = get_list_dir();
    let mut writer = Writer::from_path(repo_dir).unwrap();
    // let mut new_repos:Vec<Row>= Vec::new();
    let len = repos.len();


    for record in repos{
        writer.serialize(Row {
            repo: record.repo.clone().to_string(),
            id: config_index_to_id(len).to_string(),
        }).unwrap();
      //  let rec = vec![, record.id.clone().to_string()];
       // writer.serialize(rec).unwrap();
    }
//     repos.clone_into(&mut new_repos);

    writer.serialize(Row {
        repo: repository.git_url.clone().to_string(),
        id: config_index_to_id(len + 1).to_string(),
    }).unwrap();

    Ok(())

}