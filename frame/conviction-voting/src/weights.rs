// This file is part of Substrate.

// Copyright (C) 2022 Axia Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

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

//! Autogenerated weights for pallet_conviction_voting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-09, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_conviction_voting
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/conviction-voting/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_conviction_voting.
pub trait WeightInfo {
	fn vote_new() -> Weight;
	fn vote_existing() -> Weight;
	fn remove_vote() -> Weight;
	fn remove_other_vote() -> Weight;
	fn delegate(r: u32, ) -> Weight;
	fn undelegate(r: u32, ) -> Weight;
	fn unlock() -> Weight;
}

/// Weights for pallet_conviction_voting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote_new() -> Weight {
		(159_647_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote_existing() -> Weight {
		(339_851_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn remove_vote() -> Weight {
		(317_673_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	fn remove_other_vote() -> Weight {
		(52_222_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: ConvictionVoting VotingFor (r:2 w:2)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn delegate(r: u32, ) -> Weight {
		(61_553_000 as Weight)
			// Standard Error: 123_000
			.saturating_add((33_092_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: ConvictionVoting VotingFor (r:2 w:2)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn undelegate(r: u32, ) -> Weight {
		(42_037_000 as Weight)
			// Standard Error: 582_000
			.saturating_add((32_296_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn unlock() -> Weight {
		(69_017_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote_new() -> Weight {
		(159_647_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn vote_existing() -> Weight {
		(339_851_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn remove_vote() -> Weight {
		(317_673_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	fn remove_other_vote() -> Weight {
		(52_222_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: ConvictionVoting VotingFor (r:2 w:2)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn delegate(r: u32, ) -> Weight {
		(61_553_000 as Weight)
			// Standard Error: 123_000
			.saturating_add((33_092_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: ConvictionVoting VotingFor (r:2 w:2)
	// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	// Storage: Scheduler Agenda (r:2 w:2)
	fn undelegate(r: u32, ) -> Weight {
		(42_037_000 as Weight)
			// Standard Error: 582_000
			.saturating_add((32_296_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	// Storage: ConvictionVoting VotingFor (r:1 w:1)
	// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn unlock() -> Weight {
		(69_017_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
