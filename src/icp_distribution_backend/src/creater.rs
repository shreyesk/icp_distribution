use ic_cdk_macros::*;

pub mod creater {
    use super::*;
    use ic_cdk::api::management_canister::{main::*, provisional::{ProvisionalCreateCanisterWithCyclesArgument, provisional_create_canister_with_cycles, ProvisionalTopUpCanisterArgument, provisional_top_up_canister}};

    // #[import(canister = "hello_icp")]
    // struct MainCanister;

    #[update]
    pub async fn create_new_canister_and_get_principal() -> candid::Principal {
        let settings = CanisterSettings {
            controllers: Some(vec![ic_cdk::caller(), ic_cdk::id()]),
            compute_allocation: Some(5.into()),
            memory_allocation: Some(1.into()),
            freezing_threshold: Some(1.into()),
        };
        let arg = ProvisionalCreateCanisterWithCyclesArgument {
            amount: Some(1_000_000_000.into()),
            settings: Some(settings),
        };
        let canister_id = provisional_create_canister_with_cycles(arg)
            .await
            .unwrap()
            .0
            .canister_id;

        let arg = ProvisionalTopUpCanisterArgument {
            canister_id,
            amount: 1_000_000_000.into(),
        };
        provisional_top_up_canister(arg).await.unwrap();

        // let arg = InstallCodeArgument {
        //     mode: CanisterInstallMode::Install,
        //     canister_id,
        //     // A minimal valid wasm module
        //     // wat2wasm "(module)"
        //     wasm_module: [0x00, b'a', b's', b'm', 0x01, 0x00, 0x00, 0x00].to_vec(),
        //     arg: vec![],
        // };
        // install_code(arg).await.unwrap();

        canister_id

        // let arg = CreateCanisterArgument {
        //     settings: Some(CanisterSettings {
        //         controllers: Some(vec![ic_cdk::caller(), ic_cdk::id()]),
        //         compute_allocation: Some(50.into()),
        //         memory_allocation: Some(10000.into()),
        //         freezing_threshold: Some(10000.into()),
        //     }),
        // };
        // let canister_id = create_canister(arg).await.unwrap().0.canister_id;
        // let arg = UpdateSettingsArgument {
        //     canister_id,
        //     settings: CanisterSettings::default(),
        // };
        // update_settings(arg).await.unwrap();

        // let arg = InstallCodeArgument {
        //     mode: CanisterInstallMode::Install,
        //     canister_id,
        //     // A minimal valid wasm module
        //     // wat2wasm "(module)"
        //     wasm_module: [0x00, b'a', b's', b'm', 0x01, 0x00, 0x00, 0x00].to_vec(),
        //     arg: vec![],
        // };
        // install_code(arg).await.unwrap();

        // canister_id

        // let arg = CanisterIdRecord { canister_id };
        // deposit_cycles(arg, 100_000_000_000u128).await.unwrap();

        // canister_status(arg)

                // uninstall_code(arg).await.unwrap();
        // start_canister(arg).await.unwrap();
        // stop_canister(arg).await.unwrap();
        // let response = canister_status(arg).await.unwrap().0;
        // assert_eq!(response.status, CanisterStatusType::Stopped);
        // delete_canister(arg).await.unwrap();
        // let response = raw_rand().await.unwrap().0;
        // assert_eq!(response.len(), 32);
    }
}

/*
use ic_agent::{Agent, ic_types::Principal};
use candid::{Encode, Decode, CandidType, Nat};
use serde::Deserialize;

#[derive(CandidType)]
struct Argument {
  amount: Option<Nat>,
}

#[derive(Deserialize)]
struct CreateCanisterResult {
  canister_id: Principal,
}

pub(crate) async fn create_a_canister() -> Result<Principal, Box<dyn std::error::Error>> {
  let agent = Agent::builder()
    .with_identity(ic_agent::identity::AnonymousIdentity)
    .build()?;
  // Only do the following call when not contacting the IC main net (e.g. a local emulator).
  // This is important as the main net public key is static and a rogue network could return
  // a different key.
  // If you know the root key ahead of time, you can use `agent.set_root_key(root_key)?;`.
  agent.fetch_root_key().await?;
  let management_canister_id = Principal::from_text("aaaaa-aa")?;

  let waiter = garcon::Delay::builder()
    .throttle(std::time::Duration::from_millis(500))
    .timeout(std::time::Duration::from_secs(60 * 5))
    .build();

  // Create a call to the management canister to create a new canister ID,
  // and wait for a result.
  let response = agent.update(&management_canister_id, "provisional_create_canister_with_cycles")
    .with_arg(&Encode!(&Argument { amount: None})?)
    .call_and_wait(waiter)
    .await?;

  //   let result = Decode!(response.as_slice(), CreateCanisterResult)?;
  //   let canister_id: Principal = result.canister_id;
  let canister_id = Principal::from_slice(response.as_slice());
  Ok(canister_id)
}

*/