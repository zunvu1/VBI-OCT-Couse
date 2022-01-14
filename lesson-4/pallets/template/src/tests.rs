use super::*;
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_std::str;

pub fn str2vec(s: &str) -> Vec<u8> {
	s.as_bytes().to_vec()
}

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let chassis_num: Option<u32> = Some(1234567);
		let brand_name = str2vec("toyota");
		let price: Option<u32> = Some(1000000);

		// Dispatch a signed extrinsic.
		assert_ok!(CarProof::create_claim(
			Origin::signed(1),
			chassis_num.clone(),
			brand_name,
			price.clone()
		));

		let car = <CarStorage<Test>>::get(1).unwrap();
		// Read pallet storage and assert an expected result.
		assert_eq!(car.chassis_num, chassis_num);
	});
}

#[test]
fn create_claim_fails_already_exist() {
	new_test_ext().execute_with(|| {
		let chassis_num: Option<u32> = Some(1234567);
		let brand_name = str2vec("toyota");
		let price: Option<u32> = Some(1000000);

		let _ = CarProof::create_claim(
			Origin::signed(1),
			chassis_num.clone(),
			brand_name.clone(),
			price.clone(),
		);

		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			CarProof::create_claim(
				Origin::signed(1),
				chassis_num.clone(),
				brand_name,
				price.clone(),
			),
			<Error<Test>>::CarProofAlreadyExists
		);
	});
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let chassis_num: Option<u32> = Some(1234567);
		let brand_name = str2vec("toyota");
		let price: Option<u32> = Some(1000000);

		let _ = CarProof::create_claim(
			Origin::signed(1),
			chassis_num.clone(),
			brand_name,
			price.clone(),
		);

		let _ = CarProof::revoke_claim(Origin::signed(1));

		// Ensure the expected error is thrown when no value is present.
		assert_eq!(<CarStorage<Test>>::get(1), None);
	});
}

#[test]
fn revoke_claim_fails_not_exist() {
	new_test_ext().execute_with(|| {
		let _ = CarProof::revoke_claim(Origin::signed(1));

		// Ensure the expected error is thrown when no value is present.
		assert_noop!(CarProof::revoke_claim(Origin::signed(1)), <Error<Test>>::NoSuchCarOwner);
	});
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let chassis_num: Option<u32> = Some(1234567);
		let brand_name = str2vec("toyota");
		let price: Option<u32> = Some(1000000);

		let _ = CarProof::create_claim(
			Origin::signed(1),
			chassis_num.clone(),
			brand_name,
			price.clone(),
		);

		let _ = CarProof::transfer_claim(Origin::signed(1), 2u64);

		// Ensure the expected error is thrown when no value is present.
		assert_eq!(<CarStorage<Test>>::get(1), None);

		let car = <CarStorage<Test>>::get(2).unwrap();

		assert_eq!(car.chassis_num, chassis_num);
	});
}

#[test]
fn transfer_claim_fails_not_exist() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			CarProof::transfer_claim(Origin::signed(1), 2u64),
			<Error<Test>>::NoSuchCarOwner
		);
	});
}

#[test]
fn transfer_claim_fails_no_owner() {
	new_test_ext().execute_with(|| {
		let chassis_num: Option<u32> = Some(1234567);
		let brand_name = str2vec("toyota");
		let price: Option<u32> = Some(1000000);

		let _ = CarProof::create_claim(
			Origin::signed(1),
			chassis_num.clone(),
			brand_name,
			price.clone(),
		);
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			CarProof::transfer_claim(Origin::signed(2), 1u64),
			<Error<Test>>::NoSuchCarOwner
		);
	});
}
