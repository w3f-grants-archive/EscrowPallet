use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use crate::Contract;


#[test]
fn sign_contract_test1() {
    new_test_ext().execute_with(|| {

            const ALICE: u64 = 2;
            const BOB: u64 = 2;

            let origin = Origin::signed(ALICE);
            let to = BOB;
            let amount = 4000;
            let work_days = 2;
            let take_action_days = 3;

            assert_noop!(Escrow::sign_contract(
			origin,
			to,
			amount,
			work_days,
			take_action_days
		),
			Error::<Test>::SameAddressError);

    })
}

#[test]
fn sign_contract_test2() {
    new_test_ext().execute_with(|| {


            const ALICE: u64 = 1;
            const BOB: u64 = 2;

            let origin = Origin::signed(ALICE);
            let to = BOB;
            let amount = 4000;
            let work_days = 2;
            let take_action_days = 3;

            assert_ok!(Escrow::sign_contract(
			origin.clone(),
			to,
			amount.clone(),
			work_days,
			take_action_days));

    })
}

#[test]
fn sign_contract_test3() {
    new_test_ext().execute_with(|| {

            const ALICE: u64 = 1;
            const BOB: u64 = 2;
            let origin = Origin::signed(ALICE);
            let to = BOB;
            let amount = 4000;
            let work_days: u64 = 5761;
            let take_action_days: u64 = 14401;

           assert_ok!(Escrow::sign_contract(
			origin.clone(),
			to.clone(),
			amount.clone(),
			work_days.clone(),
			take_action_days.clone()
		    ));

         let contract = Contract {
            origin: ALICE,
            to: BOB,
            amount,
            current_block_number: 0,
            work_days_in_block_number: 82958400,
            take_action_days_in_block: 290332800,
        };

        assert_eq!(Escrow::contract_sender(ALICE), Some(contract.clone()));

        assert_eq!(Escrow::contract_receiver(BOB), Some(contract.clone()));

    })
}
