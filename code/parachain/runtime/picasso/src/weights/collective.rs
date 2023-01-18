
//! Autogenerated weights for `collective`
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

/// Weight functions for `collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> collective::WeightInfo for WeightInfo<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Voting (r:100 w:100)
	// Storage: Council Prime (r:0 w:1)
	/// The range of component `m` is `[1, 100]`.
	/// The range of component `n` is `[1, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn set_members(m: u32, n: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(0_u64)
			// Standard Error: 29_000
			.saturating_add(Weight::from_ref_time(20_303_000_u64).saturating_mul(m as u64))
			// Standard Error: 29_000
			.saturating_add(Weight::from_ref_time(362_000_u64).saturating_mul(n as u64))
			// Standard Error: 29_000
			.saturating_add(Weight::from_ref_time(27_188_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p as u64)))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(54_848_000_u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000_u64).saturating_mul(b as u64))
			// Standard Error: 3_000
			.saturating_add(Weight::from_ref_time(90_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		Weight::from_ref_time(57_208_000_u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(5_000_u64).saturating_mul(b as u64))
			// Standard Error: 4_000
			.saturating_add(Weight::from_ref_time(183_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(52_932_000_u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(17_000_u64).saturating_mul(b as u64))
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(173_000_u64).saturating_mul(m as u64))
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(653_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		Weight::from_ref_time(85_570_000_u64)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(239_000_u64).saturating_mul(m as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(77_058_000_u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(205_000_u64).saturating_mul(m as u64))
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(552_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(97_556_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(19_000_u64).saturating_mul(b as u64))
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(156_000_u64).saturating_mul(m as u64))
			// Standard Error: 9_000
			.saturating_add(Weight::from_ref_time(694_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(75_407_000_u64)
			// Standard Error: 8_000
			.saturating_add(Weight::from_ref_time(280_000_u64).saturating_mul(m as u64))
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(610_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: CallFilter DisabledCalls (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[1, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		Weight::from_ref_time(104_731_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(8_000_u64).saturating_mul(b as u64))
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(310_000_u64).saturating_mul(m as u64))
			// Standard Error: 10_000
			.saturating_add(Weight::from_ref_time(692_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		Weight::from_ref_time(48_967_000_u64)
			// Standard Error: 6_000
			.saturating_add(Weight::from_ref_time(715_000_u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}
