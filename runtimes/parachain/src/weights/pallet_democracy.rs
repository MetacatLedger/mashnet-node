// KILT Blockchain – https://botlabs.org
// Copyright (C) 2019-2021 BOTLabs GmbH

// The KILT Blockchain is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The KILT Blockchain is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// If you feel like getting in touch with us, you can do so at info@botlabs.org

//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-17, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/kilt-parachain
// benchmark
// --chain
// dev
// --heap-pages
// 4096
// --extrinsic
// *
// --pallet
// pallet_democracy
// --steps
// 50
// --repeat
// 20
// --execution
// wasm
// --wasm-execution
// Compiled
// --output
// ../../runtimes/spiritnet/src/weights/pallet_democracy.rs
// --template
// .maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for pallet_democracy using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	fn propose() -> Weight {
		99_266_000_u64
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn second(s: u32, ) -> Weight {
		60_693_000_u64
			// Standard Error: 3_000
			.saturating_add(279_000_u64.saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn vote_new(r: u32, ) -> Weight {
		65_149_000_u64
			// Standard Error: 3_000
			.saturating_add(448_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn vote_existing(r: u32, ) -> Weight {
		65_455_000_u64
			// Standard Error: 3_000
			.saturating_add(438_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn emergency_cancel() -> Weight {
		46_858_000_u64
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn blacklist(p: u32, ) -> Weight {
		121_971_000_u64
			// Standard Error: 8_000
			.saturating_add(774_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	fn external_propose(v: u32, ) -> Weight {
		20_785_000_u64
			// Standard Error: 0
			.saturating_add(118_000_u64.saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn external_propose_majority() -> Weight {
		4_338_000_u64
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn external_propose_default() -> Weight {
		4_328_000_u64
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn fast_track() -> Weight {
		44_013_000_u64
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn veto_external(v: u32, ) -> Weight {
		43_973_000_u64
			// Standard Error: 1_000
			.saturating_add(162_000_u64.saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn cancel_proposal(p: u32, ) -> Weight {
		78_717_000_u64
			// Standard Error: 3_000
			.saturating_add(769_000_u64.saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	fn cancel_referendum() -> Weight {
		25_778_000_u64
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn cancel_queued(r: u32, ) -> Weight {
		37_485_000_u64
			// Standard Error: 7_000
			.saturating_add(2_229_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn on_initialize_base(r: u32, ) -> Weight {
		0_u64
			// Standard Error: 25_000
			.saturating_add(9_512_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64.saturating_mul(r as Weight)))
	}
	fn delegate(r: u32, ) -> Weight {
		46_054_000_u64
			// Standard Error: 32_000
			.saturating_add(13_514_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64.saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64.saturating_mul(r as Weight)))
	}
	fn undelegate(r: u32, ) -> Weight {
		1_793_000_u64
			// Standard Error: 29_000
			.saturating_add(13_453_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads(1_u64.saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64.saturating_mul(r as Weight)))
	}
	fn clear_public_proposals() -> Weight {
		4_499_000_u64
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn note_preimage(b: u32, ) -> Weight {
		68_379_000_u64
			// Standard Error: 0
			.saturating_add(3_000_u64.saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn note_imminent_preimage(b: u32, ) -> Weight {
		42_134_000_u64
			// Standard Error: 0
			.saturating_add(3_000_u64.saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn reap_preimage(b: u32, ) -> Weight {
		58_189_000_u64
			// Standard Error: 0
			.saturating_add(2_000_u64.saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn unlock_remove(r: u32, ) -> Weight {
		55_693_000_u64
			// Standard Error: 3_000
			.saturating_add(264_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn unlock_set(r: u32, ) -> Weight {
		51_697_000_u64
			// Standard Error: 2_000
			.saturating_add(411_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn remove_vote(r: u32, ) -> Weight {
		27_816_000_u64
			// Standard Error: 1_000
			.saturating_add(374_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn remove_other_vote(r: u32, ) -> Weight {
		27_972_000_u64
			// Standard Error: 1_000
			.saturating_add(232_000_u64.saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}