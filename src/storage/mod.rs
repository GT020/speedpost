pub mod json_file;
pub mod trait_def;

pub use json_file::JsonFileStorage;
pub use trait_def::Storage;
#[cfg(feature = "sqlite")]
pub mod sqlite;
