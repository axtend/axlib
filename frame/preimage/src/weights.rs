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

//! Autogenerated weights for pallet_preimage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE AXLIB BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-01-31, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_preimage
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/preimage/src/weights.rs
// --template=.maintain/frame-weight-template.hbs
// --header=HEADER-APACHE2
// --raw

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_preimage.
pub trait WeightInfo {
	fn note_preimage(s: u32, ) -> Weight;
	fn note_requested_preimage(s: u32, ) -> Weight;
	fn note_no_deposit_preimage(s: u32, ) -> Weight;
	fn unnote_preimage() -> Weight;
	fn unnote_no_deposit_preimage() -> Weight;
	fn request_preimage() -> Weight;
	fn request_no_deposit_preimage() -> Weight;
	fn request_unnoted_preimage() -> Weight;
	fn request_requested_preimage() -> Weight;
	fn unrequest_preimage() -> Weight;
	fn unrequest_unnoted_preimage() -> Weight;
	fn unrequest_multi_referenced_preimage() -> Weight;
}

/// Weights for pallet_preimage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn note_preimage(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:0)
	fn note_requested_preimage(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:0)
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_preimage() -> Weight {
		(39_239_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_no_deposit_preimage() -> Weight {
		(24_905_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_preimage() -> Weight {
		(37_451_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_no_deposit_preimage() -> Weight {
		(23_397_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_unnoted_preimage() -> Weight {
		(13_407_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_requested_preimage() -> Weight {
		(4_202_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unrequest_preimage() -> Weight {
		(24_858_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unrequest_unnoted_preimage() -> Weight {
		(13_763_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn unrequest_multi_referenced_preimage() -> Weight {
		(4_262_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:1)
	fn note_preimage(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:0)
	fn note_requested_preimage(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage PreimageFor (r:1 w:1)
	// Storage: Preimage StatusFor (r:1 w:0)
	fn note_no_deposit_preimage(s: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_preimage() -> Weight {
		(39_239_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unnote_no_deposit_preimage() -> Weight {
		(24_905_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_preimage() -> Weight {
		(37_451_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_no_deposit_preimage() -> Weight {
		(23_397_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_unnoted_preimage() -> Weight {
		(13_407_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn request_requested_preimage() -> Weight {
		(4_202_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unrequest_preimage() -> Weight {
		(24_858_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	// Storage: Preimage PreimageFor (r:0 w:1)
	fn unrequest_unnoted_preimage() -> Weight {
		(13_763_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Preimage StatusFor (r:1 w:1)
	fn unrequest_multi_referenced_preimage() -> Weight {
		(4_262_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
