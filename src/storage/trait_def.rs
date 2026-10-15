use crate::model::{Collection, Collections, Folder, HistoryEntry, RequestItem, Settings};

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    async fn load_history(&self) -> Vec<HistoryEntry>;
    async fn save_history(&self, history: &[HistoryEntry]) -> anyhow::Result<()>;

    async fn load_collections(&self) -> Collections;
    async fn save_collections(&self, collections: &Collections) -> anyhow::Result<()>;

    async fn add_collection(&self, name: &str) -> anyhow::Result<Collection>;
    async fn rename_collection(&self, collection_id: usize, name: &str) -> anyhow::Result<()>;
    async fn delete_collection(&self, collection_id: usize) -> anyhow::Result<()>;

    async fn add_folder(
        &self,
        collection_id: usize,
        parent_folder_id: Option<usize>,
        name: &str,
    ) -> anyhow::Result<Folder>;
    async fn rename_folder(
        &self,
        collection_id: usize,
        folder_id: usize,
        name: &str,
    ) -> anyhow::Result<()>;
    async fn delete_folder(&self, collection_id: usize, folder_id: usize) -> anyhow::Result<()>;

    async fn add_request(
        &self,
        collection_id: usize,
        folder_id: Option<usize>,
        req: RequestItem,
    ) -> anyhow::Result<()>;
    async fn update_request(
        &self,
        collection_id: usize,
        folder_id: Option<usize>,
        request_id: usize,
        req: RequestItem,
    ) -> anyhow::Result<()>;
    async fn delete_request(
        &self,
        collection_id: usize,
        folder_id: Option<usize>,
        request_id: usize,
    ) -> anyhow::Result<()>;

    async fn load_settings(&self) -> Settings;
    async fn save_settings(&self, settings: &Settings) -> anyhow::Result<()>;
}
