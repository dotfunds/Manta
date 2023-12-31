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

//! Autogenerated weights for pallet_collective
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-22, STEPS: `50`, REPEAT: 40, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("/home/runner/runners/2.280.1/_work/Manta/Manta/tests/data/fork.json"), DB CACHE: 1024

// Executed Command:
// ./target/production/manta
// benchmark
// pallet
// --chain=/home/runner/runners/2.280.1/_work/Manta/Manta/tests/data/fork.json
// --steps=50
// --repeat=40
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./scripts/benchmarking/frame-weights-output/pallet_collective.rs
// --template=.github/resources/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;
use manta_primitives::constants::RocksDbWeight;

/// Weight functions needed for pallet_collective.
pub trait WeightInfo {
    fn set_members(m: u32, n: u32, p: u32, ) -> Weight;
    fn execute(b: u32, m: u32, ) -> Weight;
    fn propose_execute(b: u32, m: u32, ) -> Weight;
    fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight;
    fn vote(m: u32, ) -> Weight;
    fn close_early_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn close_disapproved(m: u32, p: u32, ) -> Weight;
    fn close_approved(b: u32, m: u32, p: u32, ) -> Weight;
    fn disapprove_proposal(p: u32, ) -> Weight;
}

/// Weights for pallet_collective using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for SubstrateWeight<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 27_601 nanoseconds.
		Weight::from_ref_time(27_900_000)
			// Standard Error: 52_324
			.saturating_add(Weight::from_ref_time(5_869_834).saturating_mul(m.into()))
			// Standard Error: 52_324
			.saturating_add(Weight::from_ref_time(8_607_972).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Council Members (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 21_573 nanoseconds.
		Weight::from_ref_time(22_119_117)
			// Standard Error: 55
			.saturating_add(Weight::from_ref_time(1_731).saturating_mul(b.into()))
			// Standard Error: 570
			.saturating_add(Weight::from_ref_time(14_306).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 24_035 nanoseconds.
		Weight::from_ref_time(24_413_549)
			// Standard Error: 500
			.saturating_add(Weight::from_ref_time(2_228).saturating_mul(b.into()))
			// Standard Error: 5_154
			.saturating_add(Weight::from_ref_time(27_937).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 32_139 nanoseconds.
		Weight::from_ref_time(32_988_414)
			// Standard Error: 96
			.saturating_add(Weight::from_ref_time(3_883).saturating_mul(b.into()))
			// Standard Error: 1_011
			.saturating_add(Weight::from_ref_time(25_351).saturating_mul(m.into()))
			// Standard Error: 998
			.saturating_add(Weight::from_ref_time(167_015).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 35_026 nanoseconds.
		Weight::from_ref_time(35_937_818)
			// Standard Error: 474
			.saturating_add(Weight::from_ref_time(50_834).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 34_093 nanoseconds.
		Weight::from_ref_time(36_565_900)
			// Standard Error: 982
			.saturating_add(Weight::from_ref_time(29_865).saturating_mul(m.into()))
			// Standard Error: 957
			.saturating_add(Weight::from_ref_time(158_173).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 46_150 nanoseconds.
		Weight::from_ref_time(49_371_588)
			// Standard Error: 86
			.saturating_add(Weight::from_ref_time(2_476).saturating_mul(b.into()))
			// Standard Error: 910
			.saturating_add(Weight::from_ref_time(19_202).saturating_mul(m.into()))
			// Standard Error: 887
			.saturating_add(Weight::from_ref_time(170_186).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 36_621 nanoseconds.
		Weight::from_ref_time(39_825_772)
			// Standard Error: 995
			.saturating_add(Weight::from_ref_time(31_586).saturating_mul(m.into()))
			// Standard Error: 970
			.saturating_add(Weight::from_ref_time(151_300).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 48_919 nanoseconds.
		Weight::from_ref_time(51_545_038)
			// Standard Error: 88
			.saturating_add(Weight::from_ref_time(2_688).saturating_mul(b.into()))
			// Standard Error: 936
			.saturating_add(Weight::from_ref_time(21_139).saturating_mul(m.into()))
			// Standard Error: 913
			.saturating_add(Weight::from_ref_time(173_349).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 22_486 nanoseconds.
		Weight::from_ref_time(25_123_317)
			// Standard Error: 1_056
			.saturating_add(Weight::from_ref_time(157_462).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 27_601 nanoseconds.
		Weight::from_ref_time(27_900_000)
			// Standard Error: 52_324
			.saturating_add(Weight::from_ref_time(5_869_834).saturating_mul(m.into()))
			// Standard Error: 52_324
			.saturating_add(Weight::from_ref_time(8_607_972).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(RocksDbWeight::get().writes(2))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Council Members (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 21_573 nanoseconds.
		Weight::from_ref_time(22_119_117)
			// Standard Error: 55
			.saturating_add(Weight::from_ref_time(1_731).saturating_mul(b.into()))
			// Standard Error: 570
			.saturating_add(Weight::from_ref_time(14_306).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 24_035 nanoseconds.
		Weight::from_ref_time(24_413_549)
			// Standard Error: 500
			.saturating_add(Weight::from_ref_time(2_228).saturating_mul(b.into()))
			// Standard Error: 5_154
			.saturating_add(Weight::from_ref_time(27_937).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 32_139 nanoseconds.
		Weight::from_ref_time(32_988_414)
			// Standard Error: 96
			.saturating_add(Weight::from_ref_time(3_883).saturating_mul(b.into()))
			// Standard Error: 1_011
			.saturating_add(Weight::from_ref_time(25_351).saturating_mul(m.into()))
			// Standard Error: 998
			.saturating_add(Weight::from_ref_time(167_015).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(4))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 35_026 nanoseconds.
		Weight::from_ref_time(35_937_818)
			// Standard Error: 474
			.saturating_add(Weight::from_ref_time(50_834).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2))
			.saturating_add(RocksDbWeight::get().writes(1))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 34_093 nanoseconds.
		Weight::from_ref_time(36_565_900)
			// Standard Error: 982
			.saturating_add(Weight::from_ref_time(29_865).saturating_mul(m.into()))
			// Standard Error: 957
			.saturating_add(Weight::from_ref_time(158_173).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(3))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 46_150 nanoseconds.
		Weight::from_ref_time(49_371_588)
			// Standard Error: 86
			.saturating_add(Weight::from_ref_time(2_476).saturating_mul(b.into()))
			// Standard Error: 910
			.saturating_add(Weight::from_ref_time(19_202).saturating_mul(m.into()))
			// Standard Error: 887
			.saturating_add(Weight::from_ref_time(170_186).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 36_621 nanoseconds.
		Weight::from_ref_time(39_825_772)
			// Standard Error: 995
			.saturating_add(Weight::from_ref_time(31_586).saturating_mul(m.into()))
			// Standard Error: 970
			.saturating_add(Weight::from_ref_time(151_300).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(4))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 48_919 nanoseconds.
		Weight::from_ref_time(51_545_038)
			// Standard Error: 88
			.saturating_add(Weight::from_ref_time(2_688).saturating_mul(b.into()))
			// Standard Error: 936
			.saturating_add(Weight::from_ref_time(21_139).saturating_mul(m.into()))
			// Standard Error: 913
			.saturating_add(Weight::from_ref_time(173_349).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(5))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 22_486 nanoseconds.
		Weight::from_ref_time(25_123_317)
			// Standard Error: 1_056
			.saturating_add(Weight::from_ref_time(157_462).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(1))
			.saturating_add(RocksDbWeight::get().writes(3))
	}
}
