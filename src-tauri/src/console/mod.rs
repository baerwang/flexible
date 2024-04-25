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

use serde::{Deserialize, Serialize};

pub mod api;
mod model;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rest<T> {
    data: Option<T>,
    error: Option<String>,
}

impl<T> Rest<T> {
    pub fn new(data: Option<T>, error: Option<String>) -> Rest<T> {
        Rest { data, error }
    }

    pub fn from_result(result: Result<T, reqwest::Error>) -> Rest<T> {
        match result {
            Ok(data) => Rest::new(Some(data), None),
            Err(err) => Rest::new(None, Some(err.to_string())),
        }
    }
}
