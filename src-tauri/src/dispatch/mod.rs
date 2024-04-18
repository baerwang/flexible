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
use std::time::Duration;

use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

use crate::conf::config::ConfigData;
use crate::plugins::github::GitHub;

pub async fn execute(c: ConfigData) -> Result<String, JobSchedulerError> {
    let sched = JobScheduler::new().await?;
    let token = c.token;
    let reviews: HashMap<String, ()> = c.reviews.iter().map(|key| (key.clone(), ())).collect();
    let task = Job::new_repeated(Duration::from_secs(c.dispatch), move |_uuid, _l| {
        if !c.owners.name.is_empty() {
            let hub = GitHub::new(c.owners.name.clone(), reviews.clone());
            c.owners.repos.iter().for_each(|repo| {
                if !repo.is_empty() {
                    _ = hub.execute(token.as_str(), repo.as_str());
                }
            });
        }

        if !c.orgs.is_empty() {
            for (org, repos) in &c.orgs {
                if !org.is_empty() && !repos.is_empty() {
                    let hub = GitHub::new(org.clone(), reviews.clone());
                    for repo in repos {
                        _ = hub.execute(token.as_str(), repo.as_str());
                    }
                }
            }
        }
    })?;
    let uuid = sched.add(task).await?;
    sched.start().await?;
    Ok(uuid.to_string())
}
