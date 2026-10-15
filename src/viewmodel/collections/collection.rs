#![allow(dead_code)]

use super::folder::FolderVm;
use super::request_item::RequestItemVm;
use crate::model::Collection;

/// Granular view model representing a top-level collection node.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CollectionVm {
    pub id: u64,
    pub name: String,
    pub is_expanded: bool,
    pub folders: Vec<FolderVm>,
    pub requests: Vec<RequestItemVm>,
}

impl CollectionVm {
    pub fn new(id: u64, coll: Collection, id_gen: &mut impl FnMut() -> u64) -> Self {
        let folders = coll
            .folders
            .into_iter()
            .map(|f| FolderVm::new(id_gen(), f, id_gen))
            .collect();

        let requests = coll
            .requests
            .into_iter()
            .map(|r| RequestItemVm::new(id_gen(), r))
            .collect();

        Self {
            id,
            name: coll.name,
            is_expanded: false,
            folders,
            requests,
        }
    }

    pub fn toggle(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    /// Build an empty collection with a generated id.
    pub fn empty(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            is_expanded: true,
            folders: Vec::new(),
            requests: Vec::new(),
        }
    }

    // -- Reads -------------------------------------------------------------

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_expanded(&self) -> bool {
        self.is_expanded
    }

    pub fn folders(&self) -> &[FolderVm] {
        &self.folders
    }

    pub fn requests(&self) -> &[RequestItemVm] {
        &self.requests
    }

    // -- Commands ----------------------------------------------------------

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn add_folder(&mut self, id: u64, name: String) -> u64 {
        self.folders.push(FolderVm::empty(id, name));
        id
    }

    pub fn add_request(&mut self, id: u64, name: String, method: String, url: String) -> u64 {
        self.requests
            .push(RequestItemVm::blank(id, name, method, url));
        id
    }

    pub fn set_expanded(&mut self, expanded: bool) {
        self.is_expanded = expanded;
    }

    pub fn toggle_folder(&mut self, folder_id: u64) -> bool {
        for folder in &mut self.folders {
            if folder.toggle_folder(folder_id) {
                return true;
            }
        }
        false
    }

    pub fn set_folder_expanded(&mut self, folder_id: u64, expanded: bool) -> bool {
        for folder in &mut self.folders {
            if folder.set_folder_expanded(folder_id, expanded) {
                return true;
            }
        }
        false
    }

    pub fn is_folder_expanded(&self, folder_id: u64) -> Option<bool> {
        for folder in &self.folders {
            if let Some(exp) = folder.is_folder_expanded(folder_id) {
                return Some(exp);
            }
        }
        None
    }

    pub fn find_request(&self, req_id: u64) -> Option<&RequestItemVm> {
        if let Some(req) = self.requests.iter().find(|r| r.id == req_id) {
            return Some(req);
        }
        for folder in &self.folders {
            if let Some(req) = folder.find_request(req_id) {
                return Some(req);
            }
        }
        None
    }

    pub fn find_request_mut(&mut self, req_id: u64) -> Option<&mut RequestItemVm> {
        if let Some(pos) = self.requests.iter().position(|r| r.id == req_id) {
            return self.requests.get_mut(pos);
        }
        for folder in &mut self.folders {
            if let Some(req) = folder.find_request_mut(req_id) {
                return Some(req);
            }
        }
        None
    }

    pub fn find_folder_mut(&mut self, folder_id: u64) -> Option<&mut FolderVm> {
        for folder in &mut self.folders {
            if folder.id == folder_id {
                return Some(folder);
            }
            if let Some(found) = folder.find_folder_mut(folder_id) {
                return Some(found);
            }
        }
        None
    }

    /// Mark the request `req_id` as selected and clear every other selection.
    pub fn set_selected(&mut self, req_id: u64) {
        for req in &mut self.requests {
            req.set_selected(req.id == req_id);
        }
        for folder in &mut self.folders {
            folder.set_selected(req_id);
        }
    }

    pub fn to_model(&self) -> Collection {
        Collection {
            name: self.name.clone(),
            folders: self.folders.iter().map(|f| f.to_model()).collect(),
            requests: self.requests.iter().map(|r| r.to_model()).collect(),
        }
    }
}
