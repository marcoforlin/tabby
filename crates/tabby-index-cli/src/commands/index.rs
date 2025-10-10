
use tabby_common::config::{CodeRepository, HttpModelConfig};
use tabby_index::public::CodeIndexer;

use std::sync::Arc;
use http_api_bindings::create_embedding;
use tabby_inference::Embedding;

pub async fn run_index_cli(repository: &CodeRepository, http_model_config: &HttpModelConfig) -> Result<(),()> {
    let mut code = CodeIndexer::default();
    let emb: Arc<dyn Embedding> = create_embedding(http_model_config).await;
    if let Err(err) = code.refresh(emb.clone(), repository).await {
        println!("Failed to refresh code index: {}", err);
        return Err(());
    }

    // if let Err(err) = index_commits::refresh(embedding, &repository).await {
    //     // logkit::warn!("Failed to refresh commit index: {}", err);
    //     return Err(err);
    // }

    Ok(())
}