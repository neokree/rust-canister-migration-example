use ic_cdk::{export::candid, storage};
use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    // Initialize the RefCell with a value of 0
    static COUNTER: RefCell<candid::Nat> = RefCell::new(candid::Nat::from(0));
}

#[update]
fn increment() -> candid::Nat {
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1u64;
    });
    get()
}

#[query]
fn get() -> candid::Nat {
    COUNTER.with(|counter| counter.borrow().clone())
}

#[update]
fn set(input: candid::Nat) -> candid::Nat {
    COUNTER.with(|counter| counter.borrow_mut().0 = input.0);
    get()
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("pre_upgrade IN");
    COUNTER.with(|c| {
        let counter = c.borrow();
        _ = storage::stable_save((counter.clone(),));
    });
    ic_cdk::println!("pre_upgrade OUT");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("post_upgrade IN");
    let (counter,): (candid::Nat,) = storage::stable_restore().unwrap_or((candid::Nat::from(0),));
    COUNTER.with(|counter_cell| {
        *counter_cell.borrow_mut() = counter;
    });
    ic_cdk::println!("post_upgrade OUT");
}
