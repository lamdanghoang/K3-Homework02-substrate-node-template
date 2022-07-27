use crate::{mock::*, Error, Gender};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_one_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(
			Origin::signed(1),
			"Dang Hoang Lam".to_string().as_bytes().to_vec(),
			26
		));
		// Read pallet storage and assert an expected result.
		assert_eq!(Demo::student_id(), 1);
	});
}

#[test]
fn create_two_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(Origin::signed(1), b"Dang Hoang Lam".to_vec(), 26));
		assert_ok!(Demo::create_student(
			Origin::signed(1),
			"Dang Hoang Ha".to_string().as_bytes().to_vec(),
			21
		));
		// Read pallet storage and assert an expected result.
		assert_eq!(Demo::student_id(), 2);
	});
}

#[test]
fn check_name_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(
			Origin::signed(1),
			"Dang Hoang Lam".to_string().as_bytes().to_vec(),
			26
		));

		let name = Demo::student(0).unwrap().name;
		// Read pallet storage and assert an expected result.
		assert_eq!(name, b"Dang Hoang Lam".to_vec());
	});
}

#[test]
fn check_age_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(Origin::signed(1), b"Dang Hoang Lam".to_vec(), 26));

		let age = Demo::student(0).unwrap().age;
		// Read pallet storage and assert an expected result.
		assert_eq!(age, 26);
	});
}

#[test]
fn check_gender_student() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(Demo::create_student(Origin::signed(1), b"Dang Hoang Lam".to_vec(), 26));

		let gender = Demo::student(0).unwrap().gender;
		// Read pallet storage and assert an expected result.
		assert_eq!(gender, Gender::Female);
	});
}

#[test]
fn create_student_too_young() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(
			Demo::create_student(Origin::signed(1), b"Su bin".to_vec(), 7),
			Error::<Test>::TooYoung
		);
	});
}
