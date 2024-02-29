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

//! Autogenerated weights for pallet_scheduler
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
// --pallet=pallet_scheduler
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/external_pallet_weights.hbs
// --output=runtimes/eden/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use core::marker::PhantomData;

/// Weight functions for `pallet_scheduler`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_scheduler::WeightInfo for WeightInfo<T> {
	// Storage: `Scheduler::IncompleteSince` (r:1 w:1)
	// Proof: `Scheduler::IncompleteSince` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn service_agendas_base() -> Weight {
		// Minimum execution time: 3_615 nanoseconds.
		Weight::from_parts(4_066_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 50]`.
	fn service_agenda_base(s: u32, ) -> Weight {
		// Minimum execution time: 4_892 nanoseconds.
		Weight::from_parts(6_649_678_u64, 0)
			// Standard Error: 30_336
			.saturating_add(Weight::from_parts(1_004_583_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_task_base() -> Weight {
		// Minimum execution time: 5_400 nanoseconds.
		Weight::from_parts(5_850_000_u64, 0)
	}
	// Storage: `Preimage::PreimageFor` (r:1 w:1)
	// Proof: `Preimage::PreimageFor` (`max_values`: None, `max_size`: Some(4194344), added: 4196819, mode: `Measured`)
	// Storage: `Preimage::StatusFor` (r:1 w:0)
	// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	// Storage: `Preimage::RequestStatusFor` (r:1 w:1)
	// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[128, 4194304]`.
	fn service_task_fetched(s: u32, ) -> Weight {
		// Minimum execution time: 24_650 nanoseconds.
		Weight::from_parts(49_271_000_u64, 0)
			// Standard Error: 9
			.saturating_add(Weight::from_parts(1_243_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Scheduler::Lookup` (r:0 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	fn service_task_named() -> Weight {
		// Minimum execution time: 7_621 nanoseconds.
		Weight::from_parts(8_375_000_u64, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn service_task_periodic() -> Weight {
		// Minimum execution time: 5_290 nanoseconds.
		Weight::from_parts(5_678_000_u64, 0)
	}
	fn execute_dispatch_signed() -> Weight {
		// Minimum execution time: 3_939 nanoseconds.
		Weight::from_parts(4_384_000_u64, 0)
	}
	fn execute_dispatch_unsigned() -> Weight {
		// Minimum execution time: 3_794 nanoseconds.
		Weight::from_parts(4_255_000_u64, 0)
	}
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 49]`.
	fn schedule(s: u32, ) -> Weight {
		// Minimum execution time: 14_416 nanoseconds.
		Weight::from_parts(12_657_427_u64, 0)
			// Standard Error: 103_092
			.saturating_add(Weight::from_parts(1_331_595_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Lookup` (r:0 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 50]`.
	fn cancel(s: u32, ) -> Weight {
		// Minimum execution time: 22_779 nanoseconds.
		Weight::from_parts(24_456_377_u64, 0)
			// Standard Error: 116_692
			.saturating_add(Weight::from_parts(1_608_237_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Scheduler::Lookup` (r:1 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 49]`.
	fn schedule_named(s: u32, ) -> Weight {
		// Minimum execution time: 19_506 nanoseconds.
		Weight::from_parts(23_270_134_u64, 0)
			// Standard Error: 152_321
			.saturating_add(Weight::from_parts(1_208_417_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `Scheduler::Lookup` (r:1 w:1)
	// Proof: `Scheduler::Lookup` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	// Storage: `Scheduler::Agenda` (r:1 w:1)
	// Proof: `Scheduler::Agenda` (`max_values`: None, `max_size`: Some(38963), added: 41438, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 50]`.
	fn cancel_named(s: u32, ) -> Weight {
		// Minimum execution time: 23_561 nanoseconds.
		Weight::from_parts(24_801_261_u64, 0)
			// Standard Error: 96_093
			.saturating_add(Weight::from_parts(1_690_068_u64, 0).saturating_mul(s as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}
