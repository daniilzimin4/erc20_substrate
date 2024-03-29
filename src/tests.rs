use crate::{mock::*, Allowance, BalanceOf, Error, Init, Pallet, TotalSupply};
use frame_support::{assert_err, assert_ok};
use frame_system::{RawOrigin};
use sp_runtime::{ArithmeticError, DispatchError};

#[test]
fn test_init() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let total_supply = 1_000_000_000;

		assert_ok!(Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply));

		assert_eq!(total_supply, TotalSupply::<Test>::try_get().unwrap());

		assert_eq!(total_supply, Pallet::<Test>::balance_of(&origin_id).unwrap());

		assert_eq!(true, Init::<Test>::try_get().unwrap());
	});
}

#[test]
fn test_init_already_initialized() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let total_supply = 1_000_000_000;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);

		assert_err!(
			Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply),
			Error::<Test>::AlreadyInitialized
		);
	});
}

#[test]
fn test_transfer() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let receiver_id = 2;
		let total_supply = 1_000_000_000;
		let amount_to_send = 1000;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);

		assert_eq!(total_supply, Pallet::<Test>::balance_of(&origin_id).unwrap());
		assert_eq!(0, Pallet::<Test>::balance_of(&receiver_id).unwrap_or(0));

		assert_ok!(Pallet::<Test>::transfer(
			RawOrigin::Signed(origin_id).into(),
			receiver_id,
			amount_to_send
		));

		assert_eq!(999_999_000, Pallet::<Test>::balance_of(&origin_id).unwrap());
		assert_eq!(amount_to_send, Pallet::<Test>::balance_of(&receiver_id).unwrap());
	});
}

#[test]
fn test_transfer_insufficient_funds() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let receiver_id = 2;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), 0);

		assert_eq!(0, Pallet::<Test>::balance_of(&origin_id).unwrap());
		assert_eq!(0, Pallet::<Test>::balance_of(&receiver_id).unwrap_or(0));

		assert_err!(
			Pallet::<Test>::transfer(RawOrigin::Signed(origin_id).into(), receiver_id, 1000),
			Error::<Test>::InsufficientFunds
		);

		assert_eq!(0, Pallet::<Test>::balance_of(&origin_id).unwrap());
		assert_eq!(0, Pallet::<Test>::balance_of(&receiver_id).unwrap_or(0));
	});
}

#[test]
fn test_transfer_balance_overflow() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let receiver_id = 2;
		let total_supply = 1_000_000_000;
		let amount_to_send = 1;
		let receiver_balance = u64::MAX;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);
		let _ = BalanceOf::<Test>::insert(&receiver_id, receiver_balance);

		assert_eq!(total_supply, Pallet::<Test>::balance_of(&origin_id).unwrap());
		assert_eq!(receiver_balance, Pallet::<Test>::balance_of(&receiver_id).unwrap_or(0));

		assert_err!(
			Pallet::<Test>::transfer(
				RawOrigin::Signed(origin_id).into(),
				receiver_id,
				amount_to_send
			),
			DispatchError::Arithmetic(ArithmeticError::Overflow)
		);

		assert_eq!(total_supply, Pallet::<Test>::balance_of(&origin_id).unwrap());
		assert_eq!(receiver_balance, Pallet::<Test>::balance_of(&receiver_id).unwrap_or(0));
	});
}

#[test]
fn test_approve() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let spender_id = 2;
		let total_supply = 1_000_000_000;
		let amount_to_approve = 1000;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);

		assert_ok!(Pallet::<Test>::approve(
			RawOrigin::Signed(origin_id).into(),
			spender_id,
			amount_to_approve
		));

		assert_eq!(amount_to_approve, Allowance::<Test>::try_get((origin_id, spender_id)).unwrap());
	});
}

#[test]
fn test_transfer_from() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let owner_id = 2;
		let receiver_id = 3;
		let total_supply = 1_000_000_000;
		let owner_balance = 1_000_000;
		let amount_to_approve = 2_000_000;
		let amount_to_send = 1000;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);
		let _ = Pallet::<Test>::approve(
			RawOrigin::Signed(owner_id).into(),
			origin_id,
			amount_to_approve,
		);
		let _ = BalanceOf::<Test>::insert(&owner_id, owner_balance);

		assert_ok!(Pallet::<Test>::transfer_from(
			RawOrigin::Signed(origin_id).into(),
			owner_id,
			receiver_id,
			amount_to_send
		));

		assert_eq!(999_000, Pallet::<Test>::balance_of(&owner_id).unwrap());
		assert_eq!(amount_to_send, Pallet::<Test>::balance_of(&receiver_id).unwrap());
		assert_eq!(1_999_000, Allowance::<Test>::try_get((owner_id, origin_id)).unwrap());
	});
}

#[test]
fn test_transfer_from_insufficient_approved() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let owner_id = 2;
		let receiver_id = 3;
		let total_supply = 1_000_000_000;
		let owner_balance = 1_000_000;
		let amount_to_send = 1;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);
		let _ = BalanceOf::<Test>::insert(&owner_id, owner_balance);

		assert_err!(
			Pallet::<Test>::transfer_from(
				RawOrigin::Signed(origin_id).into(),
				owner_id,
				receiver_id,
				amount_to_send
			),
			Error::<Test>::InsufficientApprovedFunds
		);

		assert_eq!(owner_balance, Pallet::<Test>::balance_of(&owner_id).unwrap());
		assert_eq!(0, Pallet::<Test>::balance_of(&receiver_id).unwrap_or(0));
		assert_eq!(0, Allowance::<Test>::try_get((owner_id, origin_id)).unwrap_or(0));
	});
}

#[test]
fn test_transfer_from_insufficient_owner_balance() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let owner_id = 2;
		let receiver_id = 3;
		let total_supply = 1_000_000_000;
		let amount_to_approve = 2_000_000;
		let amount_to_send = 1;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);
		let _ = Pallet::<Test>::approve(
			RawOrigin::Signed(owner_id).into(),
			origin_id,
			amount_to_approve,
		);

		assert_err!(
			Pallet::<Test>::transfer_from(
				RawOrigin::Signed(origin_id).into(),
				owner_id,
				receiver_id,
				amount_to_send
			),
			Error::<Test>::InsufficientFunds
		);

		assert_eq!(0, Pallet::<Test>::balance_of(&owner_id).unwrap_or(0));
		assert_eq!(0, Pallet::<Test>::balance_of(&receiver_id).unwrap_or(0));
		assert_eq!(amount_to_approve, Allowance::<Test>::try_get((owner_id, origin_id)).unwrap());
	});
}

#[test]
fn test_transfer_from_receiver_balance_overflow() {
	new_test_ext().execute_with(|| {
		let origin_id = 1;
		let owner_id = 2;
		let receiver_id = 3;
		let total_supply = 1_000_000_000;
		let amount_to_approve = 2_000_000;
		let owner_balance = 1_000_000;
		let receiver_balance = u64::MAX;
		let amount_to_send = 1;

		let _ = Pallet::<Test>::init(RawOrigin::Signed(origin_id).into(), total_supply);
		let _ = Pallet::<Test>::approve(
			RawOrigin::Signed(owner_id).into(),
			origin_id,
			amount_to_approve,
		);
		BalanceOf::<Test>::insert(&owner_id, owner_balance);
		BalanceOf::<Test>::insert(&receiver_id, receiver_balance);

		assert_err!(
			Pallet::<Test>::transfer_from(
				RawOrigin::Signed(origin_id).into(),
				owner_id,
				receiver_id,
				amount_to_send
			),
			DispatchError::Arithmetic(ArithmeticError::Overflow)
		);

		assert_eq!(owner_balance, Pallet::<Test>::balance_of(&owner_id).unwrap());
		assert_eq!(receiver_balance, Pallet::<Test>::balance_of(&receiver_id).unwrap());
		assert_eq!(amount_to_approve, Allowance::<Test>::try_get((owner_id, origin_id)).unwrap());
	});
}
