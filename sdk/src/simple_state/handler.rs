// Copyright 2019 Cargill Incorporated
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

use crate::simple_state::addresser::Addresser;
use crate::{ApplyError, TpProcessRequest, TransactionContext};

pub trait SimpleTransactionHandler {
    type Key;

    fn get_family_name(&self) -> String;

    fn get_family_versions(&self) -> Vec<String>;

    fn get_namespaces(&self) -> Vec<String>;

    fn get_addresser(&self) -> Box<&dyn Addresser<Self::Key>>;

    fn apply(
        &self,
        _request: &TpProcessRequest,
        _addresser: Box<&dyn Addresser<Self::Key>>,
        _context: &mut dyn TransactionContext,
    ) -> Result<(), ApplyError>;
}
