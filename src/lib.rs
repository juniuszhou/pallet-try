#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use serde_derive::{Deserialize, Serialize};

use codec::{Codec, Decode, Encode};
use frame_support::storage::migration::{put_storage_value, StorageIterator};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage,
    dispatch::DispatchResult,
    traits::{Currency, Get, LockableCurrency, ReservableCurrency},
    Parameter,
};
use rstd::prelude::*;
use sp_runtime;
use sp_runtime::traits::{AtLeast32Bit, MaybeSerialize, Member, One, Saturating, Zero};
use system;
use system::ensure_signed;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod migration;

/// Forum data storage version
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
enum Releases {
    V1_0_0,
    V2_0_0,
}

impl Default for Releases {
    fn default() -> Self {
        Releases::V1_0_0
    }
}

type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as system::Trait>::AccountId>>::Balance;

pub trait Trait: system::Trait + timestamp::Trait + Sized {
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
    type ForumUserId: Parameter
        + Member
        + AtLeast32Bit
        + Codec
        + Default
        + Copy
        + MaybeSerialize
        + PartialEq
        + From<u64>
        + Into<u64>;

    // Fee for create a thread
    type DepositPerThread: Get<BalanceOf<Self>>;

    // Fee for create a post
    type DepositPerPost: Get<BalanceOf<Self>>;

    // Fee for store one byte in title
    type DepositTitlePerByte: Get<BalanceOf<Self>>;

    // Fee for store one byte in text
    type DepositTextPerByte: Get<BalanceOf<Self>>;

    // Currency type for this module.
    type Currency: ReservableCurrency<Self::AccountId>
        + LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;
}

decl_error! {
    pub enum Error for Module<T: Trait> {
        InvalidTextLength,
        ForumSudoNotSet,
    }
}

/// Convenient composite time stamp
#[cfg_attr(feature = "std", derive(Serialize, Deserialize, Debug))]
#[derive(Encode, Decode, Default, Clone, PartialEq, Eq)]
pub struct BlockchainTimestamp<BlockNumber, Moment> {
    /// Current block number
    pub block: BlockNumber,

    /// Time of block created
    pub time: Moment,
}

decl_storage! {
    trait Store for Module<T: Trait> as Forum {
        /// Storage version of forum pallet
        StorageVersion build(|_: &GenesisConfig| Releases::V2_0_0): Releases;
    }
}

decl_event!(
    pub enum Event<T>
    where
        <T as system::Trait>::AccountId,
    {
        /// A category was introduced
        CategoryCreated(AccountId),
    }
);

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;
        /// Migration all data during runtime upgrade
        fn on_runtime_upgrade() {
            // Migrate all categories, threads and posts
            let _ = migration::on_runtime_upgrade::<T>();
        }
    }
}

impl<T: Trait> Module<T> {
    // The method only called from other module to create a forum user.
    pub fn create_forum_user(account_id: T::AccountId) -> DispatchResult {
        Ok(())
    }

    fn current_block_and_time() -> BlockchainTimestamp<T::BlockNumber, T::Moment> {
        BlockchainTimestamp {
            block: <system::Module<T>>::block_number(),
            time: <timestamp::Module<T>>::now(),
        }
    }
}
