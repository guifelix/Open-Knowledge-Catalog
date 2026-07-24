#![allow(dead_code)]

use crate::model::document::{
    HeadingInfo, IndexStats, Link, LinkInfo, MetadataQueryResponse, ParseError, SearchResponse,
    ValidationIssue,
};
use crate::model::graph::TraverseResponse;
use std::collections::{BTreeMap, HashMap};

pub struct SearchableDocument {
    pub path: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub headings: String,
    pub body: String,
    pub concept_type: Option<String>,
}

pub struct SearchFilters {
    pub path_prefix: Option<String>,
    pub concept_types: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
}

pub type Result<T> = std::result::Result<T, anyhow::Error>;

pub trait DocumentStore: Send + Sync {
    fn init(&self) -> Result<()>;

    fn upsert_document(&self, doc: &DocumentRecord) -> Result<()>;
    fn get_document(&self, path: &str) -> Result<Option<DocumentRecord>>;
    fn delete_document(&self, path: &str) -> Result<()>;
    fn list_documents(
        &self,
        path_prefix: Option<&str>,
        limit: usize,
    ) -> Result<Vec<DocumentRecord>>;

    fn insert_tags(&self, doc_id: i64, tags: &[String]) -> Result<()>;
    fn get_tags(&self, doc_id: i64) -> Result<Vec<String>>;
    fn delete_tags(&self, doc_id: i64) -> Result<()>;

    fn insert_headings(&self, doc_id: i64, headings: &[HeadingInfo]) -> Result<()>;
    fn get_headings(&self, doc_id: i64) -> Result<Vec<HeadingInfo>>;
    fn delete_headings(&self, doc_id: i64) -> Result<()>;

    fn insert_links(&self, doc_id: i64, links: &[LinkInfo]) -> Result<()>;
    fn get_links(&self, doc_id: i64) -> Result<Vec<LinkInfo>>;
    fn delete_links(&self, doc_id: i64) -> Result<()>;

    fn insert_metadata_fields(
        &self,
        doc_id: i64,
        fields: &BTreeMap<String, serde_json::Value>,
    ) -> Result<()>;
    fn get_metadata_fields(&self, doc_id: i64) -> Result<BTreeMap<String, serde_json::Value>>;
    fn delete_metadata_fields(&self, doc_id: i64) -> Result<()>;

    fn insert_scan_errors(&self, path: &str, errors: &[ParseError]) -> Result<()>;
    fn get_scan_errors(&self, path: &str) -> Result<Vec<ParseError>>;
    fn delete_scan_errors(&self, path: &str) -> Result<()>;

    fn query_metadata(
        &self,
        filters: &HashMap<String, String>,
        select: &[String],
        limit: usize,
    ) -> Result<MetadataQueryResponse>;
    fn get_stats(&self) -> Result<IndexStats>;
}

pub trait SearchIndex: Send + Sync {
    fn init(&self) -> Result<()>;
    fn index_document(&self, doc: &SearchableDocument) -> Result<()>;
    fn remove_document(&self, path: &str) -> Result<()>;
    fn search(&self, query: &str, filters: &SearchFilters, limit: usize) -> Result<SearchResponse>;
    fn stats(&self) -> Result<IndexStats>;
}

pub trait GraphStore: Send + Sync {
    fn init(&self) -> Result<()>;
    fn store_links(&self, source_path: &str, links: &[Link]) -> Result<()>;
    fn remove_links(&self, source_path: &str) -> Result<()>;
    fn get_links(&self, path: &str) -> Result<Vec<LinkInfo>>;
    fn get_backlinks(&self, path: &str, limit: usize) -> Result<Vec<LinkInfo>>;
    fn traverse(
        &self,
        start: &str,
        relations: &[String],
        max_depth: usize,
        max_nodes: usize,
    ) -> Result<TraverseResponse>;
    fn validate_links(&self) -> Result<Vec<ValidationIssue>>;
    fn detect_circular_references(&self) -> Result<Vec<ValidationIssue>>;
}

#[derive(Debug, Clone)]
pub struct DocumentRecord {
    pub id: i64,
    pub path: String,
    pub parent_path: String,
    pub title: Option<String>,
    pub concept_type: Option<String>,
    pub description: Option<String>,
    pub body_text: String,
    pub file_size: u64,
    pub modified_at: i64,
    pub content_hash: String,
    pub parse_status: String,
}
