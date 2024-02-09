/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-12-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-b606df9f`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
    fn add_username_authority() -> cumulus_primitives_core::Weight { Weight::from_parts(14_034_304_u64, 0) }
    fn remove_username_authority() -> cumulus_primitives_core::Weight { Weight::from_parts(14_034_304_u64, 0) }
    fn set_username_for() -> cumulus_primitives_core::Weight { Weight::from_parts(14_034_304_u64, 0) }
    fn accept_username() -> cumulus_primitives_core::Weight { Weight::from_parts(14_034_304_u64, 0) }
    fn remove_expired_approval() -> cumulus_primitives_core::Weight { Weight::from_parts(14_034_304_u64, 0) }
    fn set_primary_username() -> cumulus_primitives_core::Weight { Weight::from_parts(14_034_304_u64, 0) }
    fn remove_dangling_username() -> cumulus_primitives_core::Weight { Weight::from_parts(14_034_304_u64, 0) }


	// Storage: `Identity::Registrars` (r:1 w:1)
	// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Minimum execution time: 12_670 nanoseconds.
		Weight::from_parts(14_034_304_u64, 0)
			// Standard Error: 2_544
			.saturating_add(Weight::from_parts(98_466_u64, 0).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::IdentityOf` (r:1 w:1)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn set_identity(r: u32 ) -> Weight {
		// Minimum execution time: 41_840 nanoseconds.
		Weight::from_parts(40_364_406_u64, 0)
			// Standard Error: 7_707
			.saturating_add(Weight::from_parts(114_338_u64, 0).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::IdentityOf` (r:1 w:0)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	// Storage: `Identity::SubsOf` (r:1 w:1)
	// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	// Storage: `Identity::SuperOf` (r:100 w:100)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Minimum execution time: 14_170 nanoseconds.
		Weight::from_parts(31_604_174_u64, 0)
			// Standard Error: 5_463
			.saturating_add(Weight::from_parts(4_676_799_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: `Identity::IdentityOf` (r:1 w:0)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	// Storage: `Identity::SubsOf` (r:1 w:1)
	// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	// Storage: `Identity::SuperOf` (r:0 w:100)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Minimum execution time: 14_069 nanoseconds.
		Weight::from_parts(31_129_179_u64, 0)
			// Standard Error: 4_536
			.saturating_add(Weight::from_parts(1_820_761_u64, 0).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p as u64)))
	}
	// Storage: `Identity::SubsOf` (r:1 w:1)
	// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityOf` (r:1 w:1)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	// Storage: `Identity::SuperOf` (r:0 w:100)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32 ) -> Weight {
		// Minimum execution time: 80_480 nanoseconds.
		Weight::from_parts(40_018_329_u64, 0)
			// Standard Error: 15_796
			.saturating_add(Weight::from_parts(134_584_u64, 0).saturating_mul(r as u64))
			// Standard Error: 3_084
			.saturating_add(Weight::from_parts(1_834_550_u64, 0).saturating_mul(s as u64))
			
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: `Identity::Registrars` (r:1 w:0)
	// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityOf` (r:1 w:1)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn request_judgement(r: u32 ) -> Weight {
		// Minimum execution time: 40_670 nanoseconds.
		Weight::from_parts(37_656_928_u64, 0)
			// Standard Error: 14_311
			.saturating_add(Weight::from_parts(179_668_u64, 0).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::IdentityOf` (r:1 w:1)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `x` is `[0, 100]`.
	fn cancel_request(x: u32, ) -> Weight {
		// Minimum execution time: 37_500 nanoseconds.
		Weight::from_parts(41_493_197_u64, 0)
			// Standard Error: 3_327
			.saturating_add(Weight::from_parts(809_129_u64, 0).saturating_mul(x as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::Registrars` (r:1 w:1)
	// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Minimum execution time: 9_120 nanoseconds.
		Weight::from_parts(10_049_275_u64, 0)
			// Standard Error: 1_944
			.saturating_add(Weight::from_parts(64_588_u64, 0).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::Registrars` (r:1 w:1)
	// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Minimum execution time: 9_480 nanoseconds.
		Weight::from_parts(10_253_357_u64, 0)
			// Standard Error: 1_837
			.saturating_add(Weight::from_parts(68_072_u64, 0).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::Registrars` (r:1 w:1)
	// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Minimum execution time: 9_110 nanoseconds.
		Weight::from_parts(9_854_170_u64, 0)
			// Standard Error: 1_675
			.saturating_add(Weight::from_parts(75_738_u64, 0).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::Registrars` (r:1 w:0)
	// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityOf` (r:1 w:1)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	/// The range of component `x` is `[0, 100]`.
	fn provide_judgement(r: u32) -> Weight {
		// Minimum execution time: 29_330 nanoseconds.
		Weight::from_parts(28_995_803_u64, 0)
			// Standard Error: 12_882
			.saturating_add(Weight::from_parts(49_319_u64, 0).saturating_mul(r as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::SubsOf` (r:1 w:1)
	// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	// Storage: `Identity::IdentityOf` (r:1 w:1)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Identity::SuperOf` (r:0 w:100)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	/// The range of component `x` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32 ) -> Weight {
		// Minimum execution time: 101_580 nanoseconds.
		Weight::from_parts(60_360_110_u64, 0)
			// Standard Error: 13_442
			.saturating_add(Weight::from_parts(131_610_u64, 0).saturating_mul(r as u64))
			// Standard Error: 2_625
			.saturating_add(Weight::from_parts(1_830_071_u64, 0).saturating_mul(s as u64))
			// Standard Error: 2_625
			.saturating_add(Weight::from_parts(442_446_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s as u64)))
	}
	// Storage: `Identity::IdentityOf` (r:1 w:0)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	// Storage: `Identity::SuperOf` (r:1 w:1)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	// Storage: `Identity::SubsOf` (r:1 w:1)
	// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Minimum execution time: 36_680 nanoseconds.
		Weight::from_parts(42_151_516_u64, 0)
			// Standard Error: 1_522
			.saturating_add(Weight::from_parts(75_511_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Identity::IdentityOf` (r:1 w:0)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	// Storage: `Identity::SuperOf` (r:1 w:1)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Minimum execution time: 18_651 nanoseconds.
		Weight::from_parts(20_914_286_u64, 0)
			// Standard Error: 856
			.saturating_add(Weight::from_parts(27_833_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Identity::IdentityOf` (r:1 w:0)
	// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7538), added: 10013, mode: `MaxEncodedLen`)
	// Storage: `Identity::SuperOf` (r:1 w:1)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	// Storage: `Identity::SubsOf` (r:1 w:1)
	// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Minimum execution time: 40_250 nanoseconds.
		Weight::from_parts(44_097_178_u64, 0)
			// Standard Error: 1_097
			.saturating_add(Weight::from_parts(59_863_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Identity::SuperOf` (r:1 w:1)
	// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	// Storage: `Identity::SubsOf` (r:1 w:1)
	// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:0)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Minimum execution time: 29_760 nanoseconds.
		Weight::from_parts(32_244_934_u64, 0)
			// Standard Error: 919
			.saturating_add(Weight::from_parts(66_067_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
