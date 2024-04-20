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
pub async fn repos(conf: ConfigData) -> Result<Vec<Repo>, anyhow::Error> {
    let api = get_api(
        conf.plugin.as_str(),
        conf.owners.name.clone(),
        conf.reviews(),
    );
    Ok(client::<Vec<Repo>>(api.repos(), api.headers(conf.token.as_str())).await?)
}

#[tauri::command]
#[allow(clippy::unused_unit)]
pub async fn orgs(conf: ConfigData) -> Result<Vec<Org>, anyhow::Error> {
    let api = get_api(conf.plugin.as_str(), "".to_string(), conf.reviews());
    let client = reqwest::Client::new();
    // todo request There is a problem with populating access with headers
    let request = client
        .get(api.orgs())
        .header("Authorization", format!("Bearer {}", conf.token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "Awesome-Octocat-App")
        .build()?;
    let organizations: Vec<Org> = client.execute(request).await?.json().await?;
    Ok(organizations)
}

#[tauri::command]
#[allow(clippy::unused_unit)]
pub async fn org_repos(conf: ConfigData) -> Result<Vec<Repo>, anyhow::Error> {
    let api = get_api(
        conf.plugin.as_str(),
        conf.owners.name.clone(),
        conf.reviews(),
    );
    Ok(client::<Vec<Repo>>(api.org_repos(), api.headers(conf.token.as_str())).await?)
}

#[cfg(test)]
mod test {
    use crate::conf::config::{ConfigData, Owner};
    use crate::console::api::{org_repos, repos};

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

    /*#[tokio::test]
    async fn test_orgs() {
        let result = orgs(ConfigData::new(
            "github",
            env::var("TOKEN").unwrap().as_str(),
        ))
        .await;
        assert!(result.is_ok());
        assert_ne!(result.unwrap().len(), 0);
    }*/

    #[tokio::test]
    async fn test_org_repos() {
        let result = org_repos(ConfigData::new_owner(
            "github",
            "",
            Owner {
                name: "Suzaku-APIX".to_string(),
                repos: Vec::new(),
            },
        ))
        .await;
        assert!(result.is_ok());
        assert_ne!(result.unwrap().len(), 0);
    }
}
