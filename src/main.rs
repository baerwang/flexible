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

use std::fs;

use flexible::conf::config::ConfigData;
use flexible::plugins::get_api;

fn main() {
    let toml_data = match fs::read_to_string("config.toml") {
        Ok(content) => content,
        Err(err) => {
            panic!("Failed to read TOML file: {}", err);
        }
    };

    let config: ConfigData = match toml::from_str(&toml_data) {
        Ok(parsed) => parsed,
        Err(err) => {
            panic!("Failed to parse TOML data: {}", err);
        }
    };

    let token = config.token.as_str();
    let plugin = config.plugin.as_str();

    config.owners.iter().for_each(|owner| {
        let api = get_api(plugin, owner.name.clone());
        owner.repos.iter().for_each(|repo| {
            _ = api.execute(token, repo.as_str());
        })
    });

    for (org, repos) in &config.orgs {
        let api = get_api(plugin, org.clone());
        for repo in repos {
            _ = api.execute(token, repo.as_str());
        }
    }
}
