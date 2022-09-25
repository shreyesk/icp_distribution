use candid::types::ic_types::principal;
use ic_cdk::{
    api::call::ManualReply,
    export::{candid, Principal},
};
use ic_cdk_macros::*;
use std::cell::{Cell, RefCell};
use ic_cdk::api::management_canister::main::*;

mod creater;

thread_local! {
    static COUNTER: RefCell<candid::Nat> = RefCell::new(candid::Nat::from(0));
    static BALANCE: RefCell<candid::Nat> = RefCell::new(candid::Nat::from(0));
    static OWNER: Cell<Principal> = Cell::new(Principal::from_slice(&[]));
    static CREATED_PRINCIPALS: RefCell<Vec<Principal>> = RefCell::new(Vec::new());
}

#[init]
fn init() {
    OWNER.with(|owner| owner.set(ic_cdk::api::caller()));
}

#[update]
fn inc() {
    ic_cdk::println!("{:?}", OWNER.with(|owner| owner.get()));
    COUNTER.with(|counter| *counter.borrow_mut() += 1u64);
}

#[query(manual_reply = true)]
fn read() -> ManualReply<candid::Nat> {
    COUNTER.with(|counter| {
        ManualReply::one(counter)
    })
}

#[update]
fn write(input: candid::Nat) {
    COUNTER.with(|counter| {
        let mut counter_mut = counter.borrow_mut();
        *counter_mut = input + 1;
    });
}

#[query(manual_reply = true)]
fn get_balance() -> ManualReply<candid::Nat> {
    update_balance();
    BALANCE.with(|balance| {
        ManualReply::one(balance)
    })
}

#[update]
fn update_balance() {
    BALANCE.with(|balance| {
        let mut balance_mut = balance.borrow_mut();
        *balance_mut = candid::Nat::from(ic_cdk::api::canister_balance128());
    });
}

#[update(manual_reply = true)]
fn get_principal_from_text(pt: String) -> ManualReply<Principal> {
    let cp = candid::Principal::from_text(pt).unwrap();
    let arg = CanisterIdRecord { canister_id: cp};
    ManualReply::one(arg.canister_id)
}

#[update(manual_reply = true)]
async fn get_created_balance(canister_id_text: String) -> ManualReply<candid::Nat> {
    let canister_principal = candid::Principal::from_text(canister_id_text).unwrap();
    let arg = CanisterIdRecord { canister_id: canister_principal };
    let call_result = canister_status(arg);
    let awaited_result = call_result.await;
    let unwrapped_result: (CanisterStatusResponse,) = awaited_result.unwrap();
    let result = unwrapped_result.0;
    let cycles = result.cycles;
    ManualReply::one(cycles)
    // deposit_cycles(arg, 100_000_000_000u128).await.unwrap();

    // canister_status(arg)
}

#[update]
async fn add_cycles(cycles_per_canister: u128) {
    let mut captured_principals: Vec<Principal> = Vec::new();
    CREATED_PRINCIPALS.with(|principals| {
        let cp = principals.borrow();
        for p in &(*cp) {
            captured_principals.push(*p);
        }
        // let cp = principals.borrow();
        // for principal in &*cp {
        //     let arg: CanisterIdRecord = CanisterIdRecord { canister_id: *principal };
        //     deposit_cycles(arg, cycles_per_canister).await.unwrap();
        // }
    });
    for principal in captured_principals {
        let arg: CanisterIdRecord = CanisterIdRecord { canister_id: principal };
        deposit_cycles(arg, cycles_per_canister).await.unwrap();
    }
}

#[query(manual_reply = true)]
fn get_created_principals() -> ManualReply<Vec<candid::Principal>> {
    CREATED_PRINCIPALS.with(|principals| {
        ManualReply::one(principals)
    })
}

#[update(manual_reply = true)]
async fn create_new_canister() -> ManualReply<candid::Principal> {
    use creater::creater;
    let created_principal = creater::create_new_canister_and_get_principal().await;
    CREATED_PRINCIPALS.with(|principals| {
        let mut cpm = principals.borrow_mut();
        (*cpm).push(created_principal);
    });
    ManualReply::one(created_principal)

    // let canister_id = creater::create_a_canister().await.unwrap();
    // canister_id.to_string()
}

// #[pre_upgrade]
// fn pre_upgrade() {
//     let state: &State = ic_cdk::storage::get();
//     ic_cdk::storage::stable_save((state,)).unwrap();
// }

// #[post_upgrade]
// fn post_upgrade() {
//     let (state,): (State,) = ic_cdk::storage::stable_restore().unwrap();
//     *ic_cdk::storage::get_mut() = state;
// }

// #[query]
// fn get_caller() -> String {
//     let caller: Principal = ic_cdk::caller();
//     caller.to_string()
// }

// #[query]
// fn get_count() -> u32 {
//     let state: &State = ic_cdk::storage::get();
//     state.count
// }

// #[update]
// fn increment_count() -> u32 {
//     let state: &mut State = ic_cdk::storage::get_mut();
//     state.count += 1;
//     state.count
// }

// #[update]
// fn reset_count() {
//     let state: &mut State = ic_cdk::storage::get_mut();
//     std::mem::take(state);
// }

// #[query]
// fn get_balance() -> u128 {
//     ic_cdk::api::stable::
//     let state: &State = ic_cdk::storage::get();
//     state.balance
// }

// #[heartbeat]
// fn heartbeat() {
//     let state: &mut State = ic_cdk::storage::get_mut();
//     state.count += 1;
//     state.balance = ic_cdk::api::canister_balance128()
// }

// #[derive(CandidType, Deserialize, Default)]
// struct State {
//     count: u32,
//     balance: u128
// }