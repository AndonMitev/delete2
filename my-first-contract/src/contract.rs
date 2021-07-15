use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};

use hex::decode;
use log::info;
use sha2::{Digest, Sha256};

use crate::msg::{HandleMsg, InitMsg};
use crate::state::{balance_get, balance_set, config, config_read, State};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    println!("called");
    let state = State {
        buyer: deps.api.canonical_address(&msg.buyer)?,
        seller: deps.api.canonical_address(&msg.seller)?,
        expiration: msg.expiration,
        value: msg.value,
        secret_hash: msg.secret_hash,
    };

    balance_set(
        &mut deps.storage,
        &deps.api.canonical_address(&env.contract.address)?,
        &msg.value,
    )?;

    let balance = balance_get(
        &deps.storage,
        &deps.api.canonical_address(&env.contract.address)?,
    );

    println!("balance {}", balance);
    println!("buyer {}", state.buyer);
    println!("seller {}", state.seller);
    println!("expiration {}", state.expiration);
    println!("value {}", state.value);
    println!("secret_hash {}", state.secret_hash);

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Claim { secret } => try_claim(deps, env, secret),
        HandleMsg::Refund {} => try_refund(deps, env),
    }
}

pub fn try_claim<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    secret: String,
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    let mut hasher = Sha256::default();
    let message: Vec<u8> = decode(secret).expect("Invalid Hex String");

    hasher.update(&message);

    let secret_hash: String = format!("{:x}", hasher.finalize());

    if state.secret_hash != secret_hash {
        return Err(StdError::generic_err("Invalid secret"));
    }

    // TODO: Transfer locked value to buyer
    // where this value gonna be locked !?
    // AND VALIDATIONS ON input secret

    Ok(HandleResponse::default())
}

pub fn try_refund<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
) -> StdResult<HandleResponse> {
    let state = config_read(&deps.storage).load()?;

    let current_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if current_timestamp < (state.expiration as u64) {
        return Err(StdError::generic_err("Swap is not expired"));
    }

    // TODO: Transfer locked value to buyer
    // where this value gonna be locked !?

    Ok(HandleResponse::default())
}
