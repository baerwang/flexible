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

use crate::conf::config::ConfigData;
use crate::console::model::{Org, Repo};
use crate::console::Rest;
use crate::dispatch;
use crate::plugins::{client, get_api};

#[tauri::command]
pub async fn create(conf: ConfigData) -> String {
    match conf.valid() {
        "" => {
            _ = dispatch::execute(conf).await;
            "".to_string()
        }
        r => r.to_string(),
    }
}

#[tauri::command]
pub async fn repos(conf: ConfigData) -> Rest<Vec<Repo>> {
    let api = get_api(
        conf.plugin.as_str(),
        conf.owners.name.clone(),
        conf.reviews(),
    );
    Rest::from_result(client::<Vec<Repo>>(api.repos(), api.headers(conf.token.as_str())).await)
}

#[tauri::command]
pub async fn orgs(conf: ConfigData) -> Rest<Vec<Org>> {
    let api = get_api(conf.plugin.as_str(), "".to_string(), conf.reviews());
    Rest::from_result(client::<Vec<Org>>(api.orgs(), api.headers(conf.token.as_str())).await)
}

#[tauri::command]
pub async fn org_repos(conf: ConfigData) -> Rest<Vec<Repo>> {
    let api = get_api(
        conf.plugin.as_str(),
        conf.owners.name.clone(),
        conf.reviews(),
    );
    Rest::from_result(client::<Vec<Repo>>(api.org_repos(), api.headers(conf.token.as_str())).await)
}

#[cfg(test)]
mod test {
    use std::env;

    use crate::conf::config::{ConfigData, Owner};
    use crate::console::api::{org_repos, orgs, repos};

    fn token() -> String {
        env::var("TOKEN_GITHUB").expect("TOKEN environment variable not found")
    }

    #[test]
    fn test_token() {
        assert_ne!(token().len(), 0);
    }

    #[tokio::test]
    async fn test_repos() {
        let result = repos(ConfigData::new_owner(
            "github",
            token().as_str(),
            Owner {
                name: "baerwang".to_string(),
                repos: Vec::new(),
            },
        ))
        .await;
        assert!(result.error.is_none());
        assert_ne!(result.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_orgs() {
        let result = orgs(ConfigData::new("github", token().as_str())).await;
        assert!(result.error.is_none());
        assert_ne!(result.data.unwrap().len(), 0);
    }

    #[tokio::test]
    async fn test_org_repos() {
        let result = org_repos(ConfigData::new_owner(
            "github",
            token().as_str(),
            Owner {
                name: "Suzaku-APIX".to_string(),
                repos: Vec::new(),
            },
        ))
        .await;
        assert!(result.error.is_none());
        assert_ne!(result.data.unwrap().len(), 0);
    }
}
