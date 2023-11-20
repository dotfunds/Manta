// Copyright 2020-2023 Manta Network.
// This file is part of Manta.
//
// Manta is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Manta is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Manta.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_manta_pay
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-21, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("/home/runner/runners/2.280.1/_work/Manta/Manta/tests/data/fork.json"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=/home/runner/runners/2.280.1/_work/Manta/Manta/tests/data/fork.json
// --steps=50
// --repeat=40
// --pallet=pallet_manta_pay
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_manta_pay.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_manta_pay.
pub trait WeightInfo {
    fn to_private() -> Weight;
    fn to_public() -> Weight;
    fn private_transfer() -> Weight;
    fn public_transfer() -> Weight;
}

/// Weights for pallet_manta_pay using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_manta_pay::WeightInfo for SubstrateWeight<T> {
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay NullifierSetSize (r:1 w:0)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_private() -> Weight {
		// Minimum execution time: 39_656_740 nanoseconds.
		Weight::from_ref_time(39_719_889_000)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
	// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay NullifierSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_public() -> Weight {
		// Minimum execution time: 52_751_266 nanoseconds.
		Weight::from_ref_time(52_859_613_000)
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(12))
	}
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
	// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:2 w:2)
	// Storage: MantaPay NullifierSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:2 w:2)
	// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:2)
	fn private_transfer() -> Weight {
		// Minimum execution time: 71_002_626 nanoseconds.
		Weight::from_ref_time(71_128_802_000)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(13))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	fn public_transfer() -> Weight {
		// Minimum execution time: 49_494 nanoseconds.
		Weight::from_ref_time(52_868_000)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay NullifierSetSize (r:1 w:0)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:0 w:1)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_private() -> Weight {
		// Minimum execution time: 39_656_740 nanoseconds.
		Weight::from_ref_time(39_719_889_000)
			.saturating_add(RocksDbWeight::get().reads(6))
			.saturating_add(RocksDbWeight::get().writes(7))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:1)
	// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:1 w:1)
	// Storage: MantaPay NullifierSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:1 w:1)
	// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:1)
	fn to_public() -> Weight {
		// Minimum execution time: 52_751_266 nanoseconds.
		Weight::from_ref_time(52_859_613_000)
			.saturating_add(RocksDbWeight::get().reads(10))
			.saturating_add(RocksDbWeight::get().writes(12))
	}
	// Storage: MantaPay UtxoAccumulatorOutputs (r:2 w:2)
	// Storage: MantaPay NullifierCommitmentSet (r:2 w:2)
	// Storage: MantaPay UtxoSet (r:2 w:2)
	// Storage: MantaPay NullifierSetSize (r:1 w:1)
	// Storage: MantaPay ShardTrees (r:2 w:2)
	// Storage: MantaPay NullifierSetInsertionOrder (r:0 w:2)
	// Storage: MantaPay Shards (r:0 w:2)
	fn private_transfer() -> Weight {
		// Minimum execution time: 71_002_626 nanoseconds.
		Weight::from_ref_time(71_128_802_000)
			.saturating_add(RocksDbWeight::get().reads(9))
			.saturating_add(RocksDbWeight::get().writes(13))
	}
	// Storage: Assets Asset (r:1 w:1)
	// Storage: Assets Account (r:2 w:2)
	fn public_transfer() -> Weight {
		// Minimum execution time: 49_494 nanoseconds.
		Weight::from_ref_time(52_868_000)
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}
