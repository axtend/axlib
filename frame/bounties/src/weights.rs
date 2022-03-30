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

//! Autogenerated weights for pallet_bounties
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-30, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_bounties
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/bounties/src/weights.rs
// --template=.maintain/frame-weight-template.hbs
// --header=HEADER-APACHE2
// --raw

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_bounties.
pub trait WeightInfo {
	fn propose_bounty(d: u32, ) -> Weight;
	fn approve_bounty() -> Weight;
	fn propose_curator() -> Weight;
	fn unassign_curator() -> Weight;
	fn accept_curator() -> Weight;
	fn award_bounty() -> Weight;
	fn claim_bounty() -> Weight;
	fn close_bounty_proposed() -> Weight;
	fn close_bounty_active() -> Weight;
	fn extend_bounty_expiry() -> Weight;
	fn spend_funds(b: u32, ) -> Weight;
}

/// Weights for pallet_bounties using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	fn propose_bounty(d: u32, ) -> Weight {
		(23_620_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: Bounties BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		(6_655_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		(5_720_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		(26_958_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		(21_457_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	fn award_bounty() -> Weight {
		(17_483_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		(61_763_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		(28_862_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		(45_129_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		(15_169_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn spend_funds(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((28_850_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Bounties BountyCount (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	// Storage: Bounties Bounties (r:0 w:1)
	fn propose_bounty(d: u32, ) -> Weight {
		(23_620_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: Bounties BountyApprovals (r:1 w:1)
	fn approve_bounty() -> Weight {
		(6_655_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn propose_curator() -> Weight {
		(5_720_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn unassign_curator() -> Weight {
		(26_958_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn accept_curator() -> Weight {
		(21_457_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	fn award_bounty() -> Weight {
		(17_483_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:3 w:3)
	// Storage: ChildBounties ChildrenCuratorFees (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn claim_bounty() -> Weight {
		(61_763_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_proposed() -> Weight {
		(28_862_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: ChildBounties ParentChildBounties (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Bounties BountyDescriptions (r:0 w:1)
	fn close_bounty_active() -> Weight {
		(45_129_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Bounties Bounties (r:1 w:1)
	fn extend_bounty_expiry() -> Weight {
		(15_169_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Bounties BountyApprovals (r:1 w:1)
	// Storage: Bounties Bounties (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn spend_funds(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 14_000
			.saturating_add((28_850_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().reads((3 as Weight).saturating_mul(b as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((3 as Weight).saturating_mul(b as Weight)))
	}
}
