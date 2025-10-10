use tabby_common::config::{CodeRepository, HttpModelConfig};
use std::sync::Arc;
use http_api_bindings::create_embedding;
use tabby_inference::Embedding;
use tabby_common::api::code::{CodeSearch, CodeSearchError, CodeSearchParams,CodeSearchQuery};

use crate::commands::code_search;
use crate::commands::tantivy_provider;

pub async fn run_query_cli(query: &String, repository: &CodeRepository, http_model_config: &HttpModelConfig) -> Result<String,()> {
    let emb: Arc<dyn Embedding> = create_embedding(&http_model_config.clone()).await;
    let index_reader_provider: Arc<tantivy_provider::IndexReaderProvider> = Arc::new(tantivy_provider::IndexReaderProvider::default());

    // let read = index_reader_provider.reader().await;

    let imp = Arc::new(code_search::create_code_search(emb.clone(), index_reader_provider.clone()));


    let params = CodeSearchParams{
        min_embedding_score: 0.7,
        min_bm25_score: 6.0,
        min_rrf_score: 0.028,
        num_to_return: 40,
        num_to_score: 100,
    };


    //CodeSearchParams::default().with_min_bm25_score(7).with_min_rrf_score(0.5);

    loop
    {
        let query = CodeSearchQuery::new(
            None,
            Some("rust".to_owned()),
            query.clone().to_owned(),
            repository.source_id.to_owned(),
        );
        let ret = imp.search_in_language(query, params.clone()).await;
        match ret
        {
            Ok(ret) => {
                let res:Vec<String> = ret.hits.chunks(1).flatten().map(|hit| hit.doc.body.clone()).collect();
                return Ok((res.join("\n--------------------------------\n")));
            }
            Err(CodeSearchError::NotReady) => {
                // warn!("Failed to search: {}", err);
                continue;
            }
            Err(CodeSearchError::TantivyError(err)) => {
                // warn!("Failed to search: {}", err);
                return Ok((err.to_string()));
            }
            Err(CodeSearchError::QueryParserError(err)) => {
                // warn!("Failed to parse query: {}", err);
                return Ok((err.to_string()));
            }
            Err(CodeSearchError::Other(err)) => {
                // warn!("Failed to search: {}", err);
                return Ok((err.to_string()));
            }
        }
    }
}