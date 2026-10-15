use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Settings {
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub history_limit: usize,
}
