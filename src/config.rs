// Copyright 2019 Cryptape Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ForeverConfig {
    pub name: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub pidfile: Option<String>,
    pub process: Option<Vec<ProcessConfig>>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct ProcessConfig {
    pub name: Option<String>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub pidfile: Option<String>,
    pub respawn: Option<u32>,
    pub pid: Option<u32>,
    pub respawns: Option<u32>,
}

impl ForeverConfig {
    pub fn new(path: &str) -> Self {
        parse_config!(ForeverConfig, path)
    }
}
