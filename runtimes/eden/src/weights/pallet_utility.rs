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

//! Autogenerated weights for pallet_utility
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-29, STEPS: `4`, REPEAT: 4, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `tama`, CPU: `11th Gen Intel(R) Core(TM) i7-11700 @ 2.50GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=4
// --repeat=4
// --pallet=pallet_utility
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 7_848 nanoseconds.
		Weight::from_parts(21_850_593_u64, 0)
			// Standard Error: 49_325
			.saturating_add(Weight::from_parts(4_504_028_u64, 0).saturating_mul(c as u64))
	}
	fn as_derivative() -> Weight {
		// Minimum execution time: 7_380 nanoseconds.
		Weight::from_parts(8_172_000_u64, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 7_422 nanoseconds.
		Weight::from_parts(15_607_745_u64, 0)
			// Standard Error: 15_810
			.saturating_add(Weight::from_parts(4_857_144_u64, 0).saturating_mul(c as u64))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 10_948 nanoseconds.
		Weight::from_parts(11_628_000_u64, 0)
	}
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 7_505 nanoseconds.
		Weight::from_parts(8_635_504_u64, 0)
			// Standard Error: 40_629
			.saturating_add(Weight::from_parts(4_595_714_u64, 0).saturating_mul(c as u64))
	}
}
