// Copyright 2018-2019, Wayfair GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::errors::*;
use crate::{Event, Operator};

#[derive(Debug, Clone, Hash)]
pub struct PassthroughOperator {}

op!(PassthroughFactory (_node) {
    Ok(Box::new(PassthroughOperator{}))
});

#[allow(unused_mut)]
impl Operator for PassthroughOperator {
    fn on_event(&mut self, _port: &str, event: Event) -> Result<Vec<(String, Event)>> {
        Ok(vec![("out".into(), event)])
    }
}