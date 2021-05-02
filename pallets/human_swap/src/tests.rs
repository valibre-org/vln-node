use crate::mock::*;
use crate::Swaps;
use frame_support::{assert_noop, assert_ok};
use orml_traits::MultiCurrency;
use sp_runtime::{FixedPointNumber, FixedU128, Permill};
use vln_primitives::*;

#[test]
fn test_swap_in_lifecycle() {
    new_test_ext().execute_with(|| {
        let base = 1;
        let quote = 2;
        let amount = 10;
        let swap_owner = 1;
        let expected_swap_id = 1;

        // update provider price
        assert_ok!(RatePallet::update_price(
            Origin::signed(PROVIDER_ONE),
            base,
            quote,
            PaymentMethod::BankX,
            Permill::from_percent(1)
        ),);

        // ------------create swap-------------------
        assert_ok!(HumanSwap::create_swap_in(
            Origin::signed(swap_owner),
            base,
            quote,
            PaymentMethod::BankX,
            amount,
            PROVIDER_ONE
        ),);

        assert_eq!(
            Swaps::<Test>::get(swap_owner, expected_swap_id),
            Some(Swap {
                human: PROVIDER_ONE,
                kind: SwapKind::In(SwapIn::Created),
                price: PairPrice {
                    pair: AssetPair { base, quote },
                    price: FixedU128::zero(),
                },
                amount,
            })
        );

        // -------provider accepts swap-----------------
        assert_ok!(HumanSwap::provider_process_swap_in(
            Origin::signed(PROVIDER_ONE),
            swap_owner,
            expected_swap_id,
            SwapIn::Accepted(vec![])
        ),);

        assert_eq!(
            Swaps::<Test>::get(swap_owner, expected_swap_id),
            Some(Swap {
                human: PROVIDER_ONE,
                kind: SwapKind::In(SwapIn::Accepted(vec![])),
                price: PairPrice {
                    pair: AssetPair { base, quote },
                    price: FixedU128::zero(),
                },
                amount,
            })
        );
        // ensure the balances are reserved from provider account
        assert_eq!(Tokens::total_balance(2, &PROVIDER_ONE), 100);
        assert_eq!(Tokens::free_balance(2, &PROVIDER_ONE), 90);

        // -------user confirms swap-----------------
        assert_ok!(HumanSwap::confirm_swap_in(
            Origin::signed(swap_owner),
            expected_swap_id,
            vec![]
        ),);

        assert_eq!(
            Swaps::<Test>::get(swap_owner, expected_swap_id),
            Some(Swap {
                human: PROVIDER_ONE,
                kind: SwapKind::In(SwapIn::Confirmed(vec![])),
                price: PairPrice {
                    pair: AssetPair { base, quote },
                    price: FixedU128::zero(),
                },
                amount,
            })
        );
        // ensure the balances are reserved from provider account
        assert_eq!(Tokens::total_balance(2, &PROVIDER_ONE), 100);
        assert_eq!(Tokens::free_balance(2, &PROVIDER_ONE), 90);

        // -------provider completes swapin-----------------
        assert_ok!(HumanSwap::complete_swap_in(
            Origin::signed(PROVIDER_ONE),
            swap_owner,
            expected_swap_id,
        ),);

        assert_eq!(
            Swaps::<Test>::get(swap_owner, expected_swap_id),
            Some(Swap {
                human: PROVIDER_ONE,
                kind: SwapKind::In(SwapIn::Completed),
                price: PairPrice {
                    pair: AssetPair { base, quote },
                    price: FixedU128::zero(),
                },
                amount,
            })
        );
        // ensure the balances are reserved from provider account
        assert_eq!(Tokens::total_balance(2, &PROVIDER_ONE), 90);
        assert_eq!(Tokens::free_balance(2, &PROVIDER_ONE), 90);
        assert_eq!(Tokens::free_balance(2, &swap_owner), 110);
    });
}

#[test]
fn it_works_for_swap_out_create() {
    new_test_ext().execute_with(|| {
        // update provider price
        assert_ok!(RatePallet::update_price(
            Origin::signed(PROVIDER_ONE),
            1,
            2,
            PaymentMethod::BankX,
            Permill::from_percent(1)
        ),);

        assert_ok!(HumanSwap::create_swap_out(
            Origin::signed(1),
            1,
            2,
            PaymentMethod::BankX,
            10,
            PROVIDER_ONE
        ),);

        assert_eq!(
            Swaps::<Test>::get(1, 1),
            Some(Swap {
                human: PROVIDER_ONE,
                kind: SwapKind::Out(SwapOut::Created),
                price: PairPrice {
                    pair: AssetPair { base: 1, quote: 2 },
                    price: FixedU128::zero(),
                },
                amount: 10,
            })
        );

        assert_eq!(Tokens::total_balance(2, &1), 100);
        assert_eq!(Tokens::free_balance(2, &1), 90);
    });
}
