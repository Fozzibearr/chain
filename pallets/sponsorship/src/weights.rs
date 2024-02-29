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

//! Autogenerated weights for pallet_sponsorship
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-02-20, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `chain-bench-a18ada46`, CPU: `AMD EPYC 7B13`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_sponsorship
// --extrinsic=*
// --wasm-execution=compiled
// --template=./.maintain/internal_pallet_weights.hbs
// --output=temp_weights
use core::marker::PhantomData;
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};

/// Weight functions needed for pallet_sponsorship.
pub trait WeightInfo {
	fn create_pot() -> Weight;
	fn remove_pot() -> Weight;
	fn update_pot_limits() -> Weight;
	fn update_sponsorship_type() -> Weight;
	fn register_users(l: u32) -> Weight;
	fn remove_users(l: u32) -> Weight;
	fn update_users_limits(l: u32) -> Weight;
	fn pre_sponsor() -> Weight;
	fn post_sponsor() -> Weight;
}

/// Weight functions for `pallet_sponsorship`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_pot() -> Weight {
		// Minimum execution time: 32_630 nanoseconds.
		Weight::from_parts(33_760_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_pot() -> Weight {
		// Minimum execution time: 23_490 nanoseconds.
		Weight::from_parts(24_700_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_pot_limits() -> Weight {
		// Minimum execution time: 17_471 nanoseconds.
		Weight::from_parts(17_940_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_sponsorship_type() -> Weight {
		// Minimum execution time: 14_750 nanoseconds.
		Weight::from_parts(15_220_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn register_users(l: u32) -> Weight {
		// Minimum execution time: 55_171 nanoseconds.
		Weight::from_parts(56_590_000_u64, 0)
			// Standard Error: 7_532
			.saturating_add(Weight::from_parts(35_302_171_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn remove_users(l: u32) -> Weight {
		// Minimum execution time: 117_730 nanoseconds.
		Weight::from_parts(119_071_000_u64, 0)
			// Standard Error: 41_981
			.saturating_add(Weight::from_parts(104_362_191_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn update_users_limits(l: u32) -> Weight {
		// Minimum execution time: 26_990 nanoseconds.
		Weight::from_parts(27_370_000_u64, 0)
			// Standard Error: 9_187
			.saturating_add(Weight::from_parts(9_058_572_u64, 0).saturating_mul(l as u64))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn pre_sponsor() -> Weight {
		// Minimum execution time: 68_110 nanoseconds.
		Weight::from_parts(69_520_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:0 w:1)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::Pot` (r:0 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn post_sponsor() -> Weight {
		// Minimum execution time: 60_820 nanoseconds.
		Weight::from_parts(61_909_000_u64, 0)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
}

impl WeightInfo for () {
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_pot() -> Weight {
		// Minimum execution time: 32_630 nanoseconds.
		Weight::from_parts(33_760_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn remove_pot() -> Weight {
		// Minimum execution time: 23_490 nanoseconds.
		Weight::from_parts(24_700_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_pot_limits() -> Weight {
		// Minimum execution time: 17_471 nanoseconds.
		Weight::from_parts(17_940_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn update_sponsorship_type() -> Weight {
		// Minimum execution time: 14_750 nanoseconds.
		Weight::from_parts(15_220_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `l` is `[1, 1000]`.
	fn register_users(l: u32) -> Weight {
		// Minimum execution time: 55_171 nanoseconds.
		Weight::from_parts(56_590_000_u64, 0)
			// Standard Error: 7_532
			.saturating_add(Weight::from_parts(35_302_171_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:1998 w:1998)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::UserRegistrationCount` (r:999 w:999)
	// Proof: `Sponsorship::UserRegistrationCount` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn remove_users(l: u32) -> Weight {
		// Minimum execution time: 117_730 nanoseconds.
		Weight::from_parts(119_071_000_u64, 0)
			// Standard Error: 41_981
			.saturating_add(Weight::from_parts(104_362_191_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((4_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((4_u64).saturating_mul(l as u64)))
	}
	// Storage: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Proof: UNKNOWN KEY `0x4d95b5e86eb03e9e163361bfe841137d4e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
	// Storage: `Sponsorship::Pot` (r:1 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:999 w:999)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `l` is `[1, 1000]`.
	fn update_users_limits(l: u32) -> Weight {
		// Minimum execution time: 26_990 nanoseconds.
		Weight::from_parts(27_370_000_u64, 0)
			// Standard Error: 9_187
			.saturating_add(Weight::from_parts(9_058_572_u64, 0).saturating_mul(l as u64))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(l as u64)))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(l as u64)))
	}
	// Storage: `Sponsorship::Pot` (r:1 w:0)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::User` (r:1 w:0)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn pre_sponsor() -> Weight {
		// Minimum execution time: 68_110 nanoseconds.
		Weight::from_parts(69_520_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `Sponsorship::User` (r:0 w:1)
	// Proof: `Sponsorship::User` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Sponsorship::Pot` (r:0 w:1)
	// Proof: `Sponsorship::Pot` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn post_sponsor() -> Weight {
		// Minimum execution time: 60_820 nanoseconds.
		Weight::from_parts(61_909_000_u64, 0)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
}
