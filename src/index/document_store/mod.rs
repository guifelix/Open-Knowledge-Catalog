//! SQLite-backed document storage implementation.
//!
//! This module provides [`SqliteDocumentStore`], an implementation of the
//! [`DocumentStore`] trait that persists documents, headings, links, tags,
//! and metadata fields to a SQLite database.
//!
//! The store uses a connection pool (r2d2) for thread-safe access to the
//! database. All operations acquire a connection from the pool, ensuring
//! thread safety while allowing concurrent reads through SQLite's WAL mode.

pub mod code_blocks;
pub mod documents;
pub mod errors;
pub mod headings;
pub mod links;
pub mod metadata;
pub mod stats;
pub mod tables;
pub mod tags;

use crate::index::traits::{DocumentRecord, DocumentStore, Result};
use crate::model::document::{
    HeadingInfo, IndexStats, LinkInfo, MetadataQueryResponse, ParseError,
};
use rusqlite::{params, OptionalExtension, Transaction};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

/// SQLite-backed document store with thread-safe connection pool.
///
/// Implements the [`DocumentStore`] trait for persistent document storage.
/// All operations acquire a connection from the pool, ensuring thread safety.
pub struct SqliteDocumentStore {
    pool: Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>,
}

impl SqliteDocumentStore {
    /// Create a new document store with the given connection pool.
    #[allow(dead_code)]
    pub fn new(pool: Arc<r2d2::Pool<r2d2_sqlite::SqliteConnectionManager>>) -> Self {
        Self { pool }
    }

    fn get_conn(&self) -> Result<r2d2::PooledConnection<r2d2_sqlite::SqliteConnectionManager>> {
        Ok(self.pool.get()?)
    }
}

impl DocumentStore for SqliteDocumentStore {
    fn init(&self) -> Result<()> {
        let conn = self.get_conn()?;
        documents::init(&conn)
    }

    fn upsert_document(&self, doc: &DocumentRecord) -> Result<()> {
        let conn = self.get_conn()?;
        documents::upsert_document(&conn, doc)
    }

    fn upsert_document_tx(&self, tx: &Transaction, doc: &DocumentRecord) -> Result<()> {
        documents::upsert_document_tx(tx, doc)
    }

    fn get_document(&self, path: &str, root_id: Option<i64>) -> Result<Option<DocumentRecord>> {
        let conn = self.get_conn()?;
        documents::get_document(&conn, path, root_id)
    }

    fn delete_document(&self, path: &str, root_id: Option<i64>) -> Result<()> {
        let conn = self.get_conn()?;
        documents::delete_document(&conn, path, root_id)
    }

    fn delete_document_tx(&self, tx: &Transaction, path: &str, root_id: Option<i64>) -> Result<()> {
        documents::delete_document_tx(tx, path, root_id)
    }

    fn get_doc_id_tx(&self, tx: &Transaction, path: &str, root_id: Option<i64>) -> Result<i64> {
        documents::get_doc_id_tx(tx, path, root_id)
    }

    fn list_documents(
        &self,
        path_prefix: Option<&str>,
        limit: usize,
        root_id: Option<i64>,
    ) -> Result<Vec<DocumentRecord>> {
        let conn = self.get_conn()?;
        documents::list_documents(&conn, path_prefix, Some(limit), root_id)
    }

    fn insert_tags(&self, doc_id: i64, tags: &[String]) -> Result<()> {
        let conn = self.get_conn()?;
        tags::insert_tags(&conn, doc_id, tags)
    }

    fn insert_tags_tx(&self, tx: &Transaction, doc_id: i64, tags: &[String]) -> Result<()> {
        tags::insert_tags_tx(tx, doc_id, tags)
    }

    fn get_tags(&self, doc_id: i64) -> Result<Vec<String>> {
        let conn = self.get_conn()?;
        tags::get_tags(&conn, doc_id)
    }

    fn delete_tags(&self, doc_id: i64) -> Result<()> {
        let conn = self.get_conn()?;
        tags::delete_tags(&conn, doc_id)
    }

    fn insert_headings(&self, doc_id: i64, headings: &[HeadingInfo]) -> Result<()> {
        let conn = self.get_conn()?;
        headings::insert_headings(&conn, doc_id, headings)
    }

    fn insert_headings_tx(
        &self,
        tx: &Transaction,
        doc_id: i64,
        headings: &[HeadingInfo],
    ) -> Result<()> {
        headings::insert_headings_tx(tx, doc_id, headings)
    }

    fn get_headings(&self, doc_id: i64) -> Result<Vec<HeadingInfo>> {
        let conn = self.get_conn()?;
        headings::get_headings(&conn, doc_id)
    }

    fn delete_headings(&self, doc_id: i64) -> Result<()> {
        let conn = self.get_conn()?;
        headings::delete_headings(&conn, doc_id)
    }

    fn insert_tables(&self, doc_id: i64, tables: &[crate::model::document::Table]) -> Result<()> {
        let conn = self.get_conn()?;
        tables::insert_tables(&conn, doc_id, tables)
    }

    fn get_tables(&self, doc_id: i64) -> Result<Vec<crate::model::document::Table>> {
        let conn = self.get_conn()?;
        tables::get_tables(&conn, doc_id)
    }

    fn delete_tables(&self, doc_id: i64) -> Result<()> {
        let conn = self.get_conn()?;
        tables::delete_tables(&conn, doc_id)
    }

    fn insert_code_blocks(
        &self,
        doc_id: i64,
        code_blocks: &[crate::model::document::CodeBlock],
    ) -> Result<()> {
        let conn = self.get_conn()?;
        code_blocks::insert_code_blocks(&conn, doc_id, code_blocks)
    }

    fn get_code_blocks(&self, doc_id: i64) -> Result<Vec<crate::model::document::CodeBlock>> {
        let conn = self.get_conn()?;
        code_blocks::get_code_blocks(&conn, doc_id)
    }

    fn delete_code_blocks(&self, doc_id: i64) -> Result<()> {
        let conn = self.get_conn()?;
        code_blocks::delete_code_blocks(&conn, doc_id)
    }

    fn insert_links(&self, doc_id: i64, links: &[LinkInfo]) -> Result<()> {
        let conn = self.get_conn()?;
        links::insert_links(&conn, doc_id, links)
    }

    fn insert_links_tx(&self, tx: &Transaction, doc_id: i64, links: &[LinkInfo]) -> Result<()> {
        links::insert_links_tx(tx, doc_id, links)
    }

    fn get_links(&self, doc_id: i64) -> Result<Vec<LinkInfo>> {
        let conn = self.get_conn()?;
        links::get_links(&conn, doc_id)
    }

    fn delete_links(&self, doc_id: i64) -> Result<()> {
        let conn = self.get_conn()?;
        links::delete_links(&conn, doc_id)
    }

    fn insert_metadata_fields(
        &self,
        doc_id: i64,
        fields: &BTreeMap<String, serde_json::Value>,
    ) -> Result<()> {
        let conn = self.get_conn()?;
        metadata::insert_metadata_fields(&conn, doc_id, fields)
    }

    fn insert_metadata_fields_tx(
        &self,
        tx: &Transaction,
        doc_id: i64,
        fields: &BTreeMap<String, serde_json::Value>,
    ) -> Result<()> {
        metadata::insert_metadata_fields_tx(tx, doc_id, fields)
    }

    fn get_metadata_fields(&self, doc_id: i64) -> Result<BTreeMap<String, serde_json::Value>> {
        let conn = self.get_conn()?;
        metadata::get_metadata_fields(&conn, doc_id)
    }

    fn delete_metadata_fields(&self, doc_id: i64) -> Result<()> {
        let conn = self.get_conn()?;
        metadata::delete_metadata_fields(&conn, doc_id)
    }

    fn insert_scan_errors(&self, path: &str, errors: &[ParseError]) -> Result<()> {
        let conn = self.get_conn()?;
        errors::insert_scan_errors(&conn, path, errors)
    }

    fn insert_scan_errors_tx(
        &self,
        tx: &Transaction,
        path: &str,
        errors: &[ParseError],
    ) -> Result<()> {
        errors::insert_scan_errors_tx(tx, path, errors)
    }

    fn get_scan_errors(&self, path: &str) -> Result<Vec<ParseError>> {
        let conn = self.get_conn()?;
        errors::get_scan_errors(&conn, path)
    }

    fn delete_scan_errors(&self, path: &str) -> Result<()> {
        let conn = self.get_conn()?;
        errors::delete_scan_errors(&conn, path)
    }

    fn query_metadata(
        &self,
        _filters: &HashMap<String, String>,
        _select: &[String],
        _limit: usize,
    ) -> Result<MetadataQueryResponse> {
        let conn = self.get_conn()?;
        stats::query_metadata(&conn, _filters, _select, _limit)
    }

    fn get_stats(&self) -> Result<IndexStats> {
        let conn = self.get_conn()?;
        stats::get_stats(&conn)
    }
}
