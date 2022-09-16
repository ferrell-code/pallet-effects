#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::*;
use sp_runtime::{
    traits::{One, Zero},
    ArithmeticError, RuntimeDebug,
};

mod mock;
mod tests;

use pallet::*;

/// Trait to update, commit, or revert side effects. This is the API external pallets will interface with.
pub trait UpdateSideEffects {
    /// Decodes raw bytes and adds side effects to `ExecutionPending` storage. As the name suggests the side effects are not yet executed and waiting for an external actor. Note the arguments are not guarenteed to be valid, in the case of invalid arguements it will revert when attempting to be executed
    fn add_pending_side_effects(encoded_side_effects: Vec<u8>) -> DispatchResult;

    /// Commits side effects as a batch. If one fails execution then all side effects are reverted, this is infaliable.
    fn execute_side_effects(execution_id: ExecutionId);

    /// Removes side effects from `PendingExecution` storage and does not execute them. This is infalliable
    fn revert_side_effects(execution_id: ExecutionId);
}

/// Represents different chains as a single byte
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
#[repr(u8)]
pub enum ChainId {
    Polkadot,
    Kusama,
    Rococo,
    T3rn,
}

/// Represents different potential actions as a single byte
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[repr(u8)]
pub enum Action {
    Swap,
    Tran,
    MultiTran,
}

/// Volatile side effects that can be executed
#[derive(Clone, Eq, PartialEq, Encode, Decode, RuntimeDebug, MaxEncodedLen, TypeInfo)]
#[scale_info(bounds(), skip_type_params(T, S))]
pub struct SideEffect<T: Get<u32>, S: Get<u32>> {
    chain: ChainId,
    action: Action,
    args: BoundedVec<Arguement<T>, S>,
}

impl<T: Get<u32>, S: Get<u32>> SideEffect<T, S> {
    pub fn new(id: ChainId, action: Action, args: BoundedVec<Arguement<T>, S>) -> Self {
        Self {
            chain: id,
            action,
            args,
        }
    }
}

pub type ExecutionId = u32;
pub type Arguement<T> = BoundedVec<u8, T>;

// Ease of use so we don't need to define bounds everytime
type SideEffectOf<T> = SideEffect<<T as Config>::MaxBytesPerArg, <T as Config>::MaxArgs>;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Overarching Event type
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Maximum number of arguements in a side effect (should be 5 for this particular exercise)
        #[pallet::constant]
        type MaxArgs: Get<u32>;

        /// Maximum number of bytes in an individual arguement
        #[pallet::constant]
        type MaxBytesPerArg: Get<u32>;

        /// Maximum number of side effects that can reside in a single step
        #[pallet::constant]
        type MaxSideEffects: Get<u32>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::error]
    pub enum Error<T> {
        CannotDecodeValue,
        DecodesToNothing,
        ExecutionFailed,
    }

    /// Counter for `ExecutionId`
    #[pallet::storage]
    pub type ExecutionCounter<T: Config> = StorageValue<_, ExecutionId, ValueQuery>;

    /// Maps the `ExecutionId` to a `BoundedVec` of side effects
    #[pallet::storage]
    pub type ExecutionPending<T: Config> = StorageMap<
        _,
        Twox64Concat,
        ExecutionId,
        BoundedVec<SideEffectOf<T>, T::MaxSideEffects>,
        OptionQuery,
    >;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Side effects are added to `ExecutionPending` storage
        SideEffectsPending { execution_id: ExecutionId },
        /// Side effects are committed and removed from `ExecutionPending` storage
        SideEffectsCommitted { execution_id: ExecutionId },
        /// Side effects are reverted (not executed) and removed from `ExecutionPending` storage
        SideEffectsReverted { execution_id: ExecutionId },
        /// Emits event when side effect is executed and fails
        SideEffectFailed {
            execution_id: ExecutionId,
            index: u32,
        },
    }
}

impl<T: Config> Pallet<T> {
    fn decode_to_side_effects(
        encoded_value: Vec<u8>,
    ) -> Result<BoundedVec<SideEffectOf<T>, T::MaxSideEffects>, DispatchError> {
        let side_effects: BoundedVec<SideEffectOf<T>, T::MaxSideEffects> =
            Decode::decode(&mut &encoded_value[..]).map_err(|_| Error::<T>::CannotDecodeValue)?;
        // Ensure it does not decode to empty vec
        ensure!(!side_effects.len().is_zero(), Error::<T>::DecodesToNothing);

        Ok(side_effects)
    }
}

impl<T: Config> UpdateSideEffects for Pallet<T> {
    fn add_pending_side_effects(encoded_value: Vec<u8>) -> DispatchResult {
        let side_effects = Self::decode_to_side_effects(encoded_value)?;

        let execution_id = ExecutionCounter::<T>::get();
        let increment_execution_id = execution_id
            .checked_add(One::one())
            .ok_or(ArithmeticError::Overflow)?;
        ExecutionCounter::<T>::set(increment_execution_id);
        ExecutionPending::<T>::insert(execution_id, side_effects);

        Self::deposit_event(Event::<T>::SideEffectsPending { execution_id });
        Ok(())
    }

    fn revert_side_effects(execution_id: ExecutionId) {
        ExecutionPending::<T>::remove(execution_id);
        Self::deposit_event(Event::<T>::SideEffectsReverted { execution_id });
    }

    fn execute_side_effects(_execution_id: ExecutionId) {
        unimplemented!()
    }
}
