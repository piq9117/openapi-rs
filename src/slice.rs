use std::collections::HashMap;
use std::fs;
use std::io;

use serde::{Deserialize, Serialize};
use serde_yaml;

use crate::openapi::{Component, Info, OpenApi, PathItem, Server};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct OpenApiSlice {
    pub openapi: String,
    pub info: Info,
    pub servers: Vec<Server>,
    pub path: HashMap<String, PathItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub components: Option<Component>,
}

#[allow(dead_code)]
pub fn get_path<'a>(spec: &'a OpenApi, pathname: &str) -> OpenApiSlice {
    let path = spec.paths.get(pathname);
    let mut path_item_slice: HashMap<String, PathItem> = HashMap::new();

    if let Some(path_item) = path {
        path_item_slice.insert(pathname.to_string(), path_item.clone());
    }

    OpenApiSlice {
        openapi: spec.openapi.clone(),
        info: spec.info.clone(),
        servers: spec.servers.clone(),
        path: path_item_slice,
        components: spec.components.clone(),
    }
}

#[allow(dead_code)]
pub fn write_slice_to_file<'a>(path_item: &'a OpenApiSlice, filename: &str) -> io::Result<()> {
    match serde_yaml::to_string(path_item) {
        Err(_) => panic!("Unable to decode path item"),
        Ok(serialized) => fs::write(filename, serialized),
    }
}
