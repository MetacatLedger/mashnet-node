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

//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-17, STEPS: `[1, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// did
// --steps
// 1
// --repeat
// 20
// --execution
// wasm
// --wasm-execution
// Compiled
// --output
// runtimes/parachain/src/weights/did.rs
// --template
// .maintain/runtime-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weights for did using the recommended hardware.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> did::WeightInfo for WeightInfo<T> {
	fn submit_did_create_operation_ed25519_keys(n: u32, _u: u32, ) -> Weight {
		134_704_000_u64
			// Standard Error: 212_000
			.saturating_add(2_766_000_u64.saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_create_operation_sr25519_keys(n: u32, u: u32, ) -> Weight {
		125_322_000_u64
			// Standard Error: 14_000
			.saturating_add(2_783_000_u64.saturating_mul(n as Weight))
			// Standard Error: 0
			.saturating_add(9_000_u64.saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_update_operation_ed25519_keys(n: u32, m: u32, u: u32, ) -> Weight {
		114_436_000_u64
			// Standard Error: 14_000
			.saturating_add(4_736_000_u64.saturating_mul(n as Weight))
			// Standard Error: 14_000
			.saturating_add(3_237_000_u64.saturating_mul(m as Weight))
			// Standard Error: 0
			.saturating_add(8_000_u64.saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_update_operation_sr25519_keys(n: u32, m: u32, u: u32, ) -> Weight {
		114_959_000_u64
			// Standard Error: 85_000
			.saturating_add(4_952_000_u64.saturating_mul(n as Weight))
			// Standard Error: 85_000
			.saturating_add(3_432_000_u64.saturating_mul(m as Weight))
			// Standard Error: 3_000
			.saturating_add(7_000_u64.saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_delete_operation() -> Weight {
		108_954_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_ed25519_key() -> Weight {
		112_961_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn submit_did_call_sr25519_key() -> Weight {
		115_085_000_u64
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}