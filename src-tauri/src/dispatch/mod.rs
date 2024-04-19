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

use std::sync::Arc;
use std::time::Duration;

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::conf::config::ConfigData;
use crate::plugins::github::GitHub;

pub async fn execute(c: ConfigData) -> Result<String, JobSchedulerError> {
    let sched = JobScheduler::new().await?;
    let c_shared = Arc::new(c);

    let task = Job::new_repeated(Duration::from_secs(c_shared.dispatch), move |_uuid, _l| {
        let c_shared = Arc::clone(&c_shared);
        tokio::spawn(async move {
            execute_workflow(c_shared).await;
        });
    })?;

    let uuid = sched.add(task).await?;
    sched.start().await?;
    Ok(uuid.to_string())
}

async fn execute_workflow(c_shared: Arc<ConfigData>) {
    // Execute tasks for owners' repos
    if !c_shared.owners.name.is_empty() {
        let hub = GitHub::new(c_shared.owners.name.clone(), c_shared.reviews());
        execute_plugin_tasks(
            Arc::clone(&c_shared),
            &hub,
            c_shared.owners.repos.iter().map(|repo| repo.as_str()),
        )
        .await;
    }

    // Execute tasks for orgs' repos
    for (org, repos) in &c_shared.orgs {
        if !org.is_empty() && !repos.is_empty() {
            let hub = GitHub::new(org.to_string(), c_shared.reviews());
            execute_plugin_tasks(
                Arc::clone(&c_shared),
                &hub,
                repos.iter().map(|repo| repo.as_str()),
            )
            .await;
        }
    }
}

async fn execute_plugin_tasks(
    c_shared: Arc<ConfigData>,
    hub: &GitHub,
    repos: impl Iterator<Item = &str>,
) {
    for repo in repos {
        if let Err(err) = hub.execute(c_shared.token.as_str(), repo).await {
            eprintln!("Error executing task: {}", err);
        }
    }
}
