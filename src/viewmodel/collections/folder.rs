#![allow(dead_code)]

use super::request_item::RequestItemVm;
use crate::model::Folder;

/// Granular view model representing a folder node (can contain subfolders and requests).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FolderVm {
    pub id: u64,
    pub name: String,
    pub is_expanded: bool,
    pub folders: Vec<FolderVm>,
    pub requests: Vec<RequestItemVm>,
}

impl FolderVm {
    pub fn new(id: u64, folder: Folder, id_gen: &mut impl FnMut() -> u64) -> Self {
        let sub_folders = folder
            .folders
            .into_iter()
            .map(|f| FolderVm::new(id_gen(), f, id_gen))
            .collect();

        let reqs = folder
            .requests
            .into_iter()
            .map(|r| RequestItemVm::new(id_gen(), r))
            .collect();

        Self {
            id,
            name: folder.name,
            is_expanded: false,
            folders: sub_folders,
            requests: reqs,
        }
    }

    pub fn toggle(&mut self) {
        self.is_expanded = !self.is_expanded;
    }

    /// Build an empty folder with a generated id.
    pub fn empty(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            is_expanded: false,
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

    /// Recursively search and toggle the folder with `target_id`.
    pub fn toggle_folder(&mut self, target_id: u64) -> bool {
        if self.id == target_id {
            self.toggle();
            return true;
        }
        for sub in &mut self.folders {
            if sub.toggle_folder(target_id) {
                return true;
            }
        }
        false
    }

    /// Recursively search and set expansion for the folder with `target_id`.
    pub fn set_folder_expanded(&mut self, target_id: u64, expanded: bool) -> bool {
        if self.id == target_id {
            self.set_expanded(expanded);
            return true;
        }
        for sub in &mut self.folders {
            if sub.set_folder_expanded(target_id, expanded) {
                return true;
            }
        }
        false
    }

    /// Check if target folder is expanded.
    pub fn is_folder_expanded(&self, target_id: u64) -> Option<bool> {
        if self.id == target_id {
            return Some(self.is_expanded);
        }
        for sub in &self.folders {
            if let Some(exp) = sub.is_folder_expanded(target_id) {
                return Some(exp);
            }
        }
        None
    }

    /// Find request by ID recursively.
    pub fn find_request(&self, target_id: u64) -> Option<&RequestItemVm> {
        if let Some(req) = self.requests.iter().find(|r| r.id == target_id) {
            return Some(req);
        }
        for sub in &self.folders {
            if let Some(req) = sub.find_request(target_id) {
                return Some(req);
            }
        }
        None
    }

    /// Find request by ID recursively, mutably.
    pub fn find_request_mut(&mut self, target_id: u64) -> Option<&mut RequestItemVm> {
        if let Some(pos) = self.requests.iter().position(|r| r.id == target_id) {
            return self.requests.get_mut(pos);
        }
        for sub in &mut self.folders {
            if let Some(req) = sub.find_request_mut(target_id) {
                return Some(req);
            }
        }
        None
    }

    /// Find a descendant folder (including none for self) by ID, mutably.
    pub fn find_folder_mut(&mut self, target_id: u64) -> Option<&mut FolderVm> {
        for sub in &mut self.folders {
            if sub.id == target_id {
                return Some(sub);
            }
            if let Some(found) = sub.find_folder_mut(target_id) {
                return Some(found);
            }
        }
        None
    }

    /// Mark the request `target_id` as selected and clear every other selection.
    pub fn set_selected(&mut self, target_id: u64) {
        for req in &mut self.requests {
            req.set_selected(req.id == target_id);
        }
        for sub in &mut self.folders {
            sub.set_selected(target_id);
        }
    }

    pub fn to_model(&self) -> Folder {
        Folder {
            name: self.name.clone(),
            folders: self.folders.iter().map(|f| f.to_model()).collect(),
            requests: self.requests.iter().map(|r| r.to_model()).collect(),
        }
    }
}
