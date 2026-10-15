use std::path::PathBuf;

use crate::model::{Collection, Collections, Folder, HistoryEntry, RequestItem, Settings};
use crate::storage::Storage;

pub struct SqliteStorage {
    path: PathBuf,
}

impl SqliteStorage {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

#[async_trait::async_trait]
impl Storage for SqliteStorage {
    async fn load_history(&self) -> Vec<HistoryEntry> {
        vec![]
    }
    async fn save_history(&self, _history: &[HistoryEntry]) -> anyhow::Result<()> {
        Ok(())
    }
    async fn load_collections(&self) -> Collections {
        Collections::default()
    }
    async fn save_collections(&self, _collections: &Collections) -> anyhow::Result<()> {
        Ok(())
    }
    async fn add_collection(&self, name: &str) -> anyhow::Result<Collection> {
        Ok(Collection {
            name: name.to_string(),
            folders: vec![],
            requests: vec![],
        })
    }
    async fn rename_collection(&self, _collection_id: usize, _name: &str) -> anyhow::Result<()> {
        Ok(())
    }
    async fn delete_collection(&self, _collection_id: usize) -> anyhow::Result<()> {
        Ok(())
    }
    async fn add_folder(
        &self,
        _collection_id: usize,
        _parent_folder_id: Option<usize>,
        name: &str,
    ) -> anyhow::Result<Folder> {
        Ok(Folder {
            name: name.to_string(),
            folders: vec![],
            requests: vec![],
        })
    }
    async fn rename_folder(
        &self,
        _collection_id: usize,
        _folder_id: usize,
        _name: &str,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn delete_folder(&self, _collection_id: usize, _folder_id: usize) -> anyhow::Result<()> {
        Ok(())
    }
    async fn add_request(
        &self,
        _collection_id: usize,
        _folder_id: Option<usize>,
        _req: RequestItem,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn update_request(
        &self,
        _collection_id: usize,
        _folder_id: Option<usize>,
        _request_id: usize,
        _req: RequestItem,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn delete_request(
        &self,
        _collection_id: usize,
        _folder_id: Option<usize>,
        _request_id: usize,
    ) -> anyhow::Result<()> {
        Ok(())
    }
    async fn load_settings(&self) -> Settings {
        Settings::default()
    }
    async fn save_settings(&self, _settings: &Settings) -> anyhow::Result<()> {
        Ok(())
    }
}
