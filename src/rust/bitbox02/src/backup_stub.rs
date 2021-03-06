// Copyright 2020 Shift Crypto AG
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Stubs for testing.

extern crate alloc;
use alloc::string::String;
pub use bitbox02_sys::backup_error_t as Error;

pub struct CheckData {
    pub id: String,
    pub name: String,
    // unix timestamp, UTC.
    pub birthdate: u32,
}

pub fn create(backup_create_timestamp: u32, seed_birthdate_timestamp: u32) -> Result<(), Error> {
    let data = crate::testing::DATA.0.borrow();
    data.backup_create.as_ref().unwrap()(backup_create_timestamp, seed_birthdate_timestamp)
}

pub fn check() -> Result<CheckData, Error> {
    panic!("not implemented")
}
