use std::string::String;

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde_json::Value;

use crate::plugins::api::Api;
use crate::plugins::client;

pub struct GitHub {
    pub owner: Option<String>,
    pub org: Option<String>,
}

impl GitHub {
    pub fn new(owner: Option<String>, org: Option<String>) -> Self {
        GitHub { owner, org }
    }

    fn name(&self) -> String {
        match self.owner {
            Some(ref s) => s.clone(),
            None => match self.org {
                Some(ref s) => s.clone(),
                None => panic!("No owner or org specified"),
            },
        }
    }
}

impl Api for GitHub {
    fn execute(&self, token: &str) -> Result<(), anyhow::Error> {
        let resp = client(self.repos(), self.headers(token))?;
        let parsed_json: Value = serde_json::from_str(&resp)?;
        if let Some(array) = parsed_json.as_array() {
            for element in array {
                if let Some(name) = element.get("name").and_then(|name| name.as_str()) {
                    let rsp = client(self.pull_requests(name), self.headers(token))?;
                    let parsed_json: Value = serde_json::from_str(&rsp)?;
                    if let Some(array) = parsed_json.as_array() {
                        for element in array {
                            if let Some(number) =
                                element.get("number").and_then(|number| number.as_i64())
                            {
                                let rsp = client(self.reviews(name, number), self.headers(token))?;
                                // todo
                                println!("{}", rsp)
                            }
                        }
                    }
                }
            }
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

    fn repos(&self) -> String {
        match self.owner {
            Some(ref s) => format!("{}/users/{}/repos?page=1&per_page=100", self.domain(), s),
            None => match self.org {
                Some(ref s) => format!("{}/orgs/{}/repos?page=1&per_page=100", self.domain(), s),
                None => panic!("No owner or org specified"),
            },
        }
    }

    fn pull_requests(&self, repo: &str) -> String {
        format!("{}/repos/{}/{repo}/pulls", self.domain(), self.name())
    }

    fn issues(&self, repo: &str) -> String {
        format!("{}/repos/{}/{repo}/issues", self.domain(), self.name())
    }

    fn reviews(&self, repo: &str, number: i64) -> String {
        format!(
            "{}/repos/{}/{repo}/pulls/{number}/requested_reviewers",
            self.domain(),
            self.name()
        )
    }
}
