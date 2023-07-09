use super::{error::hash_storage_error::HashStorageError, resource::Resource};
use once_cell::sync::Lazy;
use std::{
    collections::{hash_map::Entry::Vacant, HashMap},
    sync::Mutex,
};

static HASHES: Lazy<Mutex<HashMap<String, Resource>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn get_and_remove(key: &str) -> Result<Resource, HashStorageError> {
    match HASHES.lock().expect("Should acquire a lock").remove(key) {
        Some(r) => Ok(r),
        None => Err(HashStorageError::ResourceNotFound),
    }
}

pub fn set(key: &str, value: &Resource) {
    if let Vacant(e) = HASHES
        .lock()
        .expect("Should acquire a lock")
        .entry(key.to_string())
    {
        e.insert(value.clone());
    }
}
