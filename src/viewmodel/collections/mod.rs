#![allow(dead_code)]

pub mod collection;
pub mod folder;
pub mod request_item;

pub use collection::CollectionVm;
pub use folder::FolderVm;
pub use request_item::RequestItemVm;

use crate::model::{Collection, Collections, Folder, RequestItem};

/// Root view model for collections, providing granular state access per node.
#[derive(Default)]
pub struct CollectionsVm {
    collections: Vec<CollectionVm>,
    next_id: u64,
}

impl CollectionsVm {
    pub fn from_collections(collections: Collections) -> Self {
        let mut next_id = 1;
        let mut id_gen = || {
            let id = next_id;
            next_id += 1;
            id
        };

        let vms = collections
            .collections
            .into_iter()
            .map(|c| CollectionVm::new(id_gen(), c, &mut id_gen))
            .collect();

        Self {
            collections: vms,
            next_id,
        }
    }

    pub fn new_with_examples() -> Self {
        let auth_subfolder = Folder {
            name: "Auth".to_string(),
            folders: vec![],
            requests: vec![
                RequestItem {
                    name: "login".to_string(),
                    method: "POST".to_string(),
                    url: "https://httpbin.org/post".to_string(),
                    headers: vec![("Content-Type".to_string(), "application/json".to_string())],
                    body: "{\"username\":\"admin\",\"password\":\"secret\"}".to_string(),
                },
                RequestItem {
                    name: "user info".to_string(),
                    method: "GET".to_string(),
                    url: "https://httpbin.org/get".to_string(),
                    headers: vec![],
                    body: String::new(),
                },
            ],
        };

        let api_folder = Folder {
            name: "HTTPBin APIs".to_string(),
            folders: vec![auth_subfolder],
            requests: vec![RequestItem {
                name: "status check".to_string(),
                method: "GET".to_string(),
                url: "https://httpbin.org/status/200".to_string(),
                headers: vec![],
                body: String::new(),
            }],
        };

        let github_folder = Folder {
            name: "GitHub APIs".to_string(),
            folders: vec![],
            requests: vec![RequestItem {
                name: "github xilem".to_string(),
                method: "GET".to_string(),
                url: "https://api.github.com/repos/linebender/xilem".to_string(),
                headers: vec![],
                body: String::new(),
            }],
        };

        let top_requests = vec![RequestItem {
            name: "quick get".to_string(),
            method: "GET".to_string(),
            url: "https://httpbin.org/get".to_string(),
            headers: vec![],
            body: String::new(),
        }];

        let example_collection = Collection {
            name: "Demo Collection".to_string(),
            folders: vec![api_folder, github_folder],
            requests: top_requests,
        };

        Self::from_collections(Collections {
            collections: vec![example_collection],
        })
    }

    pub fn collections(&self) -> &[CollectionVm] {
        &self.collections
    }

    pub fn collections_mut(&mut self) -> &mut [CollectionVm] {
        &mut self.collections
    }

    pub fn to_collections(&self) -> Collections {
        Collections {
            collections: self.collections.iter().map(|c| c.to_model()).collect(),
        }
    }

    // --- Granular State Access ---

    pub fn is_collection_expanded(&self, id: u64) -> bool {
        self.collections
            .iter()
            .find(|c| c.id == id)
            .map(|c| c.is_expanded)
            .unwrap_or(false)
    }

    pub fn set_collection_expanded(&mut self, id: u64, expanded: bool) {
        if let Some(c) = self.collections.iter_mut().find(|c| c.id == id) {
            c.set_expanded(expanded);
        }
    }

    pub fn toggle_collection(&mut self, id: u64) {
        if let Some(c) = self.collections.iter_mut().find(|c| c.id == id) {
            c.toggle();
        }
    }

    pub fn is_folder_expanded(&self, folder_id: u64) -> bool {
        for coll in &self.collections {
            if let Some(exp) = coll.is_folder_expanded(folder_id) {
                return exp;
            }
        }
        false
    }

    pub fn set_folder_expanded(&mut self, folder_id: u64, expanded: bool) {
        for coll in &mut self.collections {
            if coll.set_folder_expanded(folder_id, expanded) {
                return;
            }
        }
    }

    pub fn toggle_folder(&mut self, folder_id: u64) {
        for coll in &mut self.collections {
            if coll.toggle_folder(folder_id) {
                return;
            }
        }
    }

    pub fn find_request(&self, req_id: u64) -> Option<&RequestItemVm> {
        for coll in &self.collections {
            if let Some(r) = coll.find_request(req_id) {
                return Some(r);
            }
        }
        None
    }

    pub fn find_request_mut(&mut self, req_id: u64) -> Option<&mut RequestItemVm> {
        for coll in &mut self.collections {
            if let Some(r) = coll.find_request_mut(req_id) {
                return Some(r);
            }
        }
        None
    }

    /// Mark `req_id` as the selected request, clearing any other selection.
    pub fn set_selected(&mut self, req_id: u64) {
        for coll in &mut self.collections {
            coll.set_selected(req_id);
        }
    }

    fn alloc_id(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    /// Add a new top-level collection.
    pub fn add_collection(&mut self, name: String) -> u64 {
        let id = self.alloc_id();
        self.collections.push(CollectionVm::empty(id, name));
        id
    }

    /// Find a collection mutably by id.
    pub fn find_collection_mut(&mut self, id: u64) -> Option<&mut CollectionVm> {
        self.collections.iter_mut().find(|c| c.id == id)
    }

    /// Rename an existing collection.
    pub fn rename_collection(&mut self, coll_id: u64, name: String) -> bool {
        if let Some(coll) = self.find_collection_mut(coll_id) {
            coll.set_name(name);
            true
        } else {
            false
        }
    }

    /// Delete a collection by id.
    pub fn delete_collection(&mut self, coll_id: u64) -> bool {
        let before = self.collections.len();
        self.collections.retain(|c| c.id != coll_id);
        before > self.collections.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree_construction_and_expansion() {
        let mut vm = CollectionsVm::new_with_examples();
        assert_eq!(vm.collections().len(), 1);

        let coll_id = vm.collections()[0].id;
        assert!(!vm.is_collection_expanded(coll_id));

        vm.toggle_collection(coll_id);
        assert!(vm.is_collection_expanded(coll_id));

        let parent_folder = &vm.collections()[0].folders[0];
        let parent_folder_id = parent_folder.id;
        assert_eq!(parent_folder.name, "HTTPBin APIs");
        assert!(!vm.is_folder_expanded(parent_folder_id));

        // Subfolder inside parent folder
        let subfolder = &parent_folder.folders[0];
        let subfolder_id = subfolder.id;
        assert_eq!(subfolder.name, "Auth");
        assert!(!vm.is_folder_expanded(subfolder_id));

        // Toggle deeply nested subfolder directly
        vm.toggle_folder(subfolder_id);
        assert!(vm.is_folder_expanded(subfolder_id));
        assert!(!vm.is_folder_expanded(parent_folder_id));

        // Toggle parent folder
        vm.toggle_folder(parent_folder_id);
        assert!(vm.is_folder_expanded(parent_folder_id));
    }

    #[test]
    fn test_find_deep_nested_request() {
        let vm = CollectionsVm::new_with_examples();
        let auth_subfolder = &vm.collections()[0].folders[0].folders[0];
        let login_req_id = auth_subfolder.requests[0].id;

        let found = vm
            .find_request(login_req_id)
            .expect("Should find login request");
        assert_eq!(found.name, "login");
        assert_eq!(found.method, "POST");
        assert_eq!(found.url, "https://httpbin.org/post");
    }

    #[test]
    fn test_to_collections_roundtrip() {
        let vm = CollectionsVm::new_with_examples();
        let model = vm.to_collections();
        assert_eq!(model.collections.len(), 1);
        assert_eq!(model.collections[0].name, "Demo Collection");
        assert_eq!(model.collections[0].folders.len(), 2);
        assert_eq!(model.collections[0].folders[0].folders.len(), 1);
    }

    #[test]
    fn test_add_and_delete_collection() {
        let mut vm = CollectionsVm::new_with_examples();
        assert_eq!(vm.collections().len(), 1);

        let new_id = vm.add_collection("New Collection".to_string());
        assert_eq!(vm.collections().len(), 2);
        assert_eq!(vm.collections()[1].name(), "New Collection");

        assert!(vm.rename_collection(new_id, "Renamed".to_string()));
        assert_eq!(vm.collections()[1].name(), "Renamed");

        assert!(vm.delete_collection(new_id));
        assert_eq!(vm.collections().len(), 1);
        assert!(!vm.delete_collection(new_id));
    }
}
