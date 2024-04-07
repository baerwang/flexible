use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
    pub plugin: String,
    pub token: String,
    pub reviews: Vec<String>,
    pub owners: Vec<Owner>,
    pub orgs: HashMap<String, Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub name: String,
    pub repos: Vec<String>,
}
