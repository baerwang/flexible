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
use std::string::String;

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

use crate::notification::notify::notify;
use crate::plugins::api::Api;
use crate::plugins::api::PullRequest as PR;
use crate::plugins::client;

pub struct GitHub {
    pub owner: String,
    pub reviews: HashMap<String, ()>,
}

impl GitHub {
    pub fn new(owner: String, reviews: HashMap<String, ()>) -> Self {
        GitHub { owner, reviews }
    }
}

#[derive(Debug, Deserialize)]
struct User {
    login: String,
}

#[derive(Debug, Deserialize)]
struct Reviews {
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
struct PullRequest {
    title: String,
    number: i64,
}

impl Api for GitHub {
    fn api(&self) -> &str {
        "https://api.github.com"
    }

    fn execute(&self, token: &str, repo: &str) -> Result<(), anyhow::Error> {
        let rsp = client(self.pull_requests(repo), self.headers(token))?;
        let prs: Vec<PullRequest> = serde_json::from_str(&rsp)?;
        for pr in prs {
            let reviews_data = client(self.reviews(repo, pr.number), self.headers(token))?;
            let reviews: Reviews = serde_json::from_str(&reviews_data)?;
            reviews.users.iter().for_each(|user| {
                if self.reviews.contains_key(user.login.as_str()) {
                    self.notify(
                        repo,
                        "",
                        PR {
                            title: pr.title.clone(),
                            number: pr.number,
                        },
                    )
                }
            });
        }
        Ok(())
    }

    fn headers(&self, token: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer: {}", token).parse().unwrap());
        headers.insert(ACCEPT, "application/vnd.github+json".parse().unwrap());
        headers.insert(USER_AGENT, "Awesome-Octocat-App".parse().unwrap());
        headers
    }

    fn repo(&self, repo: &str) -> String {
        format!("{}/users/{repo}/repos?page=1&per_page=100", self.api())
    }

    fn repos(&self) -> String {
        format!(
            "{}/users/{}/repos?page=1&per_page=100",
            self.api(),
            self.owner
        )
    }

    fn org_repos(&self) -> String {
        format!(
            "{}/orgs/{}/repos?page=1&per_page=100",
            self.api(),
            self.owner
        )
    }

    fn pull_requests(&self, repo: &str) -> String {
        format!("{}/repos/{}/{repo}/pulls", self.api(), self.owner)
    }

    fn issues(&self, repo: &str) -> String {
        format!("{}/repos/{}/{repo}/issues", self.api(), self.owner)
    }

    fn reviews(&self, repo: &str, number: i64) -> String {
        format!(
            "{}/repos/{}/{repo}/pulls/{number}/requested_reviewers",
            self.api(),
            self.owner
        )
    }
    fn notify(&self, repo: &str, _: &str, pr: PR) {
        notify(
            repo,
            pr.title.as_str(),
            format!(
                "{}/repos/{}/{repo}/pull/{}",
                self.api(),
                self.owner,
                pr.number
            )
            .as_str(),
        )
    }
}
