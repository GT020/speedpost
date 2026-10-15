use std::path::PathBuf;

use crate::model::{Collection, Collections, Folder, HistoryEntry, RequestItem, Settings};
use crate::storage::Storage;

pub struct JsonFileStorage {
    path: PathBuf,
    collections_path: PathBuf,
    settings_path: PathBuf,
}

impl JsonFileStorage {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let p = path.into();
        let base = p.parent().unwrap_or(std::path::Path::new("."));
        Self {
            path: p.clone(),
            collections_path: base.join("collections.json"),
            settings_path: base.join("settings.json"),
        }
    }
}

#[async_trait::async_trait]
impl Storage for JsonFileStorage {
    async fn load_history(&self) -> Vec<HistoryEntry> {
        if !self.path.exists() {
            return vec![];
        }
        let data = tokio::fs::read(&self.path).await.unwrap_or_default();
        serde_json::from_slice(&data).unwrap_or_default()
    }

    async fn save_history(&self, history: &[HistoryEntry]) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let data = serde_json::to_vec_pretty(history)?;
        tokio::fs::write(&self.path, data).await?;
        Ok(())
    }

    async fn load_collections(&self) -> Collections {
        if !self.collections_path.exists() {
            return Collections::default();
        }
        let data = tokio::fs::read(&self.collections_path)
            .await
            .unwrap_or_default();
        serde_json::from_slice(&data).unwrap_or_default()
    }

    async fn save_collections(&self, collections: &Collections) -> anyhow::Result<()> {
        if let Some(parent) = self.collections_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let data = serde_json::to_vec_pretty(collections)?;
        tokio::fs::write(&self.collections_path, data).await?;
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
        if !self.settings_path.exists() {
            return Settings::default();
        }
        let data = tokio::fs::read(&self.settings_path)
            .await
            .unwrap_or_default();
        serde_json::from_slice(&data).unwrap_or_default()
    }
    async fn save_settings(&self, settings: &Settings) -> anyhow::Result<()> {
        if let Some(parent) = self.settings_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
        let data = serde_json::to_vec_pretty(settings)?;
        tokio::fs::write(&self.settings_path, data).await?;
        Ok(())
    }
}
