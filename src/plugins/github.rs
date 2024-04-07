use std::string::String;

use reqwest::header::{HeaderMap, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

use crate::plugins::api::Api;
use crate::plugins::client;

pub struct GitHub {
    pub owner: String,
}

impl GitHub {
    pub fn new(owner: String) -> Self {
        GitHub { owner }
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
    number: i64,
}

impl Api for GitHub {
    fn domain(&self) -> &str {
        "https://api.github.com"
    }

    fn execute(&self, token: &str, repo: &str) -> Result<(), anyhow::Error> {
        let rsp = client(self.pull_requests(repo), self.headers(token))?;
        let prs: Vec<PullRequest> = serde_json::from_str(&rsp)?;
        for pr in prs {
            let reviews_data = client(self.reviews(repo, pr.number), self.headers(token))?;
            let reviews: Reviews = serde_json::from_str(&reviews_data)?;
            reviews.users.iter().for_each(|user| {
                if user.login == self.owner {
                    println!("{}: {}", repo, pr.number);
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
        format!("{}/users/{repo}/repos?page=1&per_page=100", self.domain())
    }

    fn repos(&self) -> String {
        format!(
            "{}/users/{}/repos?page=1&per_page=100",
            self.domain(),
            self.owner
        )
    }

    fn org_repos(&self) -> String {
        format!(
            "{}/orgs/{}/repos?page=1&per_page=100",
            self.domain(),
            self.owner
        )
    }

    fn pull_requests(&self, repo: &str) -> String {
        format!("{}/repos/{}/{repo}/pulls", self.domain(), self.owner)
    }

    fn issues(&self, repo: &str) -> String {
        format!("{}/repos/{}/{repo}/issues", self.domain(), self.owner)
    }

    fn reviews(&self, repo: &str, number: i64) -> String {
        format!(
            "{}/repos/{}/{repo}/pulls/{number}/requested_reviewers",
            self.domain(),
            self.owner
        )
    }
}
