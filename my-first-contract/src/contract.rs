use crate::msg::{CountResponse, QueryMsg};
use cosmwasm_std::HumanAddr;
use cosmwasm_std::Uint128;
use cosmwasm_std::{
    to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError,
    StdResult, Storage,
};

use hex::decode;
use sha2::{Digest, Sha256};

use crate::msg::{HandleMsg, InitMsg};
use crate::state::{balance_get, balance_set, config, config_read, State};

use std::time::{SystemTime, UNIX_EPOCH};

pub fn init<S: Storage, A: Api, Q: Querier>(
    _: &mut Extern<S, A, Q>,
    __: Env,
    ___: InitMsg,
) -> StdResult<InitResponse> {
    println!("inited");
    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    match msg {
        HandleMsg::Init {
            buyer,
            seller,
            expiration,
            value,
            secret_hash,
        } => try_init(deps, env, buyer, seller, expiration, value, secret_hash),
        HandleMsg::Claim { secret } => try_claim(deps, env, secret),
        HandleMsg::Refund {} => try_refund(deps, env),
    }
}

pub fn try_init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    buyer: HumanAddr,
    seller: HumanAddr,
    expiration: u64,
    value: Uint128,
    secret_hash: String,
) -> StdResult<HandleResponse> {
    println!("called");
    let state = State {
        buyer: deps.api.canonical_address(&buyer)?,
        seller: deps.api.canonical_address(&seller)?,
        expiration: expiration,
        value: value,
        secret_hash: secret_hash,
    };

    println!("buyer {}", state.buyer);
    println!("seller {}", state.seller);
    println!("expiration {}", state.expiration);
    println!("value {}", state.value);
    println!("secret_hash {}", state.secret_hash);

    balance_set(
        &mut deps.storage,
        &deps.api.canonical_address(&env.contract.address)?,
        &value,
    )?;

    println!("balance setter");

    let balance = balance_get(
        &deps.storage,
        &deps.api.canonical_address(&env.contract.address)?,
    );

    println!("balance {}", balance);

    config(&mut deps.storage).save(&state)?;

    println!("saved");

    Ok(HandleResponse::default())
}

pub fn try_claim<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    _: Env,
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
    _: Env,
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

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count<S: Storage, A: Api, Q: Querier>(_: &Extern<S, A, Q>) -> StdResult<CountResponse> {
    // let state = config_read(&deps.storage).load()?;
    Ok(CountResponse { count: 1 })
}
