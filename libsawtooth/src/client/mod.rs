// Copyright 2020 Bitwise IO
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

//! A trait for implementing a sawtooth client.

mod error;
pub mod rest;

pub use error::SawtoothClientError;

/// A trait that can be used to interact with a sawtooth node.
pub trait SawtoothClient {
    /// Get a single batch in the current blockchain.
    fn get_batch(&self, batch_id: String) -> Result<Option<Batch>, SawtoothClientError>;
    /// Get all existing batches in the current blockchain.
    fn list_batches(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<Batch, SawtoothClientError>>>, SawtoothClientError>;
}

/// A struct that represents a batch.
#[derive(Debug)]
pub struct Batch {
    pub header: Header,
    pub header_signature: String,
    pub trace: bool,
    pub transactions: Vec<Transaction>,
}
#[derive(Debug)]
pub struct Header {
    pub signer_public_key: String,
    pub transaction_ids: Vec<String>,
}
#[derive(Debug)]
pub struct Transaction {
    pub header: TransactionHeader,
    pub header_signature: String,
    pub payload: String,
}
#[derive(Debug)]
pub struct TransactionHeader {
    pub batcher_public_key: String,
    pub dependencies: Vec<String>,
    pub family_name: String,
    pub family_version: String,
    pub inputs: Vec<String>,
    pub nonce: String,
    pub outputs: Vec<String>,
    pub payload_sha512: String,
    pub signer_public_key: String,
}