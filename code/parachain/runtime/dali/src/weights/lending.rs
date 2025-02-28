
//! Autogenerated weights for `lending`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-12-16, STEPS: `50`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `fde3d2d43403`, CPU: `Intel(R) Xeon(R) CPU @ 2.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dali-dev"), DB CACHE: 1024

// Executed Command:
// /nix/store/y1z2mfgy9msqas77hhxszf78hqg6mx5y-composable/bin/composable
// benchmark
// pallet
// --chain=dali-dev
// --execution=wasm
// --wasm-execution=compiled
// --wasm-instantiation-strategy=legacy-instance-reuse
// --pallet=*
// --extrinsic=*
// --steps=50
// --repeat=10
// --output=code/parachain/runtime/dali/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `lending`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> lending::WeightInfo for WeightInfo<T> {
	// same as vaults deposit plus 1 more read
	// TODO: Please impl benchmarks
	fn vault_deposit() -> Weight {
		Weight::from_ref_time(140_947_000_u64)
			.saturating_add(T::DbWeight::get().reads(10_u64))
			.saturating_add(T::DbWeight::get().writes(5_u64))
	}
	// same as vaults withdraw plus 1 more read
	fn vault_withdraw() -> Weight {
		Weight::from_ref_time(112_296_000_u64)
			.saturating_add(T::DbWeight::get().reads(9_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
    // Storage: Oracle Prices (r:2 w:0)
	// Storage: Lending LendingCount (r:1 w:1)
	// Storage: Vault VaultCount (r:1 w:1)
	// Storage: CurrencyFactory AssetIdRanges (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	// Storage: Vault LpTokensToVaults (r:0 w:1)
	// Storage: Vault Vaults (r:0 w:1)
	// Storage: Vault CapitalStructure (r:0 w:1)
	// Storage: Lending DebtTokenForMarket (r:0 w:1)
	// Storage: Lending BorrowIndex (r:0 w:1)
	// Storage: Lending Markets (r:0 w:1)
	fn create_market() -> Weight {
		Weight::from_ref_time(193_469_000_u64)
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(12_u64))
	}
	// Storage: Lending Markets (r:1 w:0)
	// Storage: Lending AccountCollateral (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn deposit_collateral() -> Weight {
		Weight::from_ref_time(124_740_000_u64)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Lending Markets (r:1 w:0)
	// Storage: Lending AccountCollateral (r:1 w:1)
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Lending DebtTokenForMarket (r:1 w:0)
	// Storage: Lending DebtIndex (r:1 w:0)
	// Storage: Oracle PriceHistory (r:2 w:0)
	// Storage: Oracle Prices (r:2 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	fn withdraw_collateral() -> Weight {
		Weight::from_ref_time(156_615_000_u64)
			.saturating_add(T::DbWeight::get().reads(11_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Lending Markets (r:1 w:0)
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Oracle Prices (r:2 w:0)
	// Storage: Lending DebtTokenForMarket (r:1 w:0)
	// Storage: Lending BorrowTimestamp (r:1 w:1)
	// Storage: Lending AccountCollateral (r:1 w:0)
	// Storage: Oracle PriceHistory (r:2 w:0)
	// Storage: Lending DebtIndex (r:1 w:1)
	// Storage: Vault CapitalStructure (r:2 w:0)
	// Storage: Tokens Accounts (r:4 w:3)
	// Storage: Lending BorrowIndex (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: Lending LastBlockTimestamp (r:1 w:0)
	// Storage: Lending BorrowRent (r:1 w:1)
	fn borrow() -> Weight {
		Weight::from_ref_time(419_169_000_u64)
			.saturating_add(T::DbWeight::get().reads(21_u64))
			.saturating_add(T::DbWeight::get().writes(8_u64))
	}
	// Storage: Lending BorrowTimestamp (r:1 w:1)
	// Storage: Lending LastBlockTimestamp (r:1 w:0)
	// Storage: Lending DebtTokenForMarket (r:1 w:0)
	// Storage: Lending DebtIndex (r:1 w:1)
	// Storage: Lending BorrowIndex (r:1 w:0)
	// Storage: Tokens Accounts (r:4 w:3)
	// Storage: Lending Markets (r:1 w:0)
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:1)
	// Storage: Lending BorrowRent (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	fn repay_borrow() -> Weight {
		Weight::from_ref_time(301_514_000_u64)
			.saturating_add(T::DbWeight::get().reads(14_u64))
			.saturating_add(T::DbWeight::get().writes(7_u64))
	}
	// Storage: Lending Markets (r:1 w:0)
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Lending AccountCollateral (r:1 w:0)
	// Storage: Oracle PriceHistory (r:2 w:0)
	// Storage: Oracle Prices (r:2 w:0)
	// Storage: Lending DebtTokenForMarket (r:1 w:0)
	// Storage: Lending DebtIndex (r:1 w:0)
	/// The range of component `b` is `[1, 1000]`.
	fn liquidate(b: u32, ) -> Weight {
		Weight::from_ref_time(577_340_000_u64)
			// Standard Error: 266_000
			.saturating_add(Weight::from_ref_time(35_069_000_u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(9_u64))
	}
	// Storage: Timestamp Now (r:1 w:0)
	fn now() -> Weight {
		Weight::from_ref_time(4_457_000_u64)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	// Storage: Lending DebtTokenForMarket (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:0)
	// Storage: Tokens TotalIssuance (r:1 w:0)
	// Storage: Lending Markets (r:1 w:1)
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Lending LastBlockTimestamp (r:1 w:0)
	// Storage: Lending BorrowIndex (r:1 w:1)
	/// The range of component `x` is `[1, 1000]`.
	fn accrue_interest(x: u32, ) -> Weight {
		Weight::from_ref_time(89_649_000_u64)
			// Standard Error: 1_000
			.saturating_add(Weight::from_ref_time(7_000_u64).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(8_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	fn account_id() -> Weight {
		Weight::from_ref_time(1_760_000_u64)
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Vault CapitalStructure (r:1 w:0)
	fn available_funds() -> Weight {
		Weight::from_ref_time(19_563_000_u64)
			.saturating_add(T::DbWeight::get().reads(2_u64))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Vault CapitalStructure (r:1 w:1)
	// Storage: Tokens Accounts (r:2 w:2)
	fn handle_withdrawable() -> Weight {
		Weight::from_ref_time(72_570_000_u64)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Vault CapitalStructure (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn handle_depositable() -> Weight {
		Weight::from_ref_time(125_535_000_u64)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	// Storage: Vault Vaults (r:1 w:0)
	// Storage: Tokens Accounts (r:2 w:2)
	// Storage: Vault CapitalStructure (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn handle_must_liquidate() -> Weight {
		Weight::from_ref_time(112_571_000_u64)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}
