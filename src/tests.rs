#![cfg(test)]

use super::{Event, *};
use crate::mock::*;
use frame_support::{assert_err, assert_ok, bounded_vec};

#[test]
fn encode_decode_sanity_check() {
    let encoded_chain = ChainId::Polkadot.encode();
    assert_eq!(encoded_chain, [0_u8]);

    let encoded_chain2 = ChainId::T3rn.encode();
    assert_eq!(encoded_chain2, [3_u8]);

    // decodes back to chain id
    let decoded_chain: ChainId = Decode::decode(&mut &encoded_chain[..]).unwrap();
    assert_eq!(decoded_chain, ChainId::Polkadot);

    let decoded_chain: ChainId = Decode::decode(&mut &encoded_chain2[..]).unwrap();
    assert_eq!(decoded_chain, ChainId::T3rn);
}

#[test]
fn decode_to_side_effects_fails() {
    new_test_ext().execute_with(|| {
        // decodes as empty vec fails
        let raw_bytes: Vec<u8> = vec![0];
        assert_err!(
            VolatileEffects::decode_to_side_effects(raw_bytes),
            Error::<Runtime>::DecodesToNothing
        );

        // decode fails for value that violates bounded vec
        let raw_bytes = vec![100; 1000];
        assert_err!(
            VolatileEffects::decode_to_side_effects(raw_bytes),
            Error::<Runtime>::CannotDecodeValue
        );
    });
}

#[test]
fn decode_to_side_effect_works() {
    new_test_ext().execute_with(|| {
        let effect = SideEffectOf::<Runtime>::new(ChainId::Polkadot, Action::Swap, bounded_vec![]);
        let zero_account: BoundedVec<u8, MaxBytesPerArg> = bounded_vec![0; 32];
        let example_args: BoundedVec<BoundedVec<u8, MaxBytesPerArg>, MaxArgs> = bounded_vec![
            zero_account.clone(),
            zero_account.clone(),
            bounded_vec![1],
            bounded_vec![1]
        ];
        let effect_with_args =
            SideEffectOf::<Runtime>::new(ChainId::T3rn, Action::MultiTran, example_args.clone());

        let two_effects = vec![effect.clone(); 2];
        let two_effect_with_args = vec![effect_with_args.clone(); 2];

        // For explicit purposes. As you can see what effects encode to. Only additional overhead is that associated with the vec.
        let raw_bytes_effect: Vec<u8> = vec![0, 0, 0];
        let raw_bytes_effect_args: Vec<u8> = vec![
            3, 2, 16, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 4, 1,
        ];

        // Can handle multiple side effects encoded
        let raw_bytes_two: Vec<u8> = vec![8, 0, 0, 0, 0, 0, 0];
        let raw_bytes_two_with_args: Vec<u8> = vec![
            8, 3, 2, 16, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 4, 1, 3, 2, 16, 128, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 4, 1, 4, 1,
        ];

        assert_eq!(raw_bytes_effect, effect.encode());
        assert_eq!(raw_bytes_effect_args, effect_with_args.encode());

        assert_eq!(raw_bytes_two, two_effects.encode());
        assert_eq!(raw_bytes_two_with_args, two_effect_with_args.encode())
    });
}

#[test]
fn test_update_side_effects_trait() {
    new_test_ext().execute_with(|| {
        // Will convert these scale encoded raw bytes into side effects
        let raw_bytes: Vec<u8> = vec![4, 0, 0, 0];
        let raw_bytes_with_args: Vec<u8> = vec![
            8, 3, 2, 16, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 1, 4, 1, 3, 2, 16, 128, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 4, 1, 4, 1,
        ];
        let zero_account: Vec<u8> = vec![0; 32];

        assert_ok!(VolatileEffects::add_pending_side_effects(raw_bytes));
        assert_eq!(ExecutionCounter::<Runtime>::get(), 1);
        System::assert_last_event(Event::<Runtime>::SideEffectsPending { execution_id: 0 }.into());
        let side_effect_vec = ExecutionPending::<Runtime>::get(0).unwrap();
        let side_effect = &side_effect_vec[0];

        assert_eq!(side_effect.action, Action::Swap);
        assert_eq!(side_effect.chain, ChainId::Polkadot);
        assert_eq!(side_effect.args, vec![]);

        assert_ok!(VolatileEffects::add_pending_side_effects(
            raw_bytes_with_args
        ));
        assert_eq!(ExecutionCounter::<Runtime>::get(), 2);
        System::assert_last_event(Event::<Runtime>::SideEffectsPending { execution_id: 1 }.into());
        let side_effect_vec = ExecutionPending::<Runtime>::get(1).unwrap();
        // Should decode to two `SideEffects`
        assert_eq!(side_effect_vec.len(), 2);

        let side_effect_1 = &side_effect_vec[0];
        assert_eq!(side_effect_1.chain, ChainId::T3rn);
        assert_eq!(side_effect_1.action, Action::MultiTran);
        assert_eq!(
            side_effect_1
                .args
                .iter()
                .map(|x| x.clone().into_inner())
                .collect::<Vec<Vec<u8>>>(),
            vec![zero_account.clone(), zero_account.clone(), vec![1], vec![1]]
        );

        // Revert works
        VolatileEffects::revert_side_effects(0);
        System::assert_last_event(Event::<Runtime>::SideEffectsReverted { execution_id: 0 }.into());
        assert!(ExecutionPending::<Runtime>::get(0).is_none());
    });
}
