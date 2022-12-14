use ic_cdk::{export::candid, storage};
use ic_cdk_macros::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref COUNTER: Mutex<candid::Nat> = Mutex::new(candid::Nat::from(0));
}

#[update]
fn increment() {
    let mut counter = COUNTER.lock().unwrap();
    *counter += 1u64;
}

#[query]
fn get() -> candid::Nat {
    COUNTER.lock().unwrap().clone()
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("pre_upgrade IN");
    let counter = COUNTER.lock().unwrap();
    _ = storage::stable_save((counter.clone(),));
    ic_cdk::println!("pre_upgrade OUT");
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("post_upgrade IN");
    let (counter,): (candid::Nat,) = storage::stable_restore().unwrap_or((candid::Nat::from(0),));
    *COUNTER.lock().unwrap() = counter;
    ic_cdk::println!("post_upgrade OUT");
}
