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

use reqwest::header::HeaderMap;
use reqwest::Error;
use serde::de::DeserializeOwned;

use crate::plugins::api::Api;

mod api;
pub mod github;

pub fn get_api(api: &str, owner: String, reviews: Option<HashMap<String, ()>>) -> Box<dyn Api> {
    match api {
        "github" => Box::new(github::GitHub::new(owner, reviews.unwrap_or_default())),
        _ => panic!("Unsupported"),
    }
}

pub async fn get_client<T>(url: String, headers: HeaderMap) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let resp = reqwest::Client::new()
        .get(&url)
        .headers(headers)
        .timeout(std::time::Duration::from_secs(3))
        .send()
        .await?
        .json::<T>()
        .await?;
    Ok(resp)
}
