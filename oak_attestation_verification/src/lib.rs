//
// Copyright 2022 The Project Oak Authors
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
//

#![cfg_attr(not(feature = "std"), no_std)]
#![feature(let_chains)]

extern crate alloc;

pub(crate) mod amd;
pub(crate) mod endorsement;
pub mod expect;
pub(crate) mod extract;
pub mod policy;
pub(crate) mod rekor;
pub(crate) mod util;
pub mod verifier;

#[cfg(test)]
mod test_util;

pub use endorsement::verify_endorsement;
pub use util::{convert_pem_to_raw, reference_values_from_evidence};
