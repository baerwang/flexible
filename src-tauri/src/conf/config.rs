/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
    pub plugin: String,
    pub token: String,
    pub reviews: Vec<String>,
    pub owners: Owner,
    pub orgs: HashMap<String, Vec<String>>,
    pub dispatch: u64,
}

impl ConfigData {
    pub fn new(plugin: &str, token: &str) -> Self {
        ConfigData {
            plugin: plugin.to_string(),
            token: token.to_string(),
            reviews: vec![],
            owners: Owner {
                name: "".to_string(),
                repos: vec![],
            },
            orgs: HashMap::new(),
            dispatch: 0,
        }
    }

    pub fn new_owner(plugin: &str, token: &str, owner: Owner) -> Self {
        ConfigData {
            plugin: plugin.to_string(),
            token: token.to_string(),
            reviews: vec![],
            owners: owner,
            orgs: HashMap::new(),
            dispatch: 0,
        }
    }

    pub fn valid(&self) -> &str {
        if self.token.is_empty() {
            "token not allowed empty"
        } else if self.plugin.is_empty() {
            "plugin not allowed empty"
        } else if self.dispatch == 0 {
            "dispatch not allowed empty"
        } else if self.reviews.is_empty() || self.reviews.iter().any(|s| s.is_empty()) {
            "reviews not allowed empty"
        } else if self.orgs.is_empty()
            && (self.owners.name.is_empty()
                || self.owners.repos.is_empty()
                || self.owners.repos.iter().any(|s| s.is_empty()))
        {
            "owner/repos or orgs/repos not allowed empty"
        } else {
            ""
        }
    }

    pub fn reviews(&self) -> HashMap<String, ()> {
        self.reviews.iter().map(|key| (key.clone(), ())).collect()
    }
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub name: String,
    pub repos: Vec<String>,
}
