// Copyright (c) 2019, Arm Limited, All Rights Reserved
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//          http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use super::key_attributes::KeyAttributes;

/// Native object for cryptographic key importing operation.
#[derive(Clone, Debug)]
pub struct OpImportKey {
    /// `key_name` specifies a name by which the service will identify the key. Key
    /// name must be unique per application.
    pub key_name: String,
    /// `key_attributes` specifies the parameters
    /// to be associated with the key.
    pub key_attributes: KeyAttributes,
    /// `key_data` contains the bytes for the key,
    /// formatted in accordance with the requirements of the provider for the key type
    /// specified in `key_attributes`.
    pub key_data: Vec<u8>,
}

/// Native object for the result of a cryptographic key import operation.
///
/// The true result is sent in the `status` field of the response header.
#[derive(Copy, Clone, Debug)]
pub struct ResultImportKey;
