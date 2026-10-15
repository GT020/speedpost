use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestItem {
    pub name: String,
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Folder {
    pub name: String,
    #[serde(default)]
    pub folders: Vec<Folder>,
    #[serde(default)]
    pub requests: Vec<RequestItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Collection {
    pub name: String,
    #[serde(default)]
    pub folders: Vec<Folder>,
    #[serde(default)]
    pub requests: Vec<RequestItem>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Collections {
    #[serde(default)]
    pub collections: Vec<Collection>,
}
