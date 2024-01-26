/*
 * Copyright 2018 Bitwise IO, Inc.
 * Copyright 2019-2021 Cargill Incorporated
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * -----------------------------------------------------------------------------
 */

//! # Transact
//!
//! Transact makes writing distributed ledger software easier by providing a shared software
//! library that handles the execution of smart contracts, including all aspects of scheduling,
//! transaction dispatch, and state management.
//!
//! Framework-level projects and custom distributed ledgers can use Transact's advanced transaction
//! execution and state management to simplify the transaction execution code in their projects and
//! to take advantage of Transact’s additional features.
//!
//! More specifically, Transact provides an extensible approach to implementing new smart contract
//! languages called “smart contract engines.” Each smart contract engine implements a virtual
//! machine or interpreter that processes smart contracts.
//!
//! ## Diving Deeper
//!
//! Transact is fundamentally a transaction processing system for state transitions. State data is
//! generally stored in a Merkle-Radix tree a key-value database, or an SQL database. Given an
//! initial state and a transaction, Transact executes the transaction to produce a new state.
//! These state transitions are considered “pure” because only the initial state and the
//! transaction are used as input. (In contrast, other systems such as Ethereum combine state and
//! block information to produce the new state.) As a result, Transact is agnostic about framework
//! features other than transaction execution and state. Awesome, right?
//!
//! Transact deliberately omits other features such as consensus, blocks, chaining, and peering.
//! These features are the responsibility of the frameworks and other distributed ledger
//! implementations. The focus on smart contract execution means that Transact can be used for
//! smart contract execution without conflicting with other platform-level architectural design
//! elements.
//!
//! Transact includes the following components:
//!
//! * State. The Transact state implementation provides get, set, and delete operations against a
//!   database. For the Merkle-Radix tree state implementation, the tree structure is implemented on
//!   top of LMDB or an in-memory database.
//! * Context manager. In Transact, state reads and writes are scoped (sandboxed) to a specific
//!   "context" that contains a reference to a state ID (such as a Merkle-Radix state root hash) and
//!   one or more previous contexts. The context manager implements the context lifecycle and
//!   services the calls that read, write, and delete data from state.
//! * Scheduler. This component controls the order of transactions to be executed. Concrete
//!   implementations include a serial scheduler and a parallel scheduler. Parallel transaction
//!   execution is an important innovation for increasing network throughput.
//! * Executor. The Transact executor obtains transactions from the scheduler and executes them
//!   against a specific context. Execution is handled by sending the transaction to specific
//!   execution adapters (such as ZMQ or a static in-process adapter) which, in turn, send the
//!   transaction to a specific smart contract.
//! * Smart Contract Engines. These components provide the virtual machine implementations and
//!   interpreters that run the smart contracts. Examples of engines include WebAssembly, Ethereum
//!   Virtual Machine, Sawtooth Transactions Processors, and Fabric Chain Code.
//!
//! ## Sawtooth Compatibility Layer
//!
//! Transact provides optional support for smart contract engines implemented for Sawtooth through
//! the `sawtooth-compat` feature.


#[cfg(feature = "transact-context")]
mod collections;
#[cfg(feature = "transact-context")]
pub mod context;
#[cfg(feature = "transact-contract")]
pub mod contract;
pub mod database;
#[cfg(feature = "transact-execution")]
pub mod execution;
#[cfg(feature = "transact-handler")]
pub mod handler;
pub mod protocol;
#[cfg(feature = "transact-sawtooth-compat")]
pub mod sawtooth;
#[cfg(feature = "transact-scheduler")]
pub mod scheduler;
pub mod state;
#[cfg(feature = "transact-workload")]
pub mod workload;
