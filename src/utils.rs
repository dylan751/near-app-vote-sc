use crate::*;

pub(crate) fn refund_deposit(storage_used: u64) -> u128 {
    // Tính lượng tiền cần nạp để cover storage
    let required_cost = env::storage_byte_cost() * Balance::from(storage_used);
    let attached_deposit = env::attached_deposit();

    log!("Storage data: {}", storage_used);
    log!("Storage fee: {}", required_cost);

    // Nếu người dùng deposit lượng tiền ít hơn lượng cần thiết để lưu data -> Báo lỗi
    assert!(
        attached_deposit >= required_cost,
        "Must attach {} yoctoNear to cover storage",
        required_cost
    );

    let refund_amount = attached_deposit - required_cost;

    // Thực hiện refund
    if refund_amount > 1 {
        Promise::new(env::predecessor_account_id()).transfer(refund_amount);
    }

    required_cost
}

// pub(crate) fn assert_one_yocto() {
//     assert_eq!(
//         env::attached_deposit(),
//         1,
//         "Required attached deposit of exact 1 yoctoNear"
//     );
// }

// pub(crate) fn assert_at_least_one_yocto() {
//     assert!(
//         env::attached_deposit() >= 1,
//         "Required attached deposit of at least 1 yoctoNear"
//     );
// }
