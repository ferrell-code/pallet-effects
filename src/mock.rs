#![cfg(test)]

use super::*;
use frame_support::{
    parameter_types,
    traits::{ConstU128, ConstU32, ConstU64, Everything},
};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, AccountId32};

pub type AccountId = AccountId32;
impl frame_system::Config for Runtime {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = ConstU64<250>;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = Everything;
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

type Balance = u128;

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ConstU128<2>;
    type AccountStore = System;
    type MaxLocks = ();
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = ();
    type WeightInfo = ();
}

parameter_types! {
    // Derives are nice for ease of use in testing
    #[derive(Clone, PartialEq, RuntimeDebug)]
    pub const MaxArgs: u32 = 5;
    #[derive(Clone, PartialEq, RuntimeDebug)]
    pub const MaxBytesPerArg: u32 = 32;
    #[derive(Clone, PartialEq, RuntimeDebug)]
    pub const MaxSideEffects: u32 = 5;
}

impl super::Config for Runtime {
    type Event = Event;
    type MaxArgs = MaxArgs;
    type MaxBytesPerArg = MaxBytesPerArg;
    type MaxSideEffects = MaxSideEffects;
}

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, Call, u32, ()>;

frame_support::construct_runtime!(
    pub enum Runtime where
    Block = Block,
    NodeBlock = Block,
    UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        VolatileEffects: crate::{Pallet, Storage, Event<T>}
    }
);

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
