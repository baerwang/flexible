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

use reqwest::header::HeaderMap;

pub struct PullRequest {
    pub title: String,
    pub number: i64,
}

pub trait Api {
    fn api(&self) -> &str;
    fn execute(&self, token: &str, repo: &str) -> Result<(), anyhow::Error>;
    fn headers(&self, token: &str) -> HeaderMap;
    fn repo(&self, repo: &str) -> String;
    fn repos(&self) -> String;
    fn org_repos(&self) -> String;
    fn pull_requests(&self, repo: &str) -> String;
    fn issues(&self, repo: &str) -> String;
    fn reviews(&self, repo: &str, number: i64) -> String;
    fn notify(&self, repo: &str, content: &str, pr: PullRequest);
}
