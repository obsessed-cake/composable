
//! Autogenerated weights for `proxy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `c93baf6406af`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("picasso-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/y1z2mfgy9msqas77hhxszf78hqg6mx5y-composable/bin/composable
// benchmark
// pallet
// --chain=picasso-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/picasso/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `proxy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> proxy::WeightInfo for WeightInfo<T> {
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `p` is `[1, 3]`.
	fn proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(56_751_000_u64)
			// Standard Error: 1_424_000
			.saturating_add(Weight::from_ref_time(144_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn proxy_announced(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(100_739_000_u64)
			// Standard Error: 33_000
			.saturating_add(Weight::from_ref_time(679_000_u64).saturating_mul(a as u64))
			// Standard Error: 898_000
			.saturating_add(Weight::from_ref_time(132_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn remove_announcement(a: u32, _p: u32, ) -> Weight {
		Weight::from_ref_time(78_628_000_u64)
			// Standard Error: 39_000
			.saturating_add(Weight::from_ref_time(850_000_u64).saturating_mul(a as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn reject_announcement(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(65_887_000_u64)
			// Standard Error: 28_000
			.saturating_add(Weight::from_ref_time(574_000_u64).saturating_mul(a as u64))
			// Standard Error: 750_000
			.saturating_add(Weight::from_ref_time(1_493_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Proxy Proxies (r:1 w:0)
	// Storage: Proxy Announcements (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	/// The range of component `a` is `[0, 31]`.
	/// The range of component `p` is `[1, 3]`.
	fn announce(a: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(85_432_000_u64)
			// Standard Error: 24_000
			.saturating_add(Weight::from_ref_time(569_000_u64).saturating_mul(a as u64))
			// Standard Error: 648_000
			.saturating_add(Weight::from_ref_time(917_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn add_proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(72_349_000_u64)
			// Standard Error: 1_118_000
			.saturating_add(Weight::from_ref_time(2_417_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxy(p: u32, ) -> Weight {
		Weight::from_ref_time(72_690_000_u64)
			// Standard Error: 669_000
			.saturating_add(Weight::from_ref_time(1_046_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Proxy Proxies (r:1 w:1)
	/// The range of component `p` is `[1, 3]`.
	fn remove_proxies(p: u32, ) -> Weight {
		Weight::from_ref_time(63_884_000_u64)
			// Standard Error: 292_000
			.saturating_add(Weight::from_ref_time(11_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn create_pure(_: u32) -> Weight {
		Weight::from_ref_time(10_000_u64)
	}
	fn kill_pure(_: u32) -> Weight {
		Weight::from_ref_time(10_000_u64)
	}
}
