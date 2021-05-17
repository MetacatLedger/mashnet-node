
//! Autogenerated weights for did
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-05-17, STEPS: `[1, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Interpreted, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/mashnet-node
// benchmark
// --chain=dev
// --steps=1
// --repeat=20
// --pallet=did
// --extrinsic=*
// --execution=wasm
// --heap-pages=4096
// --output=./pallets/did/src/weights.rs


#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for did.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> did::WeightInfo for WeightInfo<T> {
	fn submit_did_create_operation(n: u32, u: u32, ) -> Weight {
		(284_022_000 as Weight)
			// Standard Error: 446_000
			.saturating_add((18_319_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 7_000
			.saturating_add((137_000 as Weight).saturating_mul(u as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
