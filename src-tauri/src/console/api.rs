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

use crate::console::model::Repos;
use crate::plugins::{client, get_api};
use crate::{conf, dispatch};

#[tauri::command]
pub async fn create(conf: conf::config::ConfigData) -> String {
    match conf.valid() {
        "" => {
            _ = dispatch::execute(conf).await;
            "".to_string()
        }
        r => r.to_string(),
    }
}

#[tauri::command]
pub async fn repos(conf: conf::config::ConfigData) -> Result<Vec<Repos>, anyhow::Error> {
    let api = get_api(
        conf.plugin.as_str(),
        conf.owners.name.clone(),
        conf.reviews(),
    );
    let rsp = client(api.repos(), api.headers(conf.token.as_str())).await?;
    let repos: Vec<Repos> = serde_json::from_str(&rsp)?;
    Ok(repos)
}

#[tauri::command]
#[allow(clippy::unused_unit)]
pub fn orgs(_conf: conf::config::ConfigData) -> () {}

#[tauri::command]
#[allow(clippy::unused_unit)]
pub fn org_repos(_org: &str) -> () {}

#[cfg(test)]
mod test {
    use crate::conf::config::{ConfigData, Owner};
    use crate::console::api::repos;

    #[tokio::test]
    async fn test_repos() {
        let result = repos(ConfigData::new_owner(
            "github",
            "",
            Owner {
                name: "baerwang".to_string(),
                repos: Vec::new(),
            },
        ))
        .await;
        assert!(result.is_ok());
        assert_ne!(result.unwrap().len(), 0);
    }
}
